use std::fs::File;
use std::io::{Cursor, Error};
use std::path::Path;

use byteorder::{ReadBytesExt, WriteBytesExt};
use num_enum::TryFromPrimitive;
use parking_lot::RwLockReadGuard;
use serde::Deserialize;

use packet_serialize::{DeserializePacket, SerializePacket, SerializePacketError};

use crate::game_server::{Broadcast, GameServer, ProcessPacketError};
use crate::game_server::client_update_packet::{Stat, StatId, Stats};
use crate::game_server::game_packet::{GamePacket, OpCode, Pos};
use crate::game_server::guid::{Guid, GuidTable};
use crate::game_server::player_update_packet::{AddNpc, BaseAttachmentGroup, Icon, RemoveGracefully, WeaponAnimation};
use crate::game_server::tunnel::TunneledPacket;

#[derive(Deserialize)]
pub struct MountConfig {
    id: u32,
    speed_multiplier: f32,
    jump_height_multiplier: f32,
    gravity_multiplier: f32,
    model_id: u32,
    texture: String,
    pub name_id: u32,
    pub icon_set_id: u32,
    mount_composite_effect: u32,
    dismount_composite_effect: u32
}

impl Guid<u32> for MountConfig {
    fn guid(&self) -> u32 {
        self.id
    }
}

pub fn load_mounts(config_dir: &Path) -> Result<GuidTable<u32, MountConfig>, Error> {
    let mut file = File::open(config_dir.join("mounts.json"))?;
    let mounts: Vec<MountConfig> = serde_json::from_reader(&mut file)?;

    let mount_table = GuidTable::new();
    {
        let mut write_handle = mount_table.write();
        for mount in mounts {
            let id = mount.guid();
            let previous = write_handle.insert(mount);

            if let Some(_) = previous {
                panic!("Two mounts have ID {}", id);
            }
        }
    }

    Ok(mount_table)
}

#[derive(Copy, Clone, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum MountOpCode {
    MountRequest             = 0x1,
    MountReply               = 0x2,
    DismountRequest          = 0x3,
    DismountReply            = 0x4,
    MountList                = 0x5,
    MountSpawn               = 0x6,
    MountSpawnByItemDef      = 0x8,
    MountListShowMarket      = 0x9,
    SetAutoMount             = 0xa
}

impl SerializePacket for MountOpCode {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<(), SerializePacketError> {
        OpCode::Mount.serialize(buffer)?;
        buffer.write_u8(*self as u8)?;
        Ok(())
    }
}

#[derive(SerializePacket, DeserializePacket)]
pub struct DismountReply {
    rider_guid: u64,
    composite_effect: u32
}

impl GamePacket for DismountReply {
    type Header = MountOpCode;
    const HEADER: Self::Header = MountOpCode::DismountReply;
}

#[derive(SerializePacket, DeserializePacket)]
pub struct MountReply {
    rider_guid: u64,
    mount_guid: u64,
    unknown1: u32,
    queue_pos: u32,
    unknown3: u32,
    composite_effect: u32,
    unknown5: u32
}

impl GamePacket for MountReply {
    type Header = MountOpCode;
    const HEADER: Self::Header = MountOpCode::MountReply;
}

#[derive(SerializePacket, DeserializePacket)]
pub struct MountSpawn {
    mount_id: u32
}

impl GamePacket for MountSpawn {
    type Header = MountOpCode;
    const HEADER: Self::Header = MountOpCode::MountSpawn;
}

fn process_dismount(sender: u32, game_server: &GameServer) -> Result<Vec<Broadcast>, ProcessPacketError> {
    let zones = game_server.read_zones();
    if let Some(zone_guid) = GameServer::zone_with_character(&zones, sender as u64) {
        if let Some(zone) = zones.get(zone_guid) {
            let zone_read_handle = zone.read();

            let characters = zone_read_handle.read_characters();
            if let Some(character) = characters.get(sender as u64) {
                let mut character_write_handle = character.write();
                if let Some(mount_id) = character_write_handle.mount_id {
                    character_write_handle.mount_id = None;

                    if let Some(mount) = game_server.mounts.read().get(mount_id) {
                        let mount_read_handle = mount.read();

                        Ok(vec![
                            Broadcast::Single(sender, vec![
                                GamePacket::serialize(
                                    &TunneledPacket {
                                        unknown1: true,
                                        inner: DismountReply {
                                            rider_guid: sender as u64,
                                            composite_effect: mount_read_handle.dismount_composite_effect,
                                        },
                                    }
                                )?,
                                GamePacket::serialize(
                                    &TunneledPacket {
                                        unknown1: true,
                                        inner: RemoveGracefully {
                                            guid: mount_guid(sender, mount_id),
                                            unknown1: false,
                                            unknown2: 0,
                                            unknown3: 0,
                                            unknown4: 0,
                                            unknown5: 0,
                                        },
                                    }
                                )?,
                                GamePacket::serialize(
                                    &TunneledPacket {
                                        unknown1: true,
                                        inner: Stats {
                                            stats: vec![
                                                Stat {
                                                    id: StatId::Speed,
                                                    multiplier: 1,
                                                    value1: 0.0,
                                                    value2: zone_read_handle.speed,
                                                },
                                                Stat {
                                                    id: StatId::JumpHeightMultiplier,
                                                    multiplier: 1,
                                                    value1: 0.0,
                                                    value2: zone_read_handle.jump_height_multiplier,
                                                },
                                                Stat {
                                                    id: StatId::GravityMultiplier,
                                                    multiplier: 1,
                                                    value1: 0.0,
                                                    value2: zone_read_handle.gravity_multiplier,
                                                }
                                            ],
                                        },
                                    }
                                )?
                            ])
                        ])
                    } else {
                        println!("Player {} tried to dismount from non-existent mount", sender);
                        Ok(Vec::new())
                    }
                } else {

                    // Character is already dismounted
                    Ok(Vec::new())

                }
            } else {
                println!("Non-existent player {} tried to dismount", sender);
                Err(ProcessPacketError::CorruptedPacket)
            }
        } else {
            println!("Player {} tried to dismount in zone that went missing", sender);
            Ok(Vec::new())
        }
    } else {
        Err(ProcessPacketError::CorruptedPacket)
    }
}

fn process_mount_spawn(cursor: &mut Cursor<&[u8]>, sender: u32,
                       game_server: &GameServer) -> Result<Vec<Broadcast>, ProcessPacketError> {
    let mount_spawn = MountSpawn::deserialize(cursor)?;
    let mount_guid = mount_guid(sender, mount_spawn.mount_id);

    if let Some(mount) = game_server.mounts.read().get(mount_spawn.mount_id) {
        let mount_read_handle = mount.read();
        let mut packets = spawn_mount_npc(mount_guid, &mount_read_handle)?;
        packets.push(
            GamePacket::serialize(
                &TunneledPacket {
                    unknown1: true,
                    inner: MountReply {
                        rider_guid: sender as u64,
                        mount_guid,
                        unknown1: 0,
                        queue_pos: 1,
                        unknown3: 1,
                        composite_effect: mount_read_handle.mount_composite_effect,
                        unknown5: 0,
                    },
                }
            )?
        );

        let zones = game_server.read_zones();
        if let Some(zone_guid) = GameServer::zone_with_character(&zones, sender as u64) {
            if let Some(zone) = zones.get(zone_guid) {
                let zone_read_handle = zone.read();
                packets.push(
                    GamePacket::serialize(
                        &TunneledPacket {
                            unknown1: true,
                            inner: Stats {
                                stats: vec![
                                    Stat {
                                        id: StatId::Speed,
                                        multiplier: 1,
                                        value1: 0.0,
                                        value2: zone_read_handle.speed * mount_read_handle.speed_multiplier,
                                    },
                                    Stat {
                                        id: StatId::JumpHeightMultiplier,
                                        multiplier: 1,
                                        value1: 0.0,
                                        value2: zone_read_handle.jump_height_multiplier * mount_read_handle.jump_height_multiplier,
                                    },
                                    Stat {
                                        id: StatId::GravityMultiplier,
                                        multiplier: 1,
                                        value1: 0.0,
                                        value2: zone_read_handle.gravity_multiplier * mount_read_handle.gravity_multiplier,
                                    }
                                ],
                            },
                        }
                    )?
                );

                let characters = zone_read_handle.read_characters();
                if let Some(character) = characters.get(sender as u64) {
                    let mut character_write_handle = character.write();
                    if let Some(mount_id) = character_write_handle.mount_id {
                        println!("Player {} tried to mount while already mounted on mount ID {}", sender, mount_id);
                        return Err(ProcessPacketError::CorruptedPacket);
                    }
                    
                    character_write_handle.mount_id = Some(mount_read_handle.guid());
                } else {
                    println!("Non-existent player {} tried to mount", sender);
                    return Err(ProcessPacketError::CorruptedPacket);
                }
            }
        } else {
            println!("Player {} tried to mount in non-existent zone", sender);
            return Err(ProcessPacketError::CorruptedPacket);
        }

        Ok(vec![Broadcast::Single(sender, packets)])
    } else {
        Err(ProcessPacketError::CorruptedPacket)
    }
}

pub fn process_mount_packet(cursor: &mut Cursor<&[u8]>, sender: u32,
                            game_server: &GameServer) -> Result<Vec<Broadcast>, ProcessPacketError> {
    let raw_op_code = cursor.read_u8()?;
    match MountOpCode::try_from(raw_op_code) {
        Ok(op_code) => match op_code {
            MountOpCode::DismountRequest => process_dismount(sender, game_server),
            MountOpCode::MountSpawn => process_mount_spawn(cursor, sender, game_server),
            _ => {
                println!("Unimplemented mount op code: {:?}", op_code);
                Ok(Vec::new())
            }
        },
        Err(_) => {
            println!("Unknown mount op code: {}", raw_op_code);
            Err(ProcessPacketError::CorruptedPacket)
        }
    }
}

pub fn mount_guid(character_guid: u32, mount_id: u32) -> u64 {
    (mount_id as u64) << 32 | (character_guid as u64)
}

fn spawn_mount_npc(guid: u64, mount: &RwLockReadGuard<MountConfig>) -> Result<Vec<Vec<u8>>, ProcessPacketError> {
    Ok(
        vec![
            GamePacket::serialize(&TunneledPacket {
                unknown1: true,
                inner: AddNpc {
                    guid,
                    name_id: mount.name_id,
                    model_id: mount.model_id,
                    unknown3: false,
                    unknown4: 0,
                    unknown5: 0,
                    unknown6: 1,
                    scale: 1.2,
                    pos: Pos {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                        w: 1.0,
                    },
                    rot: Pos {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                        w: 0.0,
                    },
                    unknown8: 0,
                    attachments: vec![],
                    is_not_targetable: 1,
                    unknown10: 0,
                    texture_name: mount.texture.clone(),
                    tint_name: "".to_string(),
                    tint_id: 0,
                    unknown11: true,
                    offset_y: 0.0,
                    composite_effect: 0,
                    weapon_animation: WeaponAnimation::None,
                    name_override: "".to_string(),
                    hide_name: true,
                    name_offset_x: 0.0,
                    name_offset_y: 0.0,
                    name_offset_z: 0.0,
                    terrain_object_id: 0,
                    invisible: false,
                    unknown20: 0.0,
                    unknown21: false,
                    interactable_size_pct: 0,
                    unknown23: -1,
                    unknown24: -1,
                    active_animation_slot: 1,
                    unknown26: false,
                    ignore_position: false,
                    sub_title_id: 0,
                    active_animation_slot2: 1,
                    head_model_id: 0,
                    unknown31: vec![],
                    disable_interact_popup: true,
                    unknown33: 0,
                    unknown34: false,
                    show_health: false,
                    unknown36: false,
                    ignore_rotation_and_shadow: false,
                    base_attachment_group: BaseAttachmentGroup {
                        unknown1: 0,
                        unknown2: "".to_string(),
                        unknown3: "".to_string(),
                        unknown4: 0,
                        unknown5: "".to_string(),
                    },
                    unknown39: Pos {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                        w: 0.0,
                    },
                    unknown40: 0,
                    unknown41: -1,
                    unknown42: 0,
                    collision: true,
                    unknown44: 0,
                    npc_type: 2,
                    unknown46: 0.0,
                    target: 0,
                    unknown50: vec![],
                    rail_id: 0,
                    rail_speed: 0.0,
                    rail_origin: Pos {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                        w: 0.0,
                    },
                    unknown54: 0,
                    rail_unknown1: 0.0,
                    rail_unknown2: 0.0,
                    rail_unknown3: 0.0,
                    attachment_group_unknown: "".to_string(),
                    unknown59: "".to_string(),
                    unknown60: "".to_string(),
                    override_terrain_model: false,
                    hover_glow: 0,
                    hover_description: 0,
                    fly_over_effect: 0,
                    unknown65: 0,
                    unknown66: 0,
                    unknown67: 0,
                    disable_move_to_interact: false,
                    unknown69: 0.0,
                    unknown70: 0.0,
                    unknown71: 0,
                    icon_id: Icon::None,
                },
            })?
        ]
    )
}

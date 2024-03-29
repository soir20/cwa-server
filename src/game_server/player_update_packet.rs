use byteorder::{LittleEndian, WriteBytesExt};
use packet_serialize::{DeserializePacket, SerializePacket, SerializePacketError};
use crate::game_server::game_packet::{Effect, GamePacket, OpCode, Pos, StringId};

#[derive(Copy, Clone, Debug)]
pub enum PlayerUpdateOpCode {
    AddNpc                   = 0x2,
    UpdateCharacterState     = 0x14,
    SetCollision             = 0x32
}

impl SerializePacket for PlayerUpdateOpCode {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<(), SerializePacketError> {
        OpCode::PlayerUpdate.serialize(buffer)?;
        buffer.write_u16::<LittleEndian>(*self as u16)?;
        Ok(())
    }
}

#[derive(SerializePacket, DeserializePacket)]
pub struct UpdateCharacterState {
    pub guid: u64,
    pub bitflags: u32,
}

impl GamePacket for UpdateCharacterState {
    type Header = PlayerUpdateOpCode;
    const HEADER: Self::Header = PlayerUpdateOpCode::UpdateCharacterState;
}

#[derive(SerializePacket, DeserializePacket)]
pub struct SetCollision {
    pub guid: u64,
    pub collide: bool
}

impl GamePacket for SetCollision {
    type Header = PlayerUpdateOpCode;
    const HEADER: Self::Header = PlayerUpdateOpCode::SetCollision;
}

#[derive(SerializePacket, DeserializePacket)]
pub struct Attachment {
    pub unknown1: String,
    pub unknown2: String,
    pub unknown3: String,
    pub unknown4: u32,
    pub unknown5: u32,
    pub unknown6: u32,
}

#[derive(SerializePacket, DeserializePacket)]
pub struct Unknown {
    pub unknown1: u32,
    pub unknown2: String,
    pub unknown3: String,
    pub unknown4: u32,
    pub unknown5: String,
}

#[derive(SerializePacket, DeserializePacket)]
pub struct Variable {
    pub unknown1: u32,
    pub unknown2: String,
    pub unknown3: u32,
}

#[derive(Copy, Clone, Debug)]
pub enum Icon {
    None = 0,
    Member = 1,
    Enforcer = 2,
    FancyMember = 3,
}

impl SerializePacket for Icon {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<(), SerializePacketError> {
        buffer.write_u32::<LittleEndian>(*self as u32)?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum DamageAnimation {
    None = 0,
    Explode = 1,
    SaberStrike = 2,
}

impl SerializePacket for DamageAnimation {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<(), SerializePacketError> {
        buffer.write_u32::<LittleEndian>(*self as u32)?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum WeaponAnimation {
    None = 0,
    SingleSaber = 1,
    StaffSaber = 2,
    ReverseSingleSaber = 3,
    DoubleSaber = 4,
    SinglePistol = 5,
    Rifle = 6,
    SniperRifle = 7,
    RocketLauncher = 8,
    Flamethrower = 9,
    DoublePistol = 10,
    Staff = 11,
}

impl SerializePacket for WeaponAnimation {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<(), SerializePacketError> {
        buffer.write_u32::<LittleEndian>(*self as u32)?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MoveAnimation {
    Standing = 1,
    Walking = 2,
    Running = 3,
}

impl SerializePacket for MoveAnimation {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<(), SerializePacketError> {
        buffer.write_u32::<LittleEndian>(*self as u32)?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum HoverGlow {
    Disabled = 0,
    Enabled = 1
}

impl SerializePacket for HoverGlow {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<(), SerializePacketError> {
        buffer.write_u32::<LittleEndian>(*self as u32)?;
        Ok(())
    }
}

#[derive(SerializePacket)]
pub struct AddNpc {
    pub guid: u64,
    pub name_id: u32,
    pub model_id: u32,
    pub unknown3: bool,
    pub unknown4: u32,
    pub unknown5: u32,
    pub unknown6: u32,
    pub scale: f32,
    pub position: Pos,
    pub rotation: Pos,
    pub unknown8: u32,
    pub attachments: Vec<Attachment>,
    pub is_terrain_object_noninteractable: u32,
    pub unknown10: u32,
    pub texture_name: String,
    pub tint_name: String,
    pub tint_id: u32,
    pub unknown11: bool,
    pub offset_y: f32,
    pub damage_animation: DamageAnimation,
    pub weapon_animation: WeaponAnimation,
    pub name_override: String,
    pub hide_name: bool,
    pub name_offset_x: f32,
    pub name_offset_y: f32,
    pub name_offset_z: f32,
    pub terrain_object_id: u32,
    pub invisible: bool,
    pub unknown20: f32,
    pub unknown21: bool,
    pub interactable_size_pct: u32,
    pub unknown23: i32,
    pub unknown24: i32,
    pub move_animation: MoveAnimation,
    pub unknown26: bool,
    pub unknown27: bool,
    pub sub_title_id: StringId,
    pub move_animation2: MoveAnimation,
    pub head_model_id: u32,
    pub unknown31: Vec<Effect>,
    pub unknown32: bool,
    pub unknown33: u32,
    pub unknown34: bool,
    pub show_health: bool,
    pub unknown36: bool,
    pub enable_move_to_interact: bool,
    pub unknown38: Unknown,
    pub unknown39: Pos,
    pub unknown40: u32,
    pub unknown41: i32,
    pub unknown42: u32,
    pub collision: bool,
    pub unknown44: u64,
    pub unknown45: u32,
    pub unknown46: f32,
    pub target: u32,
    pub unknown50: Vec<Variable>,
    pub trick_animation_id: u32,
    pub unknown52: f32,
    pub unknown53: Pos,
    pub unknown54: u32,
    pub unknown55: f32,
    pub unknown56: f32,
    pub unknown57: f32,
    pub unknown58: String,
    pub unknown59: String,
    pub unknown60: String,
    pub is_not_terrain_object: bool,
    pub hover_glow: HoverGlow,
    pub unknown63: u32,
    pub fly_over_effect: u32,
    pub unknown65: u32,
    pub unknown66: u32,
    pub unknown67: u32,
    pub disable_move_to_interact: bool,
    pub unknown69: f32,
    pub unknown70: f32,
    pub unknown71: u64,
    pub icon_id: Icon,
}

impl GamePacket for AddNpc {
    type Header = PlayerUpdateOpCode;
    const HEADER: PlayerUpdateOpCode = PlayerUpdateOpCode::AddNpc;
}

pub fn make_test_npc() -> AddNpc {
    AddNpc {
        guid: 2,
        name_id: 0,
        model_id: 0,
        unknown3: false,
        unknown4: 0,
        unknown5: 0,
        unknown6: 1,
        scale: 1.0,
        position: Pos {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            rot: 1.0,
        },
        rotation: Pos {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            rot: 0.0,
        },
        unknown8: 0,
        attachments: vec![],
        is_terrain_object_noninteractable: 0, // Terrain objects only seem interactable
                                              // when this == 0. Otherwise, click to move
                                              // targets a spot behind the object. Likely some
                                              // kind of index in the collision or mesh data.
        unknown10: 0,
        texture_name: "".to_string(),
        tint_name: "".to_string(),
        tint_id: 0,
        unknown11: true,
        offset_y: 0.0, // Only enabled when unknown45 == 2
        damage_animation: DamageAnimation::None,
        weapon_animation: WeaponAnimation::None,
        name_override: "".to_string(),
        hide_name: false,
        name_offset_x: 0.0,
        name_offset_y: 0.0,
        name_offset_z: 0.0,
        terrain_object_id: 1278971264,
        invisible: false,
        unknown20: 0.0,
        unknown21: false,
        interactable_size_pct: 100,
        unknown23: -1,
        unknown24: -1,
        move_animation: MoveAnimation::Standing,
        unknown26: false,
        unknown27: false,
        sub_title_id: 0,
        move_animation2: MoveAnimation::Standing,
        head_model_id: 0,
        unknown31: vec![],
        unknown32: false,
        unknown33: 0,
        unknown34: false,
        show_health: false,
        unknown36: false,
        enable_move_to_interact: false,
        unknown38: Unknown {
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
            rot: 0.0,
        },
        unknown40: 0,
        unknown41: -1,
        unknown42: 0,
        collision: true, // To be interactable, every NPC must have collision set,
                         // even if the model does not actually support collision
        unknown44: 0,
        unknown45: 2,
        unknown46: 0.0,
        target: 0,
        unknown50: vec![],
        trick_animation_id: 0,
        unknown52: 0.0,
        unknown53: Pos {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            rot: 0.0,
        },
        unknown54: 0,
        unknown55: 0.0,
        unknown56: 0.0,
        unknown57: 0.0,
        unknown58: "".to_string(),
        unknown59: "".to_string(),
        unknown60: "".to_string(),
        is_not_terrain_object: false, // Non-terrain NPCs must have this enabled to be interactable
        hover_glow: HoverGlow::Enabled,
        unknown63: 0, // max 7
        fly_over_effect: 0, // max 3
        unknown65: 0, // max 32
        unknown66: 0,
        unknown67: 0,
        disable_move_to_interact: false,
        unknown69: 0.0,
        unknown70: 0.0,
        unknown71: 0,
        icon_id: Icon::None,
    }
}

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
    unknown1: String,
    unknown2: String,
    unknown3: String,
    unknown4: u32,
    unknown5: u32,
    unknown6: u32,
}

#[derive(SerializePacket, DeserializePacket)]
pub struct Unknown {
    unknown1: u32,
    unknown2: String,
    unknown3: String,
    unknown4: u32,
    unknown5: String,
}

#[derive(SerializePacket, DeserializePacket)]
pub struct Variable {
    unknown1: u32,
    unknown2: String,
    unknown3: u32,
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

#[derive(SerializePacket)]
pub struct AddNpc {
    guid: u64,
    name_id: u32,
    model_id: u32,
    unknown3: bool,
    unknown4: u32,
    unknown5: u32,
    unknown6: u32,
    scale: f32,
    position: Pos,
    rotation: Pos,
    unknown8: u32,
    attachments: Vec<Attachment>,
    unknown9: u32,
    unknown10: u32,
    texture_name: String,
    tint_name: String,
    tint_id: u32,
    unknown11: bool,
    unknown12: u32,
    damage_animation: DamageAnimation,
    weapon_animation: WeaponAnimation,
    name_override: String,
    hide_name: bool,
    name_offset_x: f32,
    name_offset_y: f32,
    name_offset_z: f32,
    invisible2: u32,
    invisible: bool,
    unknown20: f32,
    unknown21: bool,
    interactable_size_pct: u32,
    unknown23: u32,
    unknown24: u32,
    move_animation: MoveAnimation,
    unknown26: bool,
    unknown27: bool,
    sub_title_id: StringId,
    move_animation2: MoveAnimation,
    head_model_id: u32,
    unknown31: Vec<Effect>,
    unknown32: bool,
    unknown33: u32,
    unknown34: bool,
    show_health: bool,
    unknown36: bool,
    unknown37: bool,
    unknown38: Unknown,
    unknown39: Pos,
    unknown40: u32,
    unknown41: u32,
    unknown42: u32,
    interactable: bool,
    unknown44: u64,
    unknown45: u32,
    unknown46: f32,
    target: u32,
    unknown50: Vec<Variable>,
    trick_animation_id: u32,
    unknown52: f32,
    unknown53: Pos,
    unknown54: u32,
    unknown55: f32,
    unknown56: f32,
    unknown57: f32,
    unknown58: String,
    unknown59: String,
    unknown60: String,
    interactable2: bool,
    unknown62: u32,
    unknown63: u32,
    unknown64: u32,
    unknown65: u32,
    unknown66: u32,
    unknown67: u32,
    unknown68: bool,
    unknown69: f32,
    unknown70: f32,
    unknown71: u64,
    icon_id: Icon,
}

impl GamePacket for AddNpc {
    type Header = PlayerUpdateOpCode;
    const HEADER: PlayerUpdateOpCode = PlayerUpdateOpCode::AddNpc;
}

pub fn make_test_npc() -> AddNpc {
    AddNpc {
        guid: 2,
        name_id: 33927,
        model_id: 2406,
        unknown3: false,
        unknown4: 0,
        unknown5: 0,
        unknown6: 1,
        scale: 1.0,
        position: Pos {
            x: 887.3,
            y: 171.95,
            z: 1546.956,
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
        unknown9: 1,
        unknown10: 0,
        texture_name: "".to_string(),
        tint_name: "".to_string(),
        tint_id: 0,
        unknown11: true,
        unknown12: 0,
        damage_animation: DamageAnimation::None,
        weapon_animation: WeaponAnimation::None,
        name_override: "".to_string(),
        hide_name: false,
        name_offset_x: 0.0,
        name_offset_y: 0.0,
        name_offset_z: 0.0,
        invisible2: 0,
        invisible: false,
        unknown20: 0.0,
        unknown21: false,
        interactable_size_pct: 100,
        unknown23: 0,
        unknown24: 0,
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
        unknown37: false,
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
        unknown41: 0,
        unknown42: 0,
        interactable: true,
        unknown44: 0,
        unknown45: 0,
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
        interactable2: true,
        unknown62: 0,
        unknown63: 0,
        unknown64: 0,
        unknown65: 0, // max 32
        unknown66: 0,
        unknown67: 0,
        unknown68: false,
        unknown69: 0.0,
        unknown70: 0.0,
        unknown71: 0,
        icon_id: Icon::None,
    }
}

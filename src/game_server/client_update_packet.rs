use std::io::Write;

use byteorder::{LittleEndian, WriteBytesExt};

use packet_serialize::{DeserializePacket, SerializePacket, SerializePacketError};

use crate::game_server::game_packet::{GamePacket, OpCode, Pos};
use crate::game_server::item::{EquipmentSlot, Item, ItemDefinition};

#[derive(Copy, Clone, Debug)]
pub enum ClientUpdateOpCode {
    Health                   = 0x1,
    AddItems                 = 0x2,
    EquipItem                = 0x5,
    Position                 = 0xc,
    Power                    = 0xd,
    Stats                    = 0x7,
    PreloadCharactersDone    = 0x1a
}

impl SerializePacket for ClientUpdateOpCode {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<(), SerializePacketError> {
        OpCode::ClientUpdate.serialize(buffer)?;
        buffer.write_u16::<LittleEndian>(*self as u16)?;
        Ok(())
    }
}

#[derive(SerializePacket, DeserializePacket)]
pub struct Position {
    pub player_pos: Pos,
    pub rot: Pos,
    pub is_teleport: bool,
    pub unknown2: bool
}

impl GamePacket for Position {
    type Header = ClientUpdateOpCode;
    const HEADER: Self::Header = ClientUpdateOpCode::Position;
}

#[derive(SerializePacket)]
pub struct AddItemsData {
    pub item: Item,
    pub definition: ItemDefinition
}

pub struct AddItems {
    pub data: AddItemsData
}

impl SerializePacket for AddItems {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<(), SerializePacketError> {
        let mut inner_buffer = Vec::new();
        self.data.serialize(&mut inner_buffer)?;
        buffer.write_u32::<LittleEndian>(inner_buffer.len() as u32)?;
        buffer.write_all(&inner_buffer)?;
        Ok(())
    }
}

impl GamePacket for AddItems {
    type Header = ClientUpdateOpCode;
    const HEADER: Self::Header = ClientUpdateOpCode::AddItems;
}

#[derive(SerializePacket)]
pub struct EquipItem {
    pub item_guid: u32,
    pub model_name: String,
    pub texture_alias: String,
    pub tint_alias: String,
    pub tint: u32,
    pub composite_effect: u32,
    pub slot: EquipmentSlot,
    pub profile_id: u32,
    pub item_def_class: u32,
    pub update_gear: bool
}

impl GamePacket for EquipItem {
    type Header = ClientUpdateOpCode;
    const HEADER: Self::Header = ClientUpdateOpCode::EquipItem;
}

#[derive(SerializePacket, DeserializePacket)]
pub struct Health {
    pub(crate) current: u32,
    pub(crate) max: u32,
}

impl GamePacket for Health {
    type Header = ClientUpdateOpCode;
    const HEADER: ClientUpdateOpCode = ClientUpdateOpCode::Health;
}

#[derive(SerializePacket, DeserializePacket)]
pub struct Power {
    pub(crate) current: u32,
    pub(crate) max: u32,
}

impl GamePacket for Power {
    type Header = ClientUpdateOpCode;
    const HEADER: ClientUpdateOpCode = ClientUpdateOpCode::Power;
}

#[derive(Copy, Clone, Debug)]
pub enum StatId {
    MaxHealth                = 1,
    Speed                    = 2,
    Range                    = 3,
    HealthRegen              = 4,
    MaxPower                 = 5,
    PowerRegen               = 6,
    MeleeDefense             = 7,
    MeleeDodge               = 8,
    MeleeCritRate            = 9,
    MeleeCritMultiplier      = 10,
    MeleeAccuracy            = 11,
    WeaponDamageMultiplier   = 12,
    HandToHandDamage         = 13,
    WeaponDamage             = 14,
    WeaponSpeed              = 15,
    DamageReductionFlat      = 16,
    ExperienceBoost          = 17,
    DamageReductionPct       = 18,
    DamageAddition           = 19,
    DamageMultiplier         = 20,
    HealingAddition          = 21,
    HealingMultiplier        = 22,
    AbilityCritRate          = 33,
    AbilityCritMultiplier    = 34,
    Luck                     = 35,
    HeadInflation            = 36,
    CurrencyBoost            = 37,
    Toughness                = 50,
    AbilityCritVulnerability = 51,
    MeleeCritVulnerability   = 52,
    RangeMultiplier          = 53,
    MaxShield                = 54,
    ShieldRegen              = 55,
    MimicMovementSpeed       = 57,
    GravityMultiplier        = 58,
    JumpHeightMultiplier     = 59
}

impl SerializePacket for StatId {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<(), SerializePacketError> {
        buffer.write_u32::<LittleEndian>(*self as u32)?;
        Ok(())
    }
}

#[derive(SerializePacket)]
pub struct Stat {
    pub(crate) id: StatId,
    pub(crate) multiplier: u32,
    pub(crate) value1: f32,
    pub(crate) value2: f32,
}

#[derive(SerializePacket)]
pub struct Stats {
    pub(crate) stats: Vec<Stat>
}

impl GamePacket for Stats {
    type Header = ClientUpdateOpCode;
    const HEADER: ClientUpdateOpCode = ClientUpdateOpCode::Stats;
}

#[derive(SerializePacket, DeserializePacket)]
pub struct PreloadCharactersDone {
    pub(crate) unknown1: bool
}

impl GamePacket for PreloadCharactersDone {
    type Header = ClientUpdateOpCode;
    const HEADER: ClientUpdateOpCode = ClientUpdateOpCode::PreloadCharactersDone;
}

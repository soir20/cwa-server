use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use packet_serialize::{DeserializePacket, SerializePacket, SerializePacketError};
use crate::game_server::game_packet::{GamePacket, OpCode};
use crate::game_server::{GameServer, ProcessPacketError};

pub fn process_command(game_server: &mut GameServer, cursor: &mut Cursor<&[u8]>) -> Result<Vec<Vec<u8>>, ProcessPacketError> {
    let raw_op_code = cursor.read_u16::<LittleEndian>()?;
    match CommandOpCode::try_from(raw_op_code) {
        Ok(op_code) => match op_code {
            CommandOpCode::InteractionRequest => {

                // TODO: determine zone from requester GUID
                if let Some(zone) = game_server.zones.get_mut(&2) {
                    let interaction_request = InteractionRequest::deserialize(cursor)?;
                    Ok(zone.process_npc_interaction(interaction_request)?)
                } else {
                    Err(ProcessPacketError::CorruptedPacket)
                }

            },
            _ => {
                println!("Unimplemented command: {:?}", op_code);
                Ok(Vec::new())
            }
        },
        Err(_) => {
            println!("Unknown command: {}", raw_op_code);
            Ok(Vec::new())
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CommandOpCode {
    InteractionList          = 0x9,
    InteractionRequest       = 0xf
}

impl SerializePacket for CommandOpCode {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<(), SerializePacketError> {
        OpCode::Command.serialize(buffer)?;
        buffer.write_u16::<LittleEndian>(*self as u16)?;
        Ok(())
    }
}

pub struct UnknownCommandOpCode;

impl TryFrom<u16> for CommandOpCode {
    type Error = UnknownCommandOpCode;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x9 => Ok(CommandOpCode::InteractionList),
            0xf => Ok(CommandOpCode::InteractionRequest),
            _ => Err(UnknownCommandOpCode)
        }
    }
}

#[derive(SerializePacket, DeserializePacket)]
pub struct Interaction {
    pub unknown1: u32,
    pub unknown2: u32,
    pub unknown3: u32,
    pub unknown4: u32,
    pub unknown5: u32,
    pub unknown6: u32,
    pub unknown7: u32,
    pub unknown8: u32,
    pub unknown9: u32,
}

#[derive(SerializePacket, DeserializePacket)]
pub struct InteractionList {
    pub guid: u64,
    pub unknown1: bool,
    pub interactions: Vec<Interaction>,
    pub unknown2: String,
    pub unknown3: bool,
    pub unknown4: bool,
}

impl GamePacket for InteractionList {
    type Header = CommandOpCode;
    const HEADER: Self::Header = CommandOpCode::InteractionList;
}

#[derive(SerializePacket, DeserializePacket)]
pub struct InteractionRequest {
    pub requester: u64,
    pub target: u64
}

impl GamePacket for InteractionRequest {
    type Header = CommandOpCode;
    const HEADER: Self::Header = CommandOpCode::InteractionRequest;
}

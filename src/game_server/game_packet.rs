use byteorder::{LittleEndian, WriteBytesExt};
use packet_serialize::{SerializePacket, SerializePacketError};

#[derive(Debug)]
pub enum OpCode {
    LoginRequest             = 0x1,
    LoginReply               = 0x2,
    TunneledClient           = 0x5,
    Player                   = 0xc,
    ClientIsReady            = 0xd,
    ZoneDetailsDone          = 0xe,
    ClientUpdate             = 0x26,
    ZoneDetails              = 0x2b,
    GameTimeSync             = 0x34,
    WelcomeScreen            = 0x5d,
    ClientGameSettings       = 0x8f,
    DeploymentEnv            = 0xa5,
}

pub struct UnknownOpCode;

impl TryFrom<u16> for OpCode {
    type Error = UnknownOpCode;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x1 => Ok(OpCode::LoginRequest),
            0x2 => Ok(OpCode::LoginReply) ,
            0x5 => Ok(OpCode::TunneledClient),
            0xc => Ok(OpCode::Player),
            0xd => Ok(OpCode::ClientIsReady),
            0xe => Ok(OpCode::ZoneDetailsDone),
            0x26 => Ok(OpCode::ClientUpdate),
            0x2b => Ok(OpCode::ZoneDetails),
            0x34 => Ok(OpCode::GameTimeSync),
            0x5d => Ok(OpCode::WelcomeScreen),
            0x8f => Ok(OpCode::ClientGameSettings),
            0xa5 => Ok(OpCode::DeploymentEnv),
            _ => Err(UnknownOpCode)
        }
    }
}

pub trait GamePacket: SerializePacket {
    const OP_CODE: OpCode;

    fn serialize_header(&self) -> Result<Vec<u8>, SerializePacketError> {
        let mut buffer = Vec::new();
        buffer.write_u16::<LittleEndian>(Self::OP_CODE as u16)?;
        Ok(buffer)
    }

    fn serialize(&self) -> Result<Vec<u8>, SerializePacketError> {
        let mut buffer = self.serialize_header()?;
        SerializePacket::serialize(self, &mut buffer)?;
        Ok(buffer)
    }
}

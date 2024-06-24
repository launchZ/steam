use crate::a2s::packet::PacketBytes;
use std::error::Error;

#[allow(non_camel_case_types)]
pub struct Packet_Info {
    bytes: PacketBytes,
    players: u8,
}

impl Packet_Info {
    pub fn bytes(&self) -> &PacketBytes {
        &self.bytes
    }
    pub fn players(&self) -> &u8 {
        &self.players
    }
}

impl TryFrom<PacketBytes> for Packet_Info {
    type Error = Box<dyn Error>;

    fn try_from(mut bytes: PacketBytes) -> Result<Self, Self::Error> {
        _ = bytes.read_u8()?;
        _ = bytes.read_u8()?;
        _ = bytes.read_until_zero()?;
        _ = bytes.read_until_zero()?;
        _ = bytes.read_until_zero()?;
        _ = bytes.read_until_zero()?;
        _ = bytes.read_u16()?;
        let players = bytes.read_u8()?;
        Ok(Self { bytes, players })
    }
}

use crate::a2s::packet::PacketBytes;
use std::error::Error;

#[allow(non_camel_case_types)]
pub struct Packet_Info {
    bytes: PacketBytes,
}

impl Packet_Info {
    pub fn bytes(&self) -> &PacketBytes {
        &self.bytes
    }
}

impl TryFrom<PacketBytes> for Packet_Info {
    type Error = Box<dyn Error>;

    fn try_from(bytes: PacketBytes) -> Result<Self, Self::Error> {
        Ok(Self { bytes })
    }
}

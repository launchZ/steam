use crate::a2s::packet::PacketBytes;
use std::error::Error;
use std::io::Read;

#[allow(non_camel_case_types)]
pub struct Packet_Challenge {
    bytes: PacketBytes,
    challenge: [u8; 4],
}

impl Packet_Challenge {
    pub fn bytes(&self) -> &PacketBytes {
        &self.bytes
    }
    pub fn challenge(&self) -> &[u8] {
        &self.challenge
    }
}

impl TryFrom<PacketBytes> for Packet_Challenge {
    type Error = Box<dyn Error>;

    fn try_from(mut bytes: PacketBytes) -> Result<Self, Self::Error> {
        let mut bytes_challenge = [0; 4];
        bytes.cursor.read_exact(&mut bytes_challenge)?;
        Ok(Self {
            bytes,
            challenge: bytes_challenge,
        })
    }
}

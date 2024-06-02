use crate::a2s::packet::PacketBytes;
use std::error::Error;

#[allow(non_camel_case_types)]
pub struct Packet_Players {
    bytes: PacketBytes,
    player_count: u8,
    players: Vec<Player>,
}

pub struct Player {
    pub index: u8,
    pub name: String,
    pub score: i32,
    pub duration: f32,
}

impl Packet_Players {
    pub fn bytes(&self) -> &PacketBytes {
        &self.bytes
    }
}

impl TryFrom<PacketBytes> for Packet_Players {
    type Error = Box<dyn Error>;

    fn try_from(mut bytes: PacketBytes) -> Result<Self, Self::Error> {
        let player_count = bytes.read_u8()?;
        let mut players = vec![];

        for _ in 0..player_count {
            players.push(Player {
                index: bytes.read_u8()?,
                name: bytes.read_str()?,
                score: bytes.read_i32()?,
                duration: bytes.read_f32()?,
            });
        }

        Ok(Self {
            bytes,
            player_count,
            players,
        })
    }
}

pub mod challenge;
pub mod info;
pub mod players;
pub mod rules;

use crate::a2s::packet::challenge::Packet_Challenge;
use crate::a2s::packet::info::Packet_Info;
use crate::a2s::packet::players::Packet_Players;
use crate::a2s::packet::rules::Packet_Rules;
use pretty_hex::pretty_hex;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::io::Cursor;
use std::io::Read;
use std::io::{BufRead, Seek, SeekFrom};
use std::net::ToSocketAddrs;
use std::net::UdpSocket;

pub enum Packet {
    Unknown(PacketBytes),
    Info(Packet_Info),
    Rules(Packet_Rules),
    Players(Packet_Players),
    Challenge(Packet_Challenge),
}

impl TryFrom<PacketBytes> for Packet {
    type Error = Box<dyn Error>;

    fn try_from(mut bytes: PacketBytes) -> Result<Self, Self::Error> {
        let cursor = &mut bytes.cursor;
        let mut pack_header = [0; 5];
        cursor.read_exact(&mut pack_header)?;

        let packet = match pack_header {
            [0xFF, 0xFF, 0xFF, 0xFF, 0x49] => Packet::Info(bytes.try_into()?),
            [0xFF, 0xFF, 0xFF, 0xFF, 0x45] => Packet::Rules(bytes.try_into()?),
            [0xFF, 0xFF, 0xFF, 0xFF, 0x44] => Packet::Players(bytes.try_into()?),
            [0xFF, 0xFF, 0xFF, 0xFF, 0x41] => Packet::Challenge(bytes.try_into()?),
            _ => {
                eprintln!("Unknown packet:");
                eprintln!("{bytes:?}");
                Packet::Unknown(bytes)
            }
        };

        Ok(packet)
    }
}

impl Packet {
    pub fn bytes(&self) -> &PacketBytes {
        match self {
            Packet::Unknown(b) => b,
            Packet::Info(p) => p.bytes(),
            Packet::Rules(p) => p.bytes(),
            Packet::Players(p) => p.bytes(),
            Packet::Challenge(p) => p.bytes(),
        }
    }
}

pub struct PacketBytes {
    cursor: Cursor<Vec<u8>>,
    pos: Vec<u64>,
}

impl PacketBytes {
    pub fn cursor(&self) -> &Cursor<Vec<u8>> {
        &self.cursor
    }

    pub fn read_u8(&mut self) -> Result<u8, Box<dyn Error>> {
        let mut buf = [0; 1];
        self.cursor.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    pub fn read_u16(&mut self) -> Result<u16, Box<dyn Error>> {
        let mut buf = [0; 2];
        self.cursor.read_exact(&mut buf)?;
        let val = u16::from_le_bytes(buf);
        Ok(val)
    }

    pub fn read_i32(&mut self) -> Result<i32, Box<dyn Error>> {
        let mut buf = [0; 4];
        self.cursor.read_exact(&mut buf)?;
        let val = i32::from_le_bytes(buf);
        Ok(val)
    }

    pub fn read_f32(&mut self) -> Result<f32, Box<dyn Error>> {
        let mut buf = [0; 4];
        self.cursor.read_exact(&mut buf)?;
        let val = f32::from_le_bytes(buf);
        Ok(val)
    }

    pub fn read_until_zero(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buf = vec![];
        self.cursor.read_until(0, &mut buf)?;
        if buf.ends_with(&[0]) {
            buf.pop();
        }
        Ok(buf)
    }

    pub fn read_str(&mut self) -> Result<String, Box<dyn Error>> {
        let bytes = self.read_until_zero()?;
        let str = String::from_utf8_lossy(&bytes).to_string();
        Ok(str)
    }
}

impl From<&[u8]> for PacketBytes {
    fn from(bytes: &[u8]) -> Self {
        let bytes = Vec::from(bytes);
        let cursor = Cursor::new(bytes);
        Self {
            cursor,
            pos: vec![],
        }
    }
}

impl Debug for PacketBytes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&pretty_hex(self.cursor.get_ref()))
    }
}

trait RecvPacketBytes {
    fn recv_packet_bytes(&self) -> Result<PacketBytes, Box<dyn Error>>;
}

impl RecvPacketBytes for UdpSocket {
    fn recv_packet_bytes(&self) -> Result<PacketBytes, Box<dyn Error>> {
        const MAX_PACKET_SIZE: usize = 65535;

        let mut buf = [0u8; MAX_PACKET_SIZE];
        let packet_size = self.recv(&mut buf)?;
        let packet = buf[0..packet_size].into();
        Ok(packet)
    }
}

pub(crate) fn send_a2s_packet<A: ToSocketAddrs>(
    socket: &UdpSocket,
    addr: A,
    packet: &[u8],
    challenge: &[u8],
    retry_num: u8,
) -> Result<Packet, Box<dyn Error>> {
    const MAX_RETRIES: u8 = 3;

    let mut bytes = vec![];
    bytes.extend(packet);
    bytes.extend(challenge);
    socket.send_to(bytes.as_ref(), &addr)?;

    let reply = socket.recv_packet_bytes()?.try_into()?;
    if let Packet::Challenge(p) = &reply {
        if retry_num < MAX_RETRIES {
            return send_a2s_packet(socket, addr, packet, p.challenge(), retry_num + 1);
        }
    }

    Ok(reply)
}

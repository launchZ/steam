use pretty_hex::pretty_hex;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::io::BufRead;
use std::io::Cursor;
use std::io::Read;
use std::net::ToSocketAddrs;
use std::net::UdpSocket;

pub fn query_info<A: ToSocketAddrs>(socket: &UdpSocket, addr: A) -> Result<Packet, Box<dyn Error>> {
    send_a2s_packet(
        &socket,
        addr,
        &[
            0xFF, 0xFF, 0xFF, 0xFF, 0x54, 0x53, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x20, 0x45, 0x6E,
            0x67, 0x69, 0x6E, 0x65, 0x20, 0x51, 0x75, 0x65, 0x72, 0x79, 0x00,
        ],
        &[],
        0,
    )
}

pub fn query_rules<A: ToSocketAddrs>(
    socket: &UdpSocket,
    addr: A,
) -> Result<Packet, Box<dyn Error>> {
    send_a2s_packet(
        &socket,
        addr,
        &[0xFF, 0xFF, 0xFF, 0xFF, 0x56],
        &[0xFF, 0xFF, 0xFF, 0xFF],
        0,
    )
}

pub fn query_players<A: ToSocketAddrs>(
    socket: &UdpSocket,
    addr: A,
) -> Result<Packet, Box<dyn Error>> {
    send_a2s_packet(
        &socket,
        addr,
        &[0xFF, 0xFF, 0xFF, 0xFF, 0x55],
        &[0xFF, 0xFF, 0xFF, 0xFF],
        0,
    )
}

pub struct PacketBytes {
    cursor: Cursor<Vec<u8>>,
}

pub enum Packet {
    Unknown(PacketBytes),
    Info(Packet_Info),
    Rules(Packet_Rules),
    Players(Packet_Players),
    Challenge(Packet_Challenge),
}

#[allow(non_camel_case_types)]
pub struct Packet_Info {
    bytes: PacketBytes,
}

#[allow(non_camel_case_types)]
pub struct Packet_Rules {
    bytes: PacketBytes,
    // rules: Vec<Rule>,
    // mods: Vec<Mod>,
}

#[allow(non_camel_case_types)]
#[allow(unused)]
pub struct Packet_Players {
    bytes: PacketBytes,
    player_count: u8,
    players: Vec<Player>,
}

#[allow(non_camel_case_types)]
pub struct Packet_Challenge {
    bytes: PacketBytes,
    challenge: [u8; 4],
}

pub struct Rule {
    pub name: String,
    pub value: String,
}

pub struct Mod {
    pub id: u32,
    pub name: String,
}

pub struct Player {
    pub index: u8,
    pub name: String,
    pub score: i32,
    pub duration: f32,
}

impl From<&[u8]> for PacketBytes {
    fn from(bytes: &[u8]) -> Self {
        let bytes = Vec::from(bytes);
        let cursor = Cursor::new(bytes);
        Self { cursor }
    }
}

impl Debug for PacketBytes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&pretty_hex(self.cursor.get_ref()))
    }
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

    pub fn read_str(&mut self) -> Result<String, Box<dyn Error>> {
        let mut buf = vec![];
        self.cursor.read_until(0, &mut buf)?;
        if buf.ends_with(&[0]) {
            _ = buf.pop();
        }
        let str = String::from_utf8_lossy(&buf).to_string();
        Ok(str)
    }
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
            Packet::Info(p) => &p.bytes,
            Packet::Rules(p) => &p.bytes,
            Packet::Players(p) => &p.bytes,
            Packet::Challenge(p) => &p.bytes,
        }
    }
}

impl TryFrom<PacketBytes> for Packet_Info {
    type Error = Box<dyn Error>;

    fn try_from(bytes: PacketBytes) -> Result<Self, Self::Error> {
        Ok(Self { bytes })
    }
}

impl TryFrom<PacketBytes> for Packet_Rules {
    type Error = Box<dyn Error>;

    fn try_from(mut bytes: PacketBytes) -> Result<Self, Self::Error> {
        let rule_count = bytes.read_u16()?;
        bytes.cursor.consume(12);
        let mod_count = bytes.read_u8()?;
        println!("rule_count={rule_count}");
        println!("mod_count={mod_count}");
        Ok(Self { bytes })
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

fn send_a2s_packet<A: ToSocketAddrs>(
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
            return send_a2s_packet(socket, addr, packet, &p.challenge, retry_num + 1);
        }
    }

    Ok(reply)
}

use pretty_hex::pretty_hex;
use std::error::Error;
use std::io::Cursor;
use std::io::Read;
use std::net::ToSocketAddrs;
use std::net::UdpSocket;
use std::time::Duration;

pub fn query_info<A: ToSocketAddrs>(addr: A) -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let timeout = Duration::from_secs(5);
    socket.set_read_timeout(Some(timeout))?;
    socket.set_write_timeout(Some(timeout))?;

    let packet = [
        0xFF, 0xFF, 0xFF, 0xFF, 0x54, 0x53, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x20, 0x45, 0x6E, 0x67,
        0x69, 0x6E, 0x65, 0x20, 0x51, 0x75, 0x65, 0x72, 0x79, 0x00,
    ];

    socket.send_to(&packet, &addr)?;
    let reply = send_a2s_packet(&socket, addr, &packet, &[], 0)?;
    debug_print(&reply); // TODO: Handle reply

    Ok(())
}

pub fn query_players<A: ToSocketAddrs>(addr: A) -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let timeout = Duration::from_secs(5);
    socket.set_read_timeout(Some(timeout))?;
    socket.set_write_timeout(Some(timeout))?;

    let packet = [0xFF, 0xFF, 0xFF, 0xFF, 0x55];
    let initial_challenge = [0xFF, 0xFF, 0xFF, 0xFF];

    socket.send_to(&packet, &addr)?;
    let reply = send_a2s_packet(&socket, addr, &packet, &initial_challenge, 0)?;
    debug_print(&reply); // TODO: Handle reply

    Ok(())
}

fn debug_print(p: &Packet) {
    let packet_type = match p {
        Packet::Info(_) => "Info",
        Packet::Players(_) => "Players",
        Packet::Challenge(_) => "Challenge",
        Packet::Unknown(_) => "Unknown",
    };
    let bytes = match p {
        Packet::Info(p) => p.bytes.clone(),
        Packet::Players(p) => p.bytes.clone(),
        Packet::Challenge(p) => p.bytes.clone(),
        Packet::Unknown(b) => b.clone(),
    };
    println!();
    println!("Packet: {packet_type}");
    println!("{}", pretty_hex(bytes.get_ref()));
}

type PacketBytes = Cursor<Vec<u8>>;

enum Packet {
    Info(Packet_Info),
    Players(Packet_Players),
    Challenge(Packet_Challenge),
    Unknown(PacketBytes),
}

#[allow(non_camel_case_types)]
struct Packet_Info {
    bytes: PacketBytes,
}

#[allow(non_camel_case_types)]
struct Packet_Players {
    bytes: PacketBytes,
}

#[allow(non_camel_case_types)]
struct Packet_Challenge {
    bytes: PacketBytes,
    bytes_challenge: [u8; 4],
}

impl TryFrom<PacketBytes> for Packet {
    type Error = Box<dyn Error>;

    fn try_from(bytes: PacketBytes) -> Result<Self, Self::Error> {
        let mut bytes = bytes.clone();

        let mut pack_header = [0; 5];
        bytes.read_exact(&mut pack_header)?;
        bytes.set_position(0);

        let packet = match pack_header {
            [0xFF, 0xFF, 0xFF, 0xFF, 0x49] => Packet::Info(bytes.try_into()?),
            [0xFF, 0xFF, 0xFF, 0xFF, 0x44] => Packet::Players(bytes.try_into()?),
            [0xFF, 0xFF, 0xFF, 0xFF, 0x41] => Packet::Challenge(bytes.try_into()?),
            _ => Packet::Unknown(bytes),
        };

        Ok(packet)
    }
}

impl TryFrom<PacketBytes> for Packet_Info {
    type Error = Box<dyn Error>;

    fn try_from(bytes: PacketBytes) -> Result<Self, Self::Error> {
        Ok(Self { bytes })
    }
}

impl TryFrom<PacketBytes> for Packet_Players {
    type Error = Box<dyn Error>;

    fn try_from(bytes: PacketBytes) -> Result<Self, Self::Error> {
        Ok(Self { bytes })
    }
}

impl TryFrom<PacketBytes> for Packet_Challenge {
    type Error = Box<dyn Error>;

    fn try_from(mut bytes: PacketBytes) -> Result<Self, Self::Error> {
        let mut bytes_challenge = [0; 4];
        bytes.set_position(5);
        bytes.read_exact(&mut bytes_challenge)?;
        Ok(Self {
            bytes,
            bytes_challenge,
        })
    }
}

trait RecvVecCursor {
    fn recv_vec_cursor(&self) -> Result<Cursor<Vec<u8>>, Box<dyn Error>>;
}

impl RecvVecCursor for UdpSocket {
    fn recv_vec_cursor(&self) -> Result<Cursor<Vec<u8>>, Box<dyn Error>> {
        let mut buf = [0u8; 65535];
        let packet_size = self.recv(&mut buf)?;
        let packet = Cursor::new(Vec::from(&buf[0..packet_size]));
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

    let reply = socket.recv_vec_cursor()?.try_into()?;
    if let Packet::Challenge(p) = &reply {
        if retry_num < MAX_RETRIES {
            return send_a2s_packet(socket, addr, packet, &p.bytes_challenge, retry_num + 1);
        }
    }

    Ok(reply)
}

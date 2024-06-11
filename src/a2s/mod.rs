use crate::a2s::packet::Packet;
use std::error::Error;
use std::net::{ToSocketAddrs, UdpSocket};

pub mod packet;

pub fn query_info<A: ToSocketAddrs>(socket: &UdpSocket, addr: A) -> Result<Packet, Box<dyn Error>> {
    packet::send_a2s_packet(
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
    packet::send_a2s_packet(
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
    packet::send_a2s_packet(
        &socket,
        addr,
        &[0xFF, 0xFF, 0xFF, 0xFF, 0x55],
        &[0xFF, 0xFF, 0xFF, 0xFF],
        0,
    )
}

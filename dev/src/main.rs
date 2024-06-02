use std::error::Error;
use std::fs;
use std::net::UdpSocket;
use std::time::Duration;
use steam::a2s::packet::{query_info, query_players, query_rules, Packet};

const ADDR: &str = "142.4.217.38:27016";
// const ADDR: &str = "195.201.150.169:2402";
// const ADDR: &str = "31.220.100.120:2411";
// const ADDR: &str = "109.248.4.55:2333";
// const ADDR: &str = "5.252.100.201:2303";

fn main() {
    let socket = open_socket().unwrap();
    let rules = query_rules(&socket, ADDR).unwrap();
    print_debug(&rules);
    save_to_file(&rules);

    if let Packet::Rules(_) = rules {}
}

fn open_socket() -> Result<UdpSocket, Box<dyn Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let timeout = Some(Duration::from_secs(5));
    socket.set_read_timeout(timeout)?;
    socket.set_write_timeout(timeout)?;
    Ok(socket)
}

fn print_debug(packet: &Packet) {
    println!();
    println!("{:?}", packet.bytes());
}

fn save_to_file(packet: &Packet) {
    fs::write("packet.bin", &packet.bytes().cursor().get_ref()).unwrap();
}

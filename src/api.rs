use crate::{command::Command, errors::CommandDispatchError};
use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    time::Instant,
};

const UDP_IP: &str = "0.0.0.0";
const UDP_PORT: u16 = 5625;
const COMMAND_PORT: u16 = 5600;
const BUFFER_SIZE: usize = 4096;

pub fn _scan_all_devices(timeout: u64) -> HashMap<String, SocketAddr> {
    let mut map: HashMap<String, SocketAddr> = HashMap::new();
    let sock = UdpSocket::bind(format!("{}:{}", UDP_IP, UDP_PORT)).expect("failed to bind socket");
    let mut now = Instant::now();
    while now.elapsed().as_secs() < timeout {
        let mut buf = [0; BUFFER_SIZE];
        let (amt, src) = sock
            .recv_from(&mut buf)
            .expect("failed to receive from socket");
        let buf = &mut buf[..amt];
        let mac = String::from_utf8_lossy(&buf[..12]).to_string();
        if buf.len() <= 15 && !mac.contains(&mac) {
            map.insert(mac, src);
            now = Instant::now(); // refresh timeout if we successfully add one
        }
    }
    map
}

// Scan for an atomberg device and return the first one found.
pub fn scan_one_device() -> Option<(String, SocketAddr)> {
    let sock = UdpSocket::bind(format!("{}:{}", UDP_IP, UDP_PORT)).expect("failed to bind socket");
    let mut buf = [0; BUFFER_SIZE];
    let (amt, src) = sock
        .recv_from(&mut buf)
        .expect("failed to receive from socket");
    let buf = &mut buf[..amt];
    if buf.len() <= 15 {
        return Some((String::from_utf8_lossy(&buf[..12]).to_string(), src));
    }
    None
}

pub fn send_command(
    device_ip: Ipv4Addr,
    command: &Command,
) -> Result<usize, CommandDispatchError> {
    let addr = SocketAddr::new(std::net::IpAddr::V4(device_ip), COMMAND_PORT);
    let sock =
        UdpSocket::bind(format!("{}:{}", UDP_IP, COMMAND_PORT)).expect("failed to bind socket");
    let result = command.json();
    match result {
        Ok(json) => match sock.send_to(json.as_bytes(), addr) {
            Ok(size) => Ok(size),
            Err(e) => Err(CommandDispatchError::IOError(e)),
        },
        Err(e) => Err(CommandDispatchError::SerdeError(e)),
    }
}

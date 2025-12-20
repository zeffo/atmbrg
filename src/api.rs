use std::{collections::HashMap, net::{SocketAddr, UdpSocket}};

use serde::{Deserialize, Serialize};

const UDP_IP: &str = "0.0.0.0";
const UDP_PORT: i32 = 5625;
const COMMAND_PORT: i32 = 5600;
const BUFFER_SIZE: usize = 4096;

#[derive(Serialize, Deserialize)]
pub struct Command {
    power: Option<bool>,
    speed: Option<u8>,
    speed_delta: Option<i8>,
    sleep: Option<bool>,
    timer: Option<u8>,
    led: Option<bool>,
    brightness: Option<u8>,
    brightness_delta: Option<i8>,
    light_mode: Option<String>
}

pub fn scan_devices() -> Option<(SocketAddr, String)> {
    let sock = UdpSocket::bind(format!("{}:{}", UDP_IP, UDP_PORT)).expect("failed to bind socket");
    let mut buf = [0; BUFFER_SIZE];
    let (amt, src) = sock.recv_from(&mut buf).expect("failed to receive from socket");
    let buf = &mut buf[..amt];
    if buf.len() <= 15 {
        return Some((src, String::from_utf8_lossy(&buf[..12]).to_string()));
    }
    None
}

pub fn send_command<K, V>(device_ip: SocketAddr, command: HashMap<K, V>) {
    let sock = UdpSocket::bind(format!("{}:{}", device_ip.ip(), COMMAND_PORT)).expect("failed to bind socket");
    sock.send(c)
    
}



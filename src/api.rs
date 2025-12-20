use crate::{command::Command, errors::CommandDispatchError};
use std::net::{SocketAddr, UdpSocket};

const UDP_IP: &str = "0.0.0.0";
const UDP_PORT: i32 = 5625;
const COMMAND_PORT: i32 = 5600;
const BUFFER_SIZE: usize = 4096;

pub fn scan_devices() -> Option<(SocketAddr, String)> {
    let sock = UdpSocket::bind(format!("{}:{}", UDP_IP, UDP_PORT)).expect("failed to bind socket");
    let mut buf = [0; BUFFER_SIZE];
    let (amt, src) = sock
        .recv_from(&mut buf)
        .expect("failed to receive from socket");
    let buf = &mut buf[..amt];
    if buf.len() <= 15 {
        return Some((src, String::from_utf8_lossy(&buf[..12]).to_string()));
    }
    None
}

pub fn send_command(
    mut device_ip: SocketAddr,
    command: &Command,
) -> Result<usize, CommandDispatchError> {
    device_ip.set_port(COMMAND_PORT as u16);
    let sock =
        UdpSocket::bind(format!("{}:{}", UDP_IP, COMMAND_PORT)).expect("failed to bind socket");
    let result = command.json();
    match result {
        Ok(json) => match sock.send_to(json.as_bytes(), device_ip) {
            Ok(size) => {
                dbg!(json);
                Ok(size)
            },
            Err(e) => Err(CommandDispatchError::IOError(e)),
        },
        Err(e) => Err(CommandDispatchError::SerdeError(e)),
    }
}

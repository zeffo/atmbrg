
pub(crate) mod api;
pub(crate) mod app;
pub(crate) mod errors;
pub(crate) mod command;


use crate::api::{send_command};

fn main() {
    let command = command::Command::Timer(command::Timer::Three);
    loop {
        let device = api::scan_devices();
        match device {
            Some((sock, _ip)) => match send_command(sock, &command) {
                Ok(_) => {
                    println!("command sent successfully");
                    break;
                }
                Err(e) => println!("failed to send command: {:#?}", e),
            },
            None => println!("no devices found"),
        }
    }
}

pub(crate) mod api;
pub(crate) mod app;
pub(crate) mod command;
pub(crate) mod errors;

use clap::{Parser, ValueEnum};

use crate::{
    api::{scan_one_device, send_command},
    command::{Color, Command, Timer},
};

// use crate::api::send_command;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(long, global = true)]
    device: Option<String>,
    #[clap(subcommand)]
    command: CLICommand,
}

#[derive(clap::Subcommand, Debug, Clone)]
enum CLICommand {
    Power { toggle: Toggle },
    Speed { value: u8 },
    SpeedDelta { delta: i8 },
    Sleep { toggle: Toggle },
    Timer { timer: Timer },
    Led { toggle: Toggle },
    Brightness { value: i8 },
    BrightnessDelta { delta: u8 },
    Color { color: Color },
    ScanOne,
}

#[derive(Clone, Debug, ValueEnum)]
enum Toggle {
    On,
    Off,
}

impl From<Toggle> for bool {
    fn from(value: Toggle) -> Self {
        match value {
            Toggle::On => true,
            Toggle::Off => false,
        }
    }
}

impl TryFrom<CLICommand> for Command {
    type Error = &'static str;
    fn try_from(value: CLICommand) -> Result<Self, Self::Error> {
        match value {
            CLICommand::Power { toggle } => Ok(Command::Power(toggle.into())),
            CLICommand::Speed { value } => Ok(Command::Speed(value)),
            CLICommand::SpeedDelta { delta } => Ok(Command::SpeedDelta(delta)),
            CLICommand::Sleep { toggle } => Ok(Command::Sleep(toggle.into())),
            CLICommand::Timer { timer } => Ok(Command::Timer(timer)),
            CLICommand::Led { toggle } => Ok(Command::Led(toggle.into())),
            CLICommand::Brightness { value } => Ok(Command::Brightness(value)),
            CLICommand::BrightnessDelta { delta } => Ok(Command::BrightnessDelta(delta)),
            CLICommand::Color { color } => Ok(Command::Color(color)),
            CLICommand::ScanOne => Err("invalid device command"),
        }
    }
}

fn main() {
    let args = Args::parse();
    let command = args.command;
    match command {
        CLICommand::ScanOne => match scan_one_device() {
            Some((_mac, addr)) => println!("{}", addr.ip()),
            None => {
                eprintln!("could not find an atomberg device");
                std::process::exit(68);
            }
        },
        _ => {
            if args.device.is_none() {
                eprintln!("--device flag is required");
                std::process::exit(64);
            }
            let dev = args.device.unwrap();
            let device = match dev.parse() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("--device flag requires a valid ipv4 address ({e:#?})");
                    std::process::exit(65); 
                }
            };
            let cmd = match Command::try_from(command) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("could not convert to device command: {:#?}", e);
                    std::process::exit(70);
                }
            };
            match send_command(device, &cmd) {
                Ok(_) => {
                    println!("command sent successfully")
                }
                Err(e) => {
                    eprintln!("failed to send command: {:#?}", e);
                    std::process::exit(70);
                }
            }
        }
    }
}

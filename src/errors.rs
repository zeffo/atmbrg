use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum CommandDispatchError {
    IOError(std::io::Error),
    SerdeError(serde_json::Error),
}

impl Display for CommandDispatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::IOError(e) => e.fmt(f),
            Self::SerdeError(e) => e.fmt(f),
        }
    }
}

impl Error for CommandDispatchError {}

#[derive(Debug)]
pub struct InvalidCommandError {
    message: String,
}

impl InvalidCommandError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl Display for InvalidCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message.as_ref())
    }
}

impl Error for InvalidCommandError {}

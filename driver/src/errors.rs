use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub enum PLCError{
    FailedToSend(String),
    FailedToRecieve(String),
    Disconnected(),
    Initialization(String),
}
impl Error for PLCError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
impl fmt::Display for PLCError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PLCError::FailedToSend(ref msg) => write!(f, "SendError: {}", msg),
            PLCError::FailedToRecieve(ref msg) => write!(f, "RecieveError: {}", msg),
            PLCError::Disconnected() => write!(f, "PLC appears to be disconnected"),
            PLCError::Initialization(ref msg) => write!(f, "Could not initialize: {}", msg)
        }
    }
}

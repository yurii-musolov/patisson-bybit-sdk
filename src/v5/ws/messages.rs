use crate::v5::{IncomingMessage, OutgoingMessage};

#[derive(Debug)]
pub enum Command {
    Connect,
    Send(OutgoingMessage),
    Disconnect,
}

#[derive(Debug)]
pub enum Event {
    Connected,
    Message(IncomingMessage),
    Reconnecting { attempt: u32, delay_ms: u64 },
    Disconnected { reason: DisconnectReason },
}

#[derive(Debug, Clone)]
pub enum DisconnectReason {
    Requested,
    RemoteClosed,
    PongTimeout,
    Error(String),
}

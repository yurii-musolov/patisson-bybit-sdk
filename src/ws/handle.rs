use tokio::sync::mpsc;

use crate::{
    Topic,
    ws::{self, Command, OutgoingMessage},
};

/// Sends commands to a running [`Stream`](crate::ws::Stream); cheap to clone.
#[derive(Debug, Clone)]
pub struct Handle {
    cmd_tx: mpsc::Sender<Command>,
}

impl Handle {
    /// Wrap the command sender of a driver; normally obtained from [`Stream::new`](crate::ws::Stream::new).
    pub fn new(cmd_tx: mpsc::Sender<Command>) -> Self {
        Self { cmd_tx }
    }

    /// Open the connection (no-op if already connected).
    pub async fn connect(&self) -> Result<(), ws::Error> {
        self.send(Command::Connect).await
    }

    /// Like [`Handle::connect`], but fails with [`Error::QueueFull`](crate::ws::Error::QueueFull) instead of waiting.
    pub fn try_connect(&self) -> Result<(), ws::Error> {
        self.try_send(Command::Connect)
    }

    /// Close the connection; the driver stays alive and can connect again.
    pub async fn disconnect(&self) -> Result<(), ws::Error> {
        self.send(Command::Disconnect).await
    }

    /// Like [`Handle::disconnect`], but fails with [`Error::QueueFull`](crate::ws::Error::QueueFull) instead of waiting.
    pub fn try_disconnect(&self) -> Result<(), ws::Error> {
        self.try_send(Command::Disconnect)
    }

    /// Send a raw message on the current connection (not restored after a reconnect).
    pub async fn send_command(&self, msg: OutgoingMessage) -> Result<(), ws::Error> {
        self.send(Command::Send(msg)).await
    }

    /// Like [`Handle::send_command`], but fails with [`Error::QueueFull`](crate::ws::Error::QueueFull) instead of waiting.
    pub fn try_send_command(&self, msg: OutgoingMessage) -> Result<(), ws::Error> {
        self.try_send(Command::Send(msg))
    }

    /// Subscribe to `topics` now and again after every reconnect.
    pub async fn subscribe(&self, topics: Vec<Topic>) -> Result<(), ws::Error> {
        self.send(Command::Subscribe(topics)).await
    }

    /// Like [`Handle::subscribe`], but fails with [`Error::QueueFull`](crate::ws::Error::QueueFull) instead of waiting.
    pub fn try_subscribe(&self, topics: Vec<Topic>) -> Result<(), ws::Error> {
        self.try_send(Command::Subscribe(topics))
    }

    /// Unsubscribe from `topics` and stop restoring them.
    pub async fn unsubscribe(&self, topics: Vec<Topic>) -> Result<(), ws::Error> {
        self.send(Command::Unsubscribe(topics)).await
    }

    /// Like [`Handle::unsubscribe`], but fails with [`Error::QueueFull`](crate::ws::Error::QueueFull) instead of waiting.
    pub fn try_unsubscribe(&self, topics: Vec<Topic>) -> Result<(), ws::Error> {
        self.try_send(Command::Unsubscribe(topics))
    }

    async fn send(&self, cmd: Command) -> Result<(), ws::Error> {
        self.cmd_tx
            .send(cmd)
            .await
            .map_err(|_| ws::Error::DriverGone)
    }

    fn try_send(&self, cmd: Command) -> Result<(), ws::Error> {
        self.cmd_tx.try_send(cmd).map_err(map_try_send_err)
    }
}

fn map_try_send_err(e: mpsc::error::TrySendError<Command>) -> ws::Error {
    match e {
        mpsc::error::TrySendError::Full(_) => ws::Error::QueueFull,
        mpsc::error::TrySendError::Closed(_) => ws::Error::DriverGone,
    }
}

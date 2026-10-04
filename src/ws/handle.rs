use tokio::sync::mpsc;

use crate::ws::{self, Command, OutgoingMessage};

#[derive(Clone)]
pub struct Handle {
    cmd_tx: mpsc::Sender<Command>,
}

impl Handle {
    pub fn new(cmd_tx: mpsc::Sender<Command>) -> Self {
        Self { cmd_tx }
    }

    pub async fn connect(&self) -> Result<(), ws::Error> {
        self.send(Command::Connect).await
    }

    pub fn try_connect(&self) -> Result<(), ws::Error> {
        self.try_send(Command::Connect)
    }

    pub async fn disconnect(&self) -> Result<(), ws::Error> {
        self.send(Command::Disconnect).await
    }

    pub fn try_disconnect(&self) -> Result<(), ws::Error> {
        self.try_send(Command::Disconnect)
    }

    pub async fn send_command(&self, msg: OutgoingMessage) -> Result<(), ws::Error> {
        self.send(Command::Send(msg)).await
    }

    pub fn try_send_command(&self, msg: OutgoingMessage) -> Result<(), ws::Error> {
        self.try_send(Command::Send(msg))
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

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
        let cmd = Command::Connect;
        self.cmd_tx
            .send(cmd)
            .await
            .map_err(|_| ws::Error::DriverGone)
    }

    pub fn try_connect(&self) -> Result<(), ws::Error> {
        let cmd = Command::Connect;
        self.cmd_tx.try_send(cmd).map_err(|e| match e {
            mpsc::error::TrySendError::Full(_) => ws::Error::QueueFull,
            mpsc::error::TrySendError::Closed(_) => ws::Error::DriverGone,
        })
    }

    pub async fn disconnect(&self) -> Result<(), ws::Error> {
        let cmd = Command::Disconnect;
        self.cmd_tx
            .send(cmd)
            .await
            .map_err(|_| ws::Error::DriverGone)
    }

    pub fn try_disconnect(&self) -> Result<(), ws::Error> {
        let cmd = Command::Disconnect;
        self.cmd_tx.try_send(cmd).map_err(|e| match e {
            mpsc::error::TrySendError::Full(_) => ws::Error::QueueFull,
            mpsc::error::TrySendError::Closed(_) => ws::Error::DriverGone,
        })
    }

    pub async fn send_command(&self, msg: OutgoingMessage) -> Result<(), ws::Error> {
        let cmd = Command::Send(msg);
        self.cmd_tx
            .send(cmd)
            .await
            .map_err(|_| ws::Error::DriverGone)
    }

    pub fn try_send_command(&self, msg: OutgoingMessage) -> Result<(), ws::Error> {
        let cmd = Command::Send(msg);
        self.cmd_tx.try_send(cmd).map_err(|e| match e {
            mpsc::error::TrySendError::Full(_) => ws::Error::QueueFull,
            mpsc::error::TrySendError::Closed(_) => ws::Error::DriverGone,
        })
    }
}

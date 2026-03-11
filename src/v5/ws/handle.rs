use tokio::sync::mpsc;

use crate::v5::{
    OutgoingMessage,
    ws::{Command, WsError},
};

#[derive(Clone)]
pub struct WsHandle {
    cmd_tx: mpsc::Sender<Command>,
}

impl WsHandle {
    pub fn new(cmd_tx: mpsc::Sender<Command>) -> Self {
        WsHandle { cmd_tx }
    }

    pub async fn connect(&self) -> Result<(), WsError> {
        let cmd = Command::Connect;
        self.cmd_tx.send(cmd).await.map_err(|_| WsError::DriverGone)
    }

    pub fn try_connect(&self) -> Result<(), WsError> {
        let cmd = Command::Connect;
        self.cmd_tx.try_send(cmd).map_err(|e| match e {
            mpsc::error::TrySendError::Full(_) => WsError::QueueFull,
            mpsc::error::TrySendError::Closed(_) => WsError::DriverGone,
        })
    }

    pub async fn disconnect(&self) -> Result<(), WsError> {
        let cmd = Command::Disconnect;
        self.cmd_tx.send(cmd).await.map_err(|_| WsError::DriverGone)
    }

    pub fn try_disconnect(&self) -> Result<(), WsError> {
        let cmd = Command::Disconnect;
        self.cmd_tx.try_send(cmd).map_err(|e| match e {
            mpsc::error::TrySendError::Full(_) => WsError::QueueFull,
            mpsc::error::TrySendError::Closed(_) => WsError::DriverGone,
        })
    }

    pub async fn send_command(&self, msg: OutgoingMessage) -> Result<(), WsError> {
        let cmd = Command::Send(msg);
        self.cmd_tx.send(cmd).await.map_err(|_| WsError::DriverGone)
    }

    pub fn try_send_command(&self, msg: OutgoingMessage) -> Result<(), WsError> {
        let cmd = Command::Send(msg);
        self.cmd_tx.try_send(cmd).map_err(|e| match e {
            mpsc::error::TrySendError::Full(_) => WsError::QueueFull,
            mpsc::error::TrySendError::Closed(_) => WsError::DriverGone,
        })
    }
}

use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;

pub type FrameResult = Result<Message, tokio_tungstenite::tungstenite::Error>;
pub type Sink = Box<
    dyn futures_util::Sink<Message, Error = tokio_tungstenite::tungstenite::Error> + Send + Unpin,
>;

pub enum State {
    Idle,
    Connecting {
        attempt: u32,
    },
    Connected {
        frame_rx: mpsc::Receiver<FrameResult>,
        read_task: tokio::task::JoinHandle<()>,
        sink: Sink,
    },
    Reconnecting {
        attempt: u32,
        delay_ms: u64,
    },
    Closing {
        sink: Sink,
    },
    Done,
}

pub enum HeartbeatState {
    Idle,
    PingSent,
}

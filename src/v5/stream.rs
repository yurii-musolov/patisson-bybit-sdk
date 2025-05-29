use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::{
    self,
    sync::mpsc::{Receiver, Sender, channel},
    time::sleep,
};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Utf8Bytes, http, protocol::Message},
};

use super::{IncomingMessage, OutgoingMessage};

pub async fn stream(
    url: &str,
    ping_interval: Duration,
) -> anyhow::Result<(
    Sender<OutgoingMessage>,
    Receiver<IncomingMessage>,
    http::response::Response<Option<Vec<u8>>>,
)> {
    let (incoming_tx, incoming_rx) = channel::<IncomingMessage>(1);
    let (outgoing_tx, mut outgoing_rx) = channel::<OutgoingMessage>(1);

    let (stream, response) = connect_async(url).await?;
    let (mut sender, mut receiver) = stream.split();

    let handshake = outgoing_tx.clone();
    tokio::spawn(async move {
        let mut count = 0_u64;
        loop {
            sleep(ping_interval).await;
            count += 1;
            let id = format!("ping-{count}");
            let message = OutgoingMessage::Ping { req_id: Some(id) };
            if let Err(e) = handshake.send(message).await {
                println!("Send ping error: {e}");
                break;
            };
        }
    });

    tokio::spawn(async move {
        while let Some(result) = receiver.next().await {
            match result {
                Ok(message) => match message {
                    Message::Text(slice) => {
                        match serde_json::from_slice(slice.as_ref()) {
                            Ok(message) => {
                                if let Err(e) = incoming_tx.send(message).await {
                                    println!("Send IncomingMessage failed with: {e}");
                                }
                            }
                            Err(e) => {
                                println!("Deserialize IncomingMessage failed with: {e}");
                                println!("DEBUG: IncomingMessage: {slice}");
                            }
                        };
                    }
                    Message::Binary(d) => println!("Binary got {} bytes: {:?}", d.len(), d),
                    Message::Close(close_frame) => {
                        match close_frame {
                            Some(close_frame) => println!(
                                "Close got close with code {} and reason `{}`",
                                close_frame.code, close_frame.reason
                            ),
                            None => {
                                println!("Close somehow got close message without CloseFrame")
                            }
                        }

                        break;
                    }
                    Message::Pong(v) => println!("Pong got pong with {v:?}."),
                    Message::Ping(v) => println!("Ping got ping with {v:?}."),
                    Message::Frame(_) => {
                        unreachable!("Frame This is never supposed to happen.")
                    }
                },
                Err(e) => println!("Receive message failed with: {e}"),
            }
        }
    });

    tokio::spawn(async move {
        while let Some(message) = outgoing_rx.recv().await {
            let message = match serde_json::to_string(&message) {
                Ok(serialized) => Message::Text(Utf8Bytes::from(&serialized)),
                Err(e) => {
                    println!("Serialize OutgoingMessage failed with {e}");
                    continue;
                }
            };

            if let Err(e) = sender.send(message).await {
                println!("Send OutgoingMessage failed with {e}");
            };
        }
    });

    Ok((outgoing_tx, incoming_rx, response))
}

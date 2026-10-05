//! Logging of REST calls.
//!
//! An integration test (its own process) because it installs a global
//! `tracing` subscriber: with `set_default` in a multi-threaded unit test
//! binary, tracing's process-wide callsite cache made the test flaky.

use std::sync::{Arc, Mutex};

use bybit::{
    Category,
    http::{Client, Config, GetOrderHistoryParams},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

/// `MakeWriter` that collects formatted log output in memory.
#[derive(Clone, Default)]
struct LogBuffer(Arc<Mutex<Vec<u8>>>);

impl std::io::Write for LogBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for LogBuffer {
    type Writer = Self;
    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}

/// HTTP server that answers every request with a Bybit API error.
async fn spawn_failing_server() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        while let Ok((mut tcp, _)) = listener.accept().await {
            let mut head = Vec::new();
            let mut buf = [0u8; 1024];
            while !head.windows(4).any(|w| w == b"\r\n\r\n") {
                match tcp.read(&mut buf).await {
                    Ok(0) | Err(_) => break,
                    Ok(n) => head.extend_from_slice(&buf[..n]),
                }
            }
            let body = r#"{"retCode":10001,"retMsg":"test","result":{},"retExtInfo":{},"time":1}"#;
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
                body.len()
            );
            let _ = tcp.write_all(response.as_bytes()).await;
        }
    });
    format!("http://{addr}")
}

#[tokio::test]
async fn api_errors_are_logged_at_debug_without_request_parameters() {
    let logs = LogBuffer::default();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_ansi(false)
        .with_writer(logs.clone())
        .init();

    let client =
        Client::new(Config::new(spawn_failing_server().await).credentials("key", "secret"))
            .unwrap();
    let params = GetOrderHistoryParams::new(Category::Linear).with_symbol("SECRETUSDT".into());
    assert!(client.get_order_history(&params).await.is_err());

    let output = String::from_utf8(logs.0.lock().unwrap().clone()).unwrap();
    assert!(output.contains("api call failed"), "{output}");
    assert!(!output.contains("ERROR"), "{output}");
    assert!(!output.contains("SECRETUSDT"), "{output}");
}

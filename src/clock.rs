//! Server clock estimate shared by the REST client and WebSocket streams.

use std::sync::{
    Arc,
    atomic::{AtomicI64, Ordering},
};

use crate::{Timestamp, timestamp};

/// The Bybit server clock, estimated as the local clock plus an offset.
///
/// Cloning is cheap and clones share the offset: pass
/// [`http::Client::clock`](crate::http::Client::clock) to
/// [`ws::Config::server_clock`](crate::ws::Config::server_clock) so that the
/// stream `auth` uses the offset measured by
/// [`http::Client::sync_time`](crate::http::Client::sync_time).
#[derive(Debug, Clone, Default)]
pub struct ServerClock {
    offset_ms: Arc<AtomicI64>,
}

impl ServerClock {
    /// A clock with offset 0, i.e. the local clock.
    pub fn new() -> Self {
        Self::default()
    }

    /// Server clock minus local clock, milliseconds.
    pub fn offset(&self) -> i64 {
        self.offset_ms.load(Ordering::Relaxed)
    }

    pub fn set_offset(&self, offset_ms: i64) {
        self.offset_ms.store(offset_ms, Ordering::Relaxed);
    }

    /// Estimated current server time, milliseconds.
    pub fn now(&self) -> Timestamp {
        timestamp().saturating_add_signed(self.offset())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clones_share_the_offset() {
        let clock = ServerClock::new();
        let clone = clock.clone();

        clone.set_offset(60_000);

        assert_eq!(clock.offset(), 60_000);
        let skew = clock.now() as i64 - (timestamp() as i64 + 60_000);
        assert!(skew.abs() < 1000);
    }
}

//! Local order book reconstruction.
//!
//! Bybit's order book stream is built from a depth snapshot (REST
//! [`Orderbook`] or a WebSocket `"snapshot"` message) plus a stream of
//! incremental `"delta"` messages ([`OrderbookDataMsg`]). Each message
//! carries an `update_id` (`u`), a per-topic sequence number that increases
//! by exactly one for every message. If the backend restarts, the next
//! message is a fresh snapshot with `update_id == 1`, which must always
//! overwrite the local book.
//!
//! [`OrderBookState`] hides all of this bookkeeping. Snapshots and diffs can
//! be fed in any order: a diff that arrives before its snapshot is buffered
//! until the snapshot arrives, and a snapshot that arrives while diffs are
//! already buffered bridges over them if they form a contiguous chain.
//! Every call returns an [`ApplyOutcome`] that tells the caller what
//! happened and, in particular, whether the book is now out of sync and a
//! fresh snapshot must be fetched.

use std::collections::BTreeMap;

use rust_decimal::Decimal;

use crate::{
    http::{Orderbook, OrderbookLevel},
    ws::{OrderbookDataMsg, OrderbookMsg},
};

/// One side of the local order book: price -> size, sorted by price.
type Side = BTreeMap<Decimal, Decimal>;

/// Outcome of [`OrderBookState::apply_snapshot`], [`OrderBookState::apply_diff`]
/// and [`OrderBookState::apply`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplyOutcome {
    /// The diff arrived before any usable snapshot and was stored for later.
    /// The book is not live yet.
    Buffered,
    /// A snapshot was applied and the book is now live and up to date
    /// (any buffered diffs that contiguously followed the snapshot were
    /// applied as well).
    Synced,
    /// A diff was applied to an already-live book.
    Applied,
    /// The snapshot or diff was older than what's already known and was
    /// dropped without changing the book.
    Ignored,
    /// A gap was detected in the update sequence: the local book has been
    /// discarded because it can no longer be trusted. The caller must fetch
    /// a fresh snapshot and feed it back via [`OrderBookState::apply_snapshot`]
    /// (or [`OrderBookState::apply`]).
    ResyncRequired,
}

/// Local order book, reconstructed from a depth snapshot and a stream of
/// diffs.
///
/// All internal bookkeeping (buffered diffs, sync state, book contents) is
/// private. Feed data in with [`apply_snapshot`](Self::apply_snapshot),
/// [`apply_diff`](Self::apply_diff) or [`apply`](Self::apply), in any order,
/// and inspect the book with [`bids`](Self::bids), [`asks`](Self::asks),
/// [`best_bid`](Self::best_bid) and [`best_ask`](Self::best_ask) once
/// [`is_synced`](Self::is_synced) is `true`.
#[derive(Debug, Default)]
pub struct OrderBookState {
    inner: Inner,
}

#[derive(Debug)]
enum Inner {
    /// No usable snapshot yet. Diffs are buffered, keyed by `update_id`, so
    /// they can be drained in order once a snapshot arrives, regardless of
    /// the order they were received in.
    Empty {
        buffered: BTreeMap<i64, OrderbookDataMsg>,
    },
    /// Fully synchronized live book.
    Synced {
        last_id: i64,
        bids: Side,
        asks: Side,
    },
}

impl Default for Inner {
    fn default() -> Self {
        Self::Empty {
            buffered: BTreeMap::new(),
        }
    }
}

impl OrderBookState {
    /// Create an empty state machine with no book yet.
    pub fn new() -> Self {
        Self::default()
    }

    /// Feed a REST depth snapshot ([`Client::get_orderbook`](crate::http::Client::get_orderbook)).
    pub fn apply_snapshot(&mut self, snapshot: Orderbook) -> ApplyOutcome {
        self.set_snapshot(snapshot.update_id, snapshot.bids, snapshot.asks)
    }

    /// Feed a live diff from the `orderbook.{depth}.{symbol}` WebSocket topic.
    ///
    /// If the book is not synced yet, the diff is buffered. Otherwise it
    /// must extend the live book by exactly one `update_id`; anything else
    /// (a stale diff or a gap) is reported via [`ApplyOutcome::Ignored`] /
    /// [`ApplyOutcome::ResyncRequired`].
    pub fn apply_diff(&mut self, diff: OrderbookDataMsg) -> ApplyOutcome {
        match &mut self.inner {
            Inner::Empty { buffered } => {
                buffered.insert(diff.update_id, diff);
                ApplyOutcome::Buffered
            }
            Inner::Synced {
                last_id,
                bids,
                asks,
            } => {
                // `update_id == 1` always means the backend restarted and
                // this data must be treated as a fresh snapshot, which a
                // delta message cannot provide on its own.
                if diff.update_id == 1 {
                    self.inner = Inner::default();
                    return ApplyOutcome::ResyncRequired;
                }
                if diff.update_id <= *last_id {
                    return ApplyOutcome::Ignored;
                }
                if diff.update_id != *last_id + 1 {
                    // Gap in the update sequence: the book can no longer be
                    // trusted. Keep this diff around in case the next
                    // snapshot bridges it.
                    self.inner = Inner::Empty {
                        buffered: BTreeMap::from([(diff.update_id, diff)]),
                    };
                    return ApplyOutcome::ResyncRequired;
                }
                apply_levels(bids, &diff.bids);
                apply_levels(asks, &diff.asks);
                *last_id = diff.update_id;
                ApplyOutcome::Applied
            }
        }
    }

    /// Feed any message from the `orderbook.{depth}.{symbol}` WebSocket
    /// topic, dispatching to [`apply_snapshot`](Self::apply_snapshot) or
    /// [`apply_diff`](Self::apply_diff) as appropriate.
    pub fn apply(&mut self, msg: OrderbookMsg) -> ApplyOutcome {
        match msg {
            OrderbookMsg::Snapshot { data, .. } => {
                self.set_snapshot(data.update_id, data.bids, data.asks)
            }
            OrderbookMsg::Delta { data, .. } => self.apply_diff(data),
        }
    }

    /// `true` once a snapshot has been applied and the book is live.
    pub fn is_synced(&self) -> bool {
        matches!(self.inner, Inner::Synced { .. })
    }

    /// The `update_id` of the last applied snapshot or diff, once synced.
    pub fn last_update_id(&self) -> Option<i64> {
        match &self.inner {
            Inner::Synced { last_id, .. } => Some(*last_id),
            Inner::Empty { .. } => None,
        }
    }

    /// The live bid side (price -> size, ascending by price), once synced.
    pub fn bids(&self) -> Option<&Side> {
        match &self.inner {
            Inner::Synced { bids, .. } => Some(bids),
            Inner::Empty { .. } => None,
        }
    }

    /// The live ask side (price -> size, ascending by price), once synced.
    pub fn asks(&self) -> Option<&Side> {
        match &self.inner {
            Inner::Synced { asks, .. } => Some(asks),
            Inner::Empty { .. } => None,
        }
    }

    /// The highest bid (price, size), once synced.
    pub fn best_bid(&self) -> Option<(Decimal, Decimal)> {
        self.bids()
            .and_then(|bids| bids.iter().next_back())
            .map(|(&price, &size)| (price, size))
    }

    /// The lowest ask (price, size), once synced.
    pub fn best_ask(&self) -> Option<(Decimal, Decimal)> {
        self.asks()
            .and_then(|asks| asks.iter().next())
            .map(|(&price, &size)| (price, size))
    }

    /// Common path for [`apply_snapshot`](Self::apply_snapshot) and the
    /// snapshot branch of [`apply`](Self::apply): replace the book with the
    /// given snapshot and try to bridge any buffered diffs on top of it.
    fn set_snapshot(
        &mut self,
        new_id: i64,
        bids: Vec<OrderbookLevel>,
        asks: Vec<OrderbookLevel>,
    ) -> ApplyOutcome {
        if let Inner::Synced { last_id, .. } = &self.inner {
            // `update_id == 1` signals a backend restart and must always
            // overwrite the local book, even if it is numerically smaller
            // than what we already have.
            if new_id != 1 && new_id <= *last_id {
                return ApplyOutcome::Ignored;
            }
        }

        let mut bids: Side = bids.into_iter().map(|l| (l.price, l.size)).collect();
        let mut asks: Side = asks.into_iter().map(|l| (l.price, l.size)).collect();

        let buffered = match std::mem::take(&mut self.inner) {
            Inner::Empty { buffered } => buffered,
            Inner::Synced { .. } => BTreeMap::new(),
        };

        // Drain buffered diffs that contiguously continue from this
        // snapshot. Anything else (stale, or separated by a gap) is dropped:
        // the snapshot itself is a valid baseline, and the live stream will
        // continue from here.
        let mut last_id = new_id;
        for (&id, diff) in buffered.range((new_id + 1)..) {
            if id != last_id + 1 {
                break;
            }
            apply_levels(&mut bids, &diff.bids);
            apply_levels(&mut asks, &diff.asks);
            last_id = id;
        }

        self.inner = Inner::Synced {
            last_id,
            bids,
            asks,
        };
        ApplyOutcome::Synced
    }
}

/// Apply a list of price levels to one side of the book: a size of zero
/// removes the price level, otherwise the level is inserted/updated.
fn apply_levels(side: &mut Side, levels: &[OrderbookLevel]) {
    for level in levels {
        if level.size.is_zero() {
            side.remove(&level.price);
        } else {
            side.insert(level.price, level.size);
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::dec;

    use crate::{DepthLevel, Topic};

    use super::*;

    fn level(price: Decimal, size: Decimal) -> OrderbookLevel {
        OrderbookLevel { price, size }
    }

    fn snapshot(update_id: i64, bids: Vec<OrderbookLevel>, asks: Vec<OrderbookLevel>) -> Orderbook {
        Orderbook {
            symbol: "BTCUSDT".into(),
            bids,
            asks,
            ts: 0,
            update_id,
            seq: update_id,
            cts: None,
        }
    }

    fn diff(
        update_id: i64,
        bids: Vec<OrderbookLevel>,
        asks: Vec<OrderbookLevel>,
    ) -> OrderbookDataMsg {
        OrderbookDataMsg {
            symbol: "BTCUSDT".into(),
            bids,
            asks,
            update_id,
            seq: update_id,
        }
    }

    #[test]
    fn snapshot_then_contiguous_diff_is_applied() {
        let mut book = OrderBookState::new();
        assert_eq!(
            book.apply_snapshot(snapshot(
                1,
                vec![level(dec!(100), dec!(1))],
                vec![level(dec!(101), dec!(2))]
            )),
            ApplyOutcome::Synced
        );
        assert!(book.is_synced());

        assert_eq!(
            book.apply_diff(diff(2, vec![level(dec!(100), dec!(1.5))], vec![])),
            ApplyOutcome::Applied
        );
        assert_eq!(book.last_update_id(), Some(2));
        assert_eq!(book.best_bid(), Some((dec!(100), dec!(1.5))));
        assert_eq!(book.best_ask(), Some((dec!(101), dec!(2))));
    }

    #[test]
    fn diff_buffered_before_snapshot_bridges_on_arrival() {
        let mut book = OrderBookState::new();
        assert_eq!(
            book.apply_diff(diff(2, vec![level(dec!(100), dec!(1.5))], vec![])),
            ApplyOutcome::Buffered
        );
        assert!(!book.is_synced());

        assert_eq!(
            book.apply_snapshot(snapshot(
                1,
                vec![level(dec!(100), dec!(1))],
                vec![level(dec!(101), dec!(2))]
            )),
            ApplyOutcome::Synced
        );
        assert_eq!(book.last_update_id(), Some(2));
        assert_eq!(book.best_bid(), Some((dec!(100), dec!(1.5))));
    }

    #[test]
    fn stale_snapshot_is_ignored() {
        let mut book = OrderBookState::new();
        book.apply_snapshot(snapshot(5, vec![], vec![]));
        assert_eq!(
            book.apply_snapshot(snapshot(3, vec![], vec![])),
            ApplyOutcome::Ignored
        );
        assert_eq!(book.last_update_id(), Some(5));
    }

    #[test]
    fn stale_diff_is_ignored() {
        let mut book = OrderBookState::new();
        book.apply_snapshot(snapshot(5, vec![], vec![]));
        assert_eq!(
            book.apply_diff(diff(4, vec![], vec![])),
            ApplyOutcome::Ignored
        );
        assert_eq!(book.last_update_id(), Some(5));
    }

    #[test]
    fn gap_in_diff_stream_requires_resync_and_a_fresh_snapshot_bridges_it() {
        let mut book = OrderBookState::new();
        book.apply_snapshot(snapshot(1, vec![], vec![]));

        assert_eq!(
            book.apply_diff(diff(5, vec![level(dec!(100), dec!(1))], vec![])),
            ApplyOutcome::ResyncRequired
        );
        assert!(!book.is_synced());

        // A fresh snapshot at update_id 4 bridges the buffered diff at 5.
        assert_eq!(
            book.apply_snapshot(snapshot(4, vec![], vec![])),
            ApplyOutcome::Synced
        );
        assert_eq!(book.last_update_id(), Some(5));
        assert_eq!(book.best_bid(), Some((dec!(100), dec!(1))));
    }

    #[test]
    fn restart_signal_resets_the_book_even_if_id_is_smaller() {
        let mut book = OrderBookState::new();
        book.apply_snapshot(snapshot(50, vec![level(dec!(100), dec!(1))], vec![]));
        assert!(book.is_synced());

        assert_eq!(
            book.apply_snapshot(snapshot(1, vec![level(dec!(200), dec!(9))], vec![])),
            ApplyOutcome::Synced
        );
        assert_eq!(book.last_update_id(), Some(1));
        assert_eq!(book.best_bid(), Some((dec!(200), dec!(9))));
    }

    #[test]
    fn zero_size_level_removes_the_price_from_the_book() {
        let mut book = OrderBookState::new();
        book.apply_snapshot(snapshot(
            1,
            vec![level(dec!(100), dec!(1)), level(dec!(99), dec!(2))],
            vec![],
        ));

        assert_eq!(
            book.apply_diff(diff(2, vec![level(dec!(100), dec!(0))], vec![])),
            ApplyOutcome::Applied
        );
        assert_eq!(book.best_bid(), Some((dec!(99), dec!(2))));
    }

    #[test]
    fn apply_dispatches_snapshot_and_delta_messages() {
        let mut book = OrderBookState::new();
        let topic = Topic::Orderbook {
            symbol: "BTCUSDT".into(),
            depth: DepthLevel::Level50,
        };

        let snapshot_msg = OrderbookMsg::Snapshot {
            topic: topic.clone(),
            ts: 0,
            data: diff(
                1,
                vec![level(dec!(100), dec!(1))],
                vec![level(dec!(101), dec!(2))],
            ),
            cts: 0,
        };
        assert_eq!(book.apply(snapshot_msg), ApplyOutcome::Synced);

        let delta_msg = OrderbookMsg::Delta {
            topic,
            ts: 0,
            data: diff(2, vec![level(dec!(100), dec!(1.5))], vec![]),
            cts: 0,
        };
        assert_eq!(book.apply(delta_msg), ApplyOutcome::Applied);
        assert_eq!(book.best_bid(), Some((dec!(100), dec!(1.5))));
    }
}

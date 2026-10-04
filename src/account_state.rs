//! Local copy of a user's account: open orders, positions and wallet balances.
//!
//! Every SDK user who trades needs the same bookkeeping: load REST snapshots,
//! then keep them current with the private `order`, `position` and `wallet`
//! WebSocket topics, without letting a late or out-of-order message overwrite
//! newer data. [`AccountState`] does exactly that.
//!
//! # Typical flow
//! 1. Connect the private stream, authenticate and subscribe to `order`,
//!    `position` and `wallet`. Feed every [`TopicMessage`] into
//!    [`AccountState::apply`] from now on.
//! 2. Load the REST snapshots (`get_open_closed_orders_all`,
//!    `get_position_info_all`, `get_wallet_balance`) and pass them to the
//!    `set_*` methods together with the response `time`.
//! 3. After every reconnect repeat step 2: messages missed while
//!    disconnected are only recovered by a fresh snapshot.
//!
//! Steps 1 and 2 may overlap: stream messages and snapshots are merged by
//! `updated_time`, so the newer version of an order or position wins
//! regardless of arrival order.
//!
//! Executions (`execution`, `execution.fast`) are a feed of events, not state,
//! and are ignored by [`AccountState::apply`].

use std::collections::{HashMap, HashSet, VecDeque};

use crate::{
    AccountType, Category, PositionIdx, Timestamp,
    http::{Order, Position, WalletBalance},
    ws::{OrderMsg, PositionMsg, TopicMessage, WalletMsg},
};

/// Maximum number of closed orders remembered to reject late stream updates
/// and stale snapshot entries for them.
pub const MAX_CLOSED_ORDERS: usize = 10_000;

/// Identifies an order. Order ids are unique only within a category.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderKey {
    pub category: Category,
    pub order_id: String,
}

/// Identifies a position: one per symbol in one-way mode, two (buy and sell)
/// in hedge mode.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PositionKey {
    pub category: Category,
    pub symbol: String,
    pub position_idx: PositionIdx,
}

/// What [`AccountState::apply`] and friends changed. Updates that were older than
/// the stored data are ignored and not reported.
#[derive(Debug)]
pub enum Change {
    /// A new open order appeared.
    OrderOpened(OrderKey),
    /// An open order changed (e.g. partially filled or amended).
    OrderUpdated(OrderKey),
    /// An order reached a final status and was removed. `order` is its final
    /// state.
    OrderClosed { key: OrderKey, order: Box<Order> },
    /// A position was created or changed (including closing to size 0).
    PositionUpdated(PositionKey),
    /// The balance of an account type was replaced.
    WalletUpdated(AccountType),
}

/// Open orders, positions and wallet balances of one account.
///
/// Plain data (`Send + Sync`): it can live in any task. It does no I/O.
#[derive(Debug, Default)]
pub struct AccountState {
    wallets: HashMap<AccountType, WalletBalance>,
    orders: HashMap<OrderKey, Order>,
    closed_orders: ClosedOrders,
    positions: HashMap<PositionKey, Position>,
}

impl AccountState {
    pub fn new() -> Self {
        Self::default()
    }

    // -- Snapshots ----------------------------------------------------------

    /// Replace the balance of `balance.account_type` with a REST snapshot.
    pub fn set_wallet(&mut self, balance: WalletBalance) {
        self.wallets.insert(balance.account_type, balance);
    }

    /// Merge a REST snapshot of the open orders of `category`.
    ///
    /// `snapshot_time` is the server time of the response
    /// ([`Response::time`](crate::http::Response::time)), milliseconds.
    /// - an order from the snapshot replaces the stored one unless the stored
    ///   one is newer, and is skipped if it was already closed by the stream;
    /// - a stored order missing from the snapshot is removed, unless it was
    ///   updated after `snapshot_time` (it was opened after the snapshot).
    pub fn set_orders(&mut self, category: Category, orders: Vec<Order>, snapshot_time: Timestamp) {
        let mut in_snapshot = HashSet::with_capacity(orders.len());
        for order in orders {
            if !order.is_open_status() {
                continue;
            }
            let key = OrderKey {
                category,
                order_id: order.order_id.clone(),
            };
            in_snapshot.insert(key.clone());
            if self.closed_orders.closed_since(&key, order.updated_time) {
                continue;
            }
            match self.orders.get(&key) {
                Some(stored) if stored.updated_time > order.updated_time => {}
                _ => {
                    self.orders.insert(key, order);
                }
            }
        }

        self.orders.retain(|key, order| {
            key.category != category
                || in_snapshot.contains(key)
                || order.updated_time > snapshot_time
        });
    }

    /// Merge a REST snapshot of the positions of `category`.
    ///
    /// A snapshot position replaces the stored one unless the stored one is
    /// newer. Stored positions missing from the snapshot are kept only if
    /// they were updated after `snapshot_time`.
    pub fn set_positions(
        &mut self,
        category: Category,
        positions: Vec<Position>,
        snapshot_time: Timestamp,
    ) {
        let mut in_snapshot = HashSet::with_capacity(positions.len());
        for position in positions {
            let key = PositionKey {
                category,
                symbol: position.symbol.clone(),
                position_idx: position.position_idx,
            };
            in_snapshot.insert(key.clone());
            match self.positions.get(&key) {
                Some(stored) if is_newer_position(stored, &position) => {}
                _ => {
                    self.positions.insert(key, position);
                }
            }
        }

        self.positions.retain(|key, position| {
            key.category != category
                || in_snapshot.contains(key)
                || position.updated_time > snapshot_time
        });
    }

    // -- Stream -------------------------------------------------------------

    /// Apply one private stream message. Topics other than `order`,
    /// `position` and `wallet` are ignored.
    pub fn apply(&mut self, msg: TopicMessage) -> Vec<Change> {
        match msg {
            TopicMessage::Order(msg) => msg
                .data
                .into_iter()
                .filter_map(|order| self.apply_order(order))
                .collect(),
            TopicMessage::Position(msg) => msg
                .data
                .into_iter()
                .filter_map(|position| self.apply_position(position))
                .collect(),
            TopicMessage::Wallet(msg) => msg
                .data
                .into_iter()
                .map(|wallet| self.apply_wallet(wallet))
                .collect(),
            TopicMessage::Execution(_)
            | TopicMessage::FastExecution(_)
            | TopicMessage::Greeks(_)
            | TopicMessage::Dcp(_) => Vec::new(),
        }
    }

    /// Apply one `order` stream update. Returns `None` if it was stale.
    pub fn apply_order(&mut self, msg: OrderMsg) -> Option<Change> {
        let key = OrderKey {
            category: msg.category,
            order_id: msg.order_id.clone(),
        };
        if self.closed_orders.closed_since(&key, msg.updated_time) {
            return None;
        }

        let open = msg.order_status.is_open();
        match self.orders.get_mut(&key) {
            Some(stored) if stored.updated_time > msg.updated_time => None,
            Some(stored) if open => {
                stored.update(msg);
                Some(Change::OrderUpdated(key))
            }
            Some(_) => {
                let mut order = self.orders.remove(&key)?;
                let updated_time = msg.updated_time;
                order.update(msg);
                self.closed_orders.insert(key.clone(), updated_time);
                Some(Change::OrderClosed {
                    key,
                    order: Box::new(order),
                })
            }
            None if open => {
                self.orders.insert(key.clone(), msg.into());
                Some(Change::OrderOpened(key))
            }
            None => {
                // Closed before we ever saw it open (e.g. an IOC order).
                self.closed_orders.insert(key.clone(), msg.updated_time);
                Some(Change::OrderClosed {
                    key,
                    order: Box::new(msg.into()),
                })
            }
        }
    }

    /// Apply one `position` stream update. Returns `None` if it was stale.
    pub fn apply_position(&mut self, msg: PositionMsg) -> Option<Change> {
        let key = PositionKey {
            category: msg.category,
            symbol: msg.symbol.clone(),
            position_idx: msg.position_idx,
        };
        let position = Position::from(msg);
        if let Some(stored) = self.positions.get(&key)
            && is_newer_position(stored, &position)
        {
            return None;
        }
        self.positions.insert(key.clone(), position);
        Some(Change::PositionUpdated(key))
    }

    /// Apply one `wallet` stream update (always a full balance).
    pub fn apply_wallet(&mut self, msg: WalletMsg) -> Change {
        let account_type = msg.account_type;
        self.wallets.insert(account_type, msg.into());
        Change::WalletUpdated(account_type)
    }

    // -- Queries ------------------------------------------------------------

    pub fn wallet(&self, account_type: AccountType) -> Option<&WalletBalance> {
        self.wallets.get(&account_type)
    }

    pub fn wallets(&self) -> impl Iterator<Item = &WalletBalance> {
        self.wallets.values()
    }

    pub fn order(&self, category: Category, order_id: &str) -> Option<&Order> {
        self.orders.get(&OrderKey {
            category,
            order_id: order_id.to_owned(),
        })
    }

    pub fn order_by_link_id(&self, category: Category, order_link_id: &str) -> Option<&Order> {
        self.orders.iter().find_map(|(key, order)| {
            (key.category == category && order.order_link_id.as_deref() == Some(order_link_id))
                .then_some(order)
        })
    }

    /// All open orders.
    pub fn orders(&self) -> impl Iterator<Item = (&OrderKey, &Order)> {
        self.orders.iter()
    }

    /// Open orders of one symbol.
    pub fn orders_for<'a>(
        &'a self,
        category: Category,
        symbol: &'a str,
    ) -> impl Iterator<Item = &'a Order> {
        self.orders
            .iter()
            .filter(move |(key, order)| key.category == category && order.symbol == symbol)
            .map(|(_, order)| order)
    }

    pub fn position(
        &self,
        category: Category,
        symbol: &str,
        position_idx: PositionIdx,
    ) -> Option<&Position> {
        self.positions.get(&PositionKey {
            category,
            symbol: symbol.to_owned(),
            position_idx,
        })
    }

    /// All known positions, including closed ones (size 0), which still carry
    /// the leverage and risk limit settings.
    pub fn positions(&self) -> impl Iterator<Item = (&PositionKey, &Position)> {
        self.positions.iter()
    }

    /// Positions with a non-zero size.
    pub fn open_positions(&self) -> impl Iterator<Item = (&PositionKey, &Position)> {
        self.positions
            .iter()
            .filter(|(_, position)| !position.size.is_zero())
    }
}

/// `true` if `stored` is newer than `incoming` and must be kept.
///
/// `updated_time` decides; on a tie the trade sequence `seq` does, when both
/// are known (Bybit sends `-1` for updates not caused by a trade).
fn is_newer_position(stored: &Position, incoming: &Position) -> bool {
    if stored.updated_time != incoming.updated_time {
        return stored.updated_time > incoming.updated_time;
    }
    stored.seq >= 0 && incoming.seq >= 0 && stored.seq > incoming.seq
}

/// Bounded memory of recently closed orders: key -> `updated_time` of the
/// closing update. The oldest entries are forgotten first.
#[derive(Debug, Default)]
struct ClosedOrders {
    by_key: HashMap<OrderKey, Timestamp>,
    order: VecDeque<OrderKey>,
}

impl ClosedOrders {
    fn insert(&mut self, key: OrderKey, updated_time: Timestamp) {
        if self.by_key.insert(key.clone(), updated_time).is_none() {
            self.order.push_back(key);
        }
        while self.order.len() > MAX_CLOSED_ORDERS {
            if let Some(oldest) = self.order.pop_front() {
                self.by_key.remove(&oldest);
            }
        }
    }

    /// `true` if the order was closed by an update at least as new as
    /// `updated_time`, i.e. data with that timestamp is stale.
    ///
    /// A conditional order that is triggered (a final status) and then
    /// continues as a regular order under the same id gets a newer
    /// `updated_time`, so it is not blocked.
    fn closed_since(&self, key: &OrderKey, updated_time: Timestamp) -> bool {
        self.by_key
            .get(key)
            .is_some_and(|&closed_at| closed_at >= updated_time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{OrderStatus, ws::IncomingMessage};
    use rust_decimal::Decimal;

    const LINEAR: Category = Category::Linear;

    fn order_msg(order_id: &str, status: &str, updated_time: Timestamp) -> OrderMsg {
        let json = format!(
            r#"{{
                "category": "linear", "symbol": "BTCUSDT", "orderId": "{order_id}",
                "orderLinkId": "link-{order_id}", "side": "Buy", "orderType": "Limit",
                "cancelType": "UNKNOWN", "price": "100", "qty": "2", "orderIv": "",
                "timeInForce": "GTC", "orderStatus": "{status}", "lastPriceOnCreated": "",
                "reduceOnly": false, "leavesQty": "", "leavesValue": "", "cumExecQty": "0",
                "cumExecValue": "0", "avgPrice": "", "blockTradeId": "", "positionIdx": 0,
                "cumExecFee": "0", "closedPnl": "0", "createdTime": "1000",
                "updatedTime": "{updated_time}", "rejectReason": "EC_NoError",
                "stopOrderType": "", "tpslMode": "", "triggerPrice": "", "takeProfit": "",
                "stopLoss": "", "tpTriggerBy": "", "slTriggerBy": "", "tpLimitPrice": "",
                "slLimitPrice": "", "triggerDirection": 0, "triggerBy": "",
                "closeOnTrigger": false, "placeType": "", "smpType": "None", "smpGroup": 0,
                "smpOrderId": "", "feeCurrency": ""
            }}"#
        );
        serde_json::from_str(&json).unwrap()
    }

    fn order(order_id: &str, status: &str, updated_time: Timestamp) -> Order {
        order_msg(order_id, status, updated_time).into()
    }

    fn position_msg(idx: u8, size: &str, updated_time: Timestamp, seq: i64) -> PositionMsg {
        let json = format!(
            r#"{{
                "category": "linear", "symbol": "BTCUSDT", "positionIdx": {idx},
                "riskId": 1, "riskLimitValue": "200000", "side": "Buy", "size": "{size}",
                "entryPrice": "100", "sessionAvgPrice": "", "leverage": "10",
                "positionValue": "100", "markPrice": "100", "positionIM": "10",
                "positionMM": "1", "positionIMByMp": "10", "positionMMByMp": "1",
                "takeProfit": "0", "stopLoss": "0", "trailingStop": "0",
                "unrealisedPnl": "5", "cumRealisedPnl": "0", "curRealisedPnl": "0",
                "createdTime": "1000", "updatedTime": "{updated_time}", "liqPrice": "",
                "positionStatus": "Normal", "adlRankIndicator": 0, "autoAddMargin": 0,
                "leverageSysUpdatedTime": "", "mmrSysUpdatedTime": "", "seq": {seq},
                "isReduceOnly": false
            }}"#
        );
        serde_json::from_str(&json).unwrap()
    }

    fn topic_message(json: &str) -> TopicMessage {
        match IncomingMessage::from_json(json).unwrap() {
            IncomingMessage::Topic(msg) => msg,
            other => panic!("not a private topic message: {other:?}"),
        }
    }

    #[test]
    fn account_state_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<AccountState>();
    }

    #[test]
    fn order_lifecycle_open_update_close() {
        let mut state = AccountState::new();

        let opened = state.apply_order(order_msg("1", "New", 1));
        let updated = state.apply_order(order_msg("1", "PartiallyFilled", 2));
        let closed = state.apply_order(order_msg("1", "Filled", 3));

        assert!(matches!(opened, Some(Change::OrderOpened(_))));
        assert!(matches!(updated, Some(Change::OrderUpdated(_))));
        assert!(matches!(
            closed,
            Some(Change::OrderClosed { ref order, .. }) if order.order_status == OrderStatus::Filled
        ));
        assert_eq!(state.orders().count(), 0);
    }

    #[test]
    fn open_order_is_queryable() {
        let mut state = AccountState::new();
        state.apply_order(order_msg("1", "New", 1));

        assert!(state.order(LINEAR, "1").is_some());
        assert!(state.order(Category::Spot, "1").is_none());
        assert!(state.order_by_link_id(LINEAR, "link-1").is_some());
        assert_eq!(state.orders_for(LINEAR, "BTCUSDT").count(), 1);
        // leavesQty is derived when the stream omits it.
        assert_eq!(
            state.order(LINEAR, "1").unwrap().leaves_qty,
            Decimal::from(2)
        );
    }

    #[test]
    fn stale_order_update_is_ignored() {
        let mut state = AccountState::new();
        state.apply_order(order_msg("1", "PartiallyFilled", 5));

        let change = state.apply_order(order_msg("1", "New", 4));

        assert!(change.is_none());
        assert_eq!(
            state.order(LINEAR, "1").unwrap().order_status,
            OrderStatus::PartiallyFilled
        );
    }

    #[test]
    fn late_update_after_close_does_not_reopen_the_order() {
        let mut state = AccountState::new();
        state.apply_order(order_msg("1", "New", 1));
        state.apply_order(order_msg("1", "Filled", 3));

        let change = state.apply_order(order_msg("1", "PartiallyFilled", 2));

        assert!(change.is_none());
        assert!(state.order(LINEAR, "1").is_none());
    }

    #[test]
    fn order_closed_before_being_seen_open_is_reported_but_not_stored() {
        let mut state = AccountState::new();

        let change = state.apply_order(order_msg("1", "Cancelled", 1));

        assert!(matches!(change, Some(Change::OrderClosed { .. })));
        assert!(state.order(LINEAR, "1").is_none());
    }

    #[test]
    fn triggered_conditional_order_continues_under_the_same_id() {
        let mut state = AccountState::new();
        state.apply_order(order_msg("1", "Untriggered", 1));
        state.apply_order(order_msg("1", "Triggered", 2));

        let change = state.apply_order(order_msg("1", "New", 3));

        assert!(matches!(change, Some(Change::OrderOpened(_))));
        assert!(state.order(LINEAR, "1").is_some());
    }

    #[test]
    fn snapshot_does_not_resurrect_an_order_closed_by_the_stream() {
        let mut state = AccountState::new();
        state.apply_order(order_msg("1", "Filled", 5));

        // The snapshot was taken before the fill.
        state.set_orders(LINEAR, vec![order("1", "New", 4)], 4);

        assert!(state.order(LINEAR, "1").is_none());
    }

    #[test]
    fn snapshot_does_not_overwrite_newer_stream_data() {
        let mut state = AccountState::new();
        state.apply_order(order_msg("1", "PartiallyFilled", 5));

        state.set_orders(LINEAR, vec![order("1", "New", 4)], 4);

        assert_eq!(
            state.order(LINEAR, "1").unwrap().order_status,
            OrderStatus::PartiallyFilled
        );
    }

    #[test]
    fn snapshot_removes_missing_orders_but_keeps_orders_opened_after_it() {
        let mut state = AccountState::new();
        state.apply_order(order_msg("old", "New", 1));
        state.apply_order(order_msg("new", "New", 20));
        state.apply_order(order_msg("spot-is-untouched", "New", 1));

        state.set_orders(LINEAR, vec![order("snap", "New", 5)], 10);

        assert!(state.order(LINEAR, "old").is_none());
        assert!(state.order(LINEAR, "new").is_some());
        assert!(state.order(LINEAR, "snap").is_some());
    }

    #[test]
    fn snapshot_skips_closed_orders() {
        let mut state = AccountState::new();

        state.set_orders(LINEAR, vec![order("1", "Filled", 5)], 10);

        assert_eq!(state.orders().count(), 0);
    }

    #[test]
    fn hedge_mode_positions_are_separate() {
        let mut state = AccountState::new();

        state.apply_position(position_msg(1, "1", 1, 1));
        state.apply_position(position_msg(2, "3", 1, 2));

        let buy = state.position(LINEAR, "BTCUSDT", PositionIdx::Buy).unwrap();
        let sell = state
            .position(LINEAR, "BTCUSDT", PositionIdx::Sell)
            .unwrap();
        assert_eq!(buy.size, Decimal::from(1));
        assert_eq!(sell.size, Decimal::from(3));
        // Per-position margin and PnL come from the position message itself.
        assert_eq!(buy.position_im, Some(Decimal::from(10)));
        assert_eq!(buy.position_mm, Some(Decimal::from(1)));
        assert_eq!(buy.unrealised_pnl, Some(Decimal::from(5)));
    }

    #[test]
    fn stale_position_update_is_ignored() {
        let mut state = AccountState::new();
        state.apply_position(position_msg(0, "2", 5, 10));

        assert!(state.apply_position(position_msg(0, "1", 4, 11)).is_none());
        // Same time: the trade sequence decides.
        assert!(state.apply_position(position_msg(0, "1", 5, 9)).is_none());
        assert!(state.apply_position(position_msg(0, "3", 5, 12)).is_some());

        let position = state
            .position(LINEAR, "BTCUSDT", PositionIdx::OneWay)
            .unwrap();
        assert_eq!(position.size, Decimal::from(3));
    }

    #[test]
    fn closed_position_is_kept_but_not_open() {
        let mut state = AccountState::new();
        state.apply_position(position_msg(0, "2", 1, 1));

        state.apply_position(position_msg(0, "0", 2, 2));

        assert_eq!(state.positions().count(), 1);
        assert_eq!(state.open_positions().count(), 0);
    }

    #[test]
    fn position_snapshot_merges_by_updated_time() {
        let mut state = AccountState::new();
        state.apply_position(position_msg(1, "5", 20, 1));
        state.apply_position(position_msg(2, "5", 1, 1));

        state.set_positions(
            LINEAR,
            vec![Position::from(position_msg(1, "1", 10, 1))],
            10,
        );

        // Buy side: the stream version is newer than the snapshot entry.
        let buy = state.position(LINEAR, "BTCUSDT", PositionIdx::Buy).unwrap();
        assert_eq!(buy.size, Decimal::from(5));
        // Sell side: missing from the snapshot and older than it.
        assert!(
            state
                .position(LINEAR, "BTCUSDT", PositionIdx::Sell)
                .is_none()
        );
    }

    #[test]
    fn apply_dispatches_order_stream_messages() {
        let json = r#"{
            "id": "5923240c6880ab-c59f-420b-9adb-3639adc9dd90",
            "topic": "order",
            "creationTime": 1672364262474,
            "data": [
                {
                    "symbol": "ETH-30DEC22-1400-C",
                    "orderId": "5cf98598-39a7-459e-97bf-76ca765ee020",
                    "side": "Sell",
                    "orderType": "Market",
                    "cancelType": "UNKNOWN",
                    "price": "72.5",
                    "qty": "1",
                    "orderIv": "",
                    "timeInForce": "IOC",
                    "orderStatus": "Filled",
                    "orderLinkId": "",
                    "lastPriceOnCreated": "",
                    "reduceOnly": false,
                    "leavesQty": "",
                    "leavesValue": "",
                    "cumExecQty": "1",
                    "cumExecValue": "75",
                    "avgPrice": "75",
                    "blockTradeId": "",
                    "positionIdx": 0,
                    "cumExecFee": "0.358635",
                    "closedPnl": "0",
                    "createdTime": "1672364262444",
                    "updatedTime": "1672364262457",
                    "rejectReason": "EC_NoError",
                    "stopOrderType": "",
                    "tpslMode": "",
                    "triggerPrice": "",
                    "takeProfit": "",
                    "stopLoss": "",
                    "tpTriggerBy": "",
                    "slTriggerBy": "",
                    "tpLimitPrice": "",
                    "slLimitPrice": "",
                    "triggerDirection": 0,
                    "triggerBy": "",
                    "closeOnTrigger": false,
                    "category": "option",
                    "placeType": "price",
                    "smpType": "None",
                    "smpGroup": 0,
                    "smpOrderId": "",
                    "feeCurrency": "",
                    "cumFeeDetail": {
                        "MNT": "0.00242968"
                    }
                }
            ]
        }"#;
        let mut state = AccountState::new();

        let changes = state.apply(topic_message(json));

        // An IOC option order filled at once: reported as closed, not stored.
        assert!(matches!(
            changes.as_slice(),
            [Change::OrderClosed { key, .. }] if key.category == Category::Option
        ));
        assert_eq!(state.orders().count(), 0);
    }

    #[test]
    fn apply_dispatches_wallet_stream_messages() {
        let json = r#"{
            "id": "592324d2bce751-ad38-48eb-8f42-4671d1fb4d4e",
            "topic": "wallet",
            "creationTime": 1700034722104,
            "data": [
                {
                    "accountIMRate": "0",
                    "accountIMRateByMp": "0",
                    "accountMMRate": "0",
                    "accountMMRateByMp": "0",
                    "totalEquity": "10262.91335023",
                    "totalWalletBalance": "9684.46297164",
                    "totalMarginBalance": "9684.46297164",
                    "totalAvailableBalance": "9556.6056555",
                    "totalPerpUPL": "0",
                    "totalInitialMargin": "0",
                    "totalInitialMarginByMp": "0",
                    "totalMaintenanceMargin": "0",
                    "totalMaintenanceMarginByMp": "0",
                    "coin": [
                        {
                            "coin": "BTC",
                            "equity": "0.00102964",
                            "usdValue": "36.70759517",
                            "walletBalance": "0.00102964",
                            "availableToWithdraw": "0.00102964",
                            "availableToBorrow": "",
                            "borrowAmount": "0",
                            "accruedInterest": "0",
                            "totalOrderIM": "",
                            "totalPositionIM": "",
                            "totalPositionMM": "",
                            "unrealisedPnl": "0",
                            "cumRealisedPnl": "-0.00000973",
                            "bonus": "0",
                            "collateralSwitch": true,
                            "marginCollateral": true,
                            "locked": "0",
                            "spotHedgingQty": "0.01592413",
                            "spotBorrow": "0"
                        }
                    ],
                    "accountLTV": "0",
                    "accountType": "UNIFIED"
                }
            ]
        }"#;
        let mut state = AccountState::new();

        let changes = state.apply(topic_message(json));

        assert!(matches!(
            changes.as_slice(),
            [Change::WalletUpdated(AccountType::UNIFIED)]
        ));
        assert!(state.wallet(AccountType::UNIFIED).is_some());
    }
}

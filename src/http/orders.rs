use rust_decimal::{Decimal, serde::str_option::deserialize as option_decimal};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string as number;

use super::common::List;
use crate::{
    CancelType, CreateType, OcoTriggerBy, OrderStatus, OrderType, PlaceType, PositionIdx,
    RejectReason, Side, SmpType, TimeInForce, Timestamp, TpslMode, TriggerBy, TriggerDirection,
    enums::{Category, StopOrderType},
    serde::{empty_string_as_none, string_to_option_bool},
    ws::OrderMsg,
};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenClosedOrdersParams {
    /// Product type
    /// UTA2.0, UTA1.0: linear, inverse, spot, option
    /// classic account: linear, inverse, spot
    pub category: Category,
    /// Symbol name, like BTCUSDT, uppercase only. For linear, either symbol, baseCoin, settleCoin is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Base coin, uppercase only
    /// Supports linear, inverse & option
    /// option: it returns all option open orders by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    /// Settle coin, uppercase only
    /// linear: either symbol, baseCoin or settleCoin is required
    /// spot: not supported
    /// option: USDT or USDC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_coin: Option<String>,
    /// Order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// User customized order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    /// 0(default): UTA2.0, UTA1.0, classic account query open status orders (e.g., New, PartiallyFilled) only
    /// 1: UTA2.0, UTA1.0(except inverse)
    /// 2: UTA1.0(inverse), classic account
    /// Query a maximum of recent 500 closed status records are kept under each account each category (e.g., Cancelled, Rejected, Filled orders).
    /// If the Bybit service is restarted due to an update, this part of the data will be cleared and accumulated again, but the order records will still be queried in order history
    /// openOnly param will be ignored when query by orderId or orderLinkId
    /// Classic spot: not supported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_only: Option<i32>,
    /// Order: active order,
    /// StopOrder: conditional order for Futures and Spot,
    /// tpslOrder: spot TP/SL order,
    /// OcoOrder: Spot oco order,
    /// BidirectionalTpslOrder: Spot bidirectional TPSL order
    /// - classic account spot: return Order active order by default
    /// - Others: all kinds of orders by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<OrderFilter>,
    /// Limit for data size per page. [1, 50]. Default: 20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetOpenClosedOrdersParams {
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
            settle_coin: None,
            order_id: None,
            order_link_id: None,
            open_only: None,
            order_filter: None,
            limit: None,
            cursor: None,
        }
    }

    pub fn with_symbol(mut self, v: String) -> Self {
        self.symbol = Some(v);
        self
    }
    pub fn with_base_coin(mut self, v: String) -> Self {
        self.base_coin = Some(v);
        self
    }
    pub fn with_settle_coin(mut self, v: String) -> Self {
        self.settle_coin = Some(v);
        self
    }
    pub fn with_order_id(mut self, v: String) -> Self {
        self.order_id = Some(v);
        self
    }
    pub fn with_order_link_id(mut self, v: String) -> Self {
        self.order_link_id = Some(v);
        self
    }
    pub fn with_open_only(mut self, v: i32) -> Self {
        self.open_only = Some(v);
        self
    }
    pub fn with_order_filter(mut self, v: OrderFilter) -> Self {
        self.order_filter = Some(v);
        self
    }
    pub fn with_limit(mut self, v: i32) -> Self {
        self.limit = Some(v);
        self
    }
    pub fn with_cursor(mut self, v: String) -> Self {
        self.cursor = Some(v);
        self
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum OrderFilter {
    /// active order,
    Order,
    /// conditional order for Futures and Spot,
    StopOrder,
    /// spot TP/SL order,
    #[serde(rename = "camelCase")]
    TpslOrder,
    /// Spot oco order,
    OcoOrder,
    /// Spot bidirectional TPSL order
    /// - classic account spot: return Order active order by default
    /// - Others: all kinds of orders by default
    BidirectionalTpslOrder,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    /// Order ID
    pub order_id: String,
    /// User customized order ID
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub order_link_id: Option<String>,
    /// Paradigm block trade ID
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub block_trade_id: Option<String>,
    /// Symbol name
    pub symbol: String,
    /// Order price
    pub price: Decimal,
    /// Order qty
    pub qty: Decimal,
    /// Side. Buy,Sell
    pub side: Side,
    /// Whether to borrow. Unified spot only. 0: false, 1: true. Classic spot is not supported, always 0
    #[serde(default, deserialize_with = "string_to_option_bool")]
    pub is_leverage: Option<bool>,
    /// Position index. Used to identify positions in different position modes.
    pub position_idx: PositionIdx,
    /// Order status
    pub order_status: OrderStatus,
    /// Order create type
    /// Only for category=linear or inverse
    /// Spot, Option do not have this key
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub create_type: Option<CreateType>,
    /// Cancel type
    pub cancel_type: CancelType,
    /// Reject reason. Classic spot is not supported
    pub reject_reason: RejectReason,
    /// Average filled price
    /// UTA: returns "" for those orders without avg price
    /// classic account: returns "0" for those orders without avg price, and also for those orders have partially filled but cancelled at the end
    #[serde(default, deserialize_with = "option_decimal")]
    pub avg_price: Option<Decimal>,
    /// The remaining qty not executed. Classic spot is not supported
    pub leaves_qty: Decimal,
    /// The estimated value not executed. Classic spot is not supported
    pub leaves_value: Decimal,
    /// Cumulative executed order qty
    pub cum_exec_qty: Decimal,
    /// Cumulative executed order value. Classic spot is not supported
    pub cum_exec_value: Decimal,
    /// Cumulative executed trading fee. Classic spot is not supported
    pub cum_exec_fee: Decimal,
    /// Time in force
    pub time_in_force: TimeInForce,
    /// Order type. Market,Limit. For TP/SL order, it means the order type after triggered
    pub order_type: OrderType,
    /// Stop order type
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub stop_order_type: Option<StopOrderType>,
    /// Implied volatility
    #[serde(default, deserialize_with = "option_decimal")]
    pub order_iv: Option<Decimal>,
    /// The unit for qty when create Spot market orders for UTA account. baseCoin, quoteCoin
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub market_unit: Option<String>,
    /// Trigger price. If stopOrderType=TrailingStop, it is activate price. Otherwise, it is trigger price
    #[serde(default, deserialize_with = "option_decimal")]
    pub trigger_price: Option<Decimal>,
    /// Take profit price
    #[serde(default, deserialize_with = "option_decimal")]
    pub take_profit: Option<Decimal>,
    /// Stop loss price
    #[serde(default, deserialize_with = "option_decimal")]
    pub stop_loss: Option<Decimal>,
    /// TP/SL mode, Full: entire position for TP/SL. Partial: partial position tp/sl. Spot does not have this field, and Option returns always ""
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub tpsl_mode: Option<TpslMode>,
    /// The trigger type of Spot OCO order.OcoTriggerByUnknown, OcoTriggerByTp, OcoTriggerByBySl. Classic spot is not supported
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub oco_trigger_by: Option<OcoTriggerBy>,
    /// The limit order price when take profit price is triggered
    #[serde(default, deserialize_with = "option_decimal")]
    pub tp_limit_price: Option<Decimal>,
    /// The limit order price when stop loss price is triggered
    #[serde(default, deserialize_with = "option_decimal")]
    pub sl_limit_price: Option<Decimal>,
    /// The price type to trigger take profit
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub tp_trigger_by: Option<TriggerBy>,
    /// The price type to trigger stop loss
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub sl_trigger_by: Option<TriggerBy>,
    /// Trigger direction. 1: rise, 2: fall
    pub trigger_direction: TriggerDirection,
    /// The price type of trigger price
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub trigger_by: Option<TriggerBy>,
    /// Last price when place the order, Spot is not applicable
    #[serde(default, deserialize_with = "option_decimal")]
    pub last_price_on_created: Option<Decimal>,
    /// Last price when place the order, Spot has this field only
    #[serde(default, deserialize_with = "option_decimal")]
    pub base_price: Option<Decimal>,
    /// Reduce only. true means reduce position size
    pub reduce_only: bool,
    /// Close on trigger. What is a close on trigger order?
    pub close_on_trigger: bool,
    /// Place type, option used. iv, price
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub place_type: Option<PlaceType>,
    /// SMP execution type
    pub smp_type: SmpType,
    /// Smp group ID. If the UID has no group, it is 0 by default
    pub smp_group: i64,
    /// The counterparty's orderID which triggers this SMP execution
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub smp_order_id: Option<String>,
    /// Order created timestamp (ms)
    #[serde(deserialize_with = "number")]
    pub created_time: Timestamp,
    /// Order updated timestamp (ms)
    #[serde(deserialize_with = "number")]
    pub updated_time: Timestamp,
}

impl Order {
    pub fn is_open_status(&self) -> bool {
        self.order_status.is_open()
    }
    pub fn is_closed_status(&self) -> bool {
        self.order_status.is_closed()
    }
    pub fn update(&mut self, msg: OrderMsg) {
        self.order_id = msg.order_id;
        self.order_link_id = msg.order_link_id;
        self.block_trade_id = msg.block_trade_id;
        self.symbol = msg.symbol;
        self.price = msg.price;
        self.qty = msg.qty;
        self.side = msg.side;
        self.is_leverage = msg.is_leverage;
        self.position_idx = msg.position_idx;
        self.order_status = msg.order_status;
        self.create_type = msg.create_type;
        self.cancel_type = msg.cancel_type;
        self.reject_reason = msg.reject_reason;
        self.avg_price = msg.avg_price;
        if let Some(leaves_qty) = msg.leaves_qty {
            self.leaves_qty = leaves_qty;
        }
        if let Some(leaves_value) = msg.leaves_value {
            self.leaves_value = leaves_value;
        }
        self.cum_exec_qty = msg.cum_exec_qty;
        self.cum_exec_value = msg.cum_exec_value;
        self.cum_exec_fee = msg.cum_exec_fee;
        self.time_in_force = msg.time_in_force;
        self.order_type = msg.order_type;
        self.stop_order_type = msg.stop_order_type;
        self.order_iv = msg.order_iv;
        self.market_unit = msg.market_unit;
        self.trigger_price = msg.trigger_price;
        self.take_profit = msg.take_profit;
        self.stop_loss = msg.stop_loss;
        self.tpsl_mode = msg.tpsl_mode;
        self.oco_trigger_by = msg.oco_trigger_by;
        self.tp_limit_price = msg.tp_limit_price;
        self.sl_limit_price = msg.sl_limit_price;
        self.tp_trigger_by = msg.tp_trigger_by;
        self.sl_trigger_by = msg.sl_trigger_by;
        self.trigger_direction = msg.trigger_direction;
        self.trigger_by = msg.trigger_by;
        self.last_price_on_created = msg.last_price_on_created;
        // TODO: self.base_price
        self.reduce_only = msg.reduce_only;
        self.close_on_trigger = msg.close_on_trigger;
        self.place_type = msg.place_type;
        self.smp_type = msg.smp_type;
        self.smp_group = msg.smp_group;
        self.smp_order_id = msg.smp_order_id;
        self.created_time = msg.created_time;
        self.updated_time = msg.updated_time;
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    /// Product type
    /// UTA2.0, UTA1.0: linear, inverse, spot, option
    /// classic account: linear, inverse, spot
    pub category: Category,
    /// Symbol name, like BTCUSDT, uppercase only
    pub symbol: String,
    // Whether to borrow. Unified account Spot trading only.
    /// 0(default): false, spot trading
    /// 1: true, margin trading, make sure you turn on margin trading, and set the relevant currency as collateral
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_leverage: Option<i64>,
    /// Buy, Sell
    pub side: Side,
    /// Market, Limit
    pub order_type: OrderType,
    /// Order quantity
    /// UTA account
    /// Spot: Market Buy order by value by default, you can set marketUnit field to choose order by value or qty for market orders
    /// Perps, Futures & Option: always order by qty
    /// classic account
    /// Spot: Market Buy order by value by default
    /// Perps, Futures: always order by qty
    /// Perps & Futures: if you pass qty="0" and specify reduceOnly=true&closeOnTrigger=true, you can close the position up to maxMktOrderQty or maxOrderQty shown on Get Instruments Info of current symbol
    pub qty: Decimal,
    /// Select the unit for qty when create Spot market orders for UTA account
    /// baseCoin: for example, buy BTCUSDT, then "qty" unit is BTC
    /// quoteCoin: for example, sell BTCUSDT, then "qty" unit is USDT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_unit: Option<String>,
    /// Slippage tolerance Type for market order, TickSize, Percent
    /// take profit, stoploss, conditional orders are not supported
    /// TickSize:
    /// the highest price of Buy order = ask1 + slippageTolerance x tickSize;
    /// the lowest price of Sell order = bid1 - slippageTolerance x tickSize
    /// Percent:
    /// the highest price of Buy order = ask1 x (1 + slippageTolerance x 0.01);
    /// the lowest price of Sell order = bid1 x (1 - slippageTolerance x 0.01)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage_tolerance_type: Option<Decimal>,
    /// Slippage tolerance value
    /// TickSize: range is [1, 10000], integer only
    /// Percent: range is [0.01, 10], up to 2 decimals
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage_tolerance: Option<Decimal>,
    /// Order price
    /// Market order will ignore this field
    /// Please check the min price and price precision from instrument info endpoint
    /// If you have position, price needs to be better than liquidation price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    /// Conditional order param. Used to identify the expected direction of the conditional order.
    /// 1: triggered when market price rises to triggerPrice
    /// 2: triggered when market price falls to triggerPrice
    /// Valid for linear & inverse
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_direction: Option<TriggerDirection>,
    /// If it is not passed, Order by default.
    /// Order
    /// tpslOrder: Spot TP/SL order, the assets are occupied even before the order is triggered
    /// StopOrder: Spot conditional order, the assets will not be occupied until the price of the underlying asset reaches the trigger price, and the required assets will be occupied after the Conditional order is triggered
    /// Valid for spot only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<String>,
    /// For Perps & Futures, it is the conditional order trigger price. If you expect the price to rise to trigger your conditional order, make sure:
    /// triggerPrice > market price
    /// Else, triggerPrice < market price
    /// For spot, it is the TP/SL and Conditional order trigger price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<Decimal>,
    /// Trigger price type, Conditional order param for Perps & Futures.
    /// LastPrice
    /// IndexPrice
    /// MarkPrice
    /// Valid for linear & inverse
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_by: Option<TriggerBy>,
    /// Implied volatility. option only. Pass the real value, e.g for 10%, 0.1 should be passed. orderIv has a higher priority when price is passed as well
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_iv: Option<Decimal>,
    /// Time in force
    /// Market order will always use IOC
    /// If not passed, GTC is used by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    /// Used to identify positions in different position modes. Under hedge-mode, this param is required
    /// 0: one-way mode
    /// 1: hedge-mode Buy side
    /// 2: hedge-mode Sell side
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<PositionIdx>,
    /// User customised order ID. A max of 36 characters. Combinations of numbers, letters (upper and lower cases), dashes, and underscores are supported.
    /// Futures & Perps: orderLinkId rules:
    /// optional param
    /// always unique
    /// option orderLinkId rules:
    /// required param
    /// always unique
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    /// Take profit price
    /// UTA: Spot Limit order supports take profit, stop loss or limit take profit, limit stop loss when creating an order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<Decimal>,
    /// Stop loss price
    /// UTA: Spot Limit order supports take profit, stop loss or limit take profit, limit stop loss when creating an order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<Decimal>,
    /// The price type to trigger take profit. MarkPrice, IndexPrice, default: LastPrice. Valid for linear & inverse
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_by: Option<TriggerBy>,
    /// The price type to trigger stop loss. MarkPrice, IndexPrice, default: LastPrice. Valid for linear & inverse
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_by: Option<TriggerBy>,
    /// What is a reduce-only order? true means your position can only reduce in size if this order is triggered.
    /// You must specify it as true when you are about to close/reduce the position
    /// When reduceOnly is true, take profit/stop loss cannot be set
    /// Valid for linear, inverse & option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    /// What is a close on trigger order? For a closing order. It can only reduce your position, not increase it. If the account has insufficient available balance when the closing order is triggered, then other active orders of similar contracts will be cancelled or reduced. It can be used to ensure your stop loss reduces your position regardless of current available margin.
    /// Valid for linear & inverse
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_on_trigger: Option<bool>,
    /// Smp execution type. What is SMP?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smp_type: Option<SmpType>,
    /// Market maker protection. option only. true means set the order as a market maker protection order. What is mmp?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,
    /// TP/SL mode
    /// Full: entire position for TP/SL. Then, tpOrderType or slOrderType must be Market
    /// Partial: partial position tp/sl (as there is no size option, so it will create tp/sl orders with the qty you actually fill). Limit TP/SL order are supported. Note: When create limit tp/sl, tpslMode is required and it must be Partial
    /// Valid for linear & inverse
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpsl_mode: Option<TpslMode>,
    /// The limit order price when take profit price is triggered
    /// linear & inverse: only works when tpslMode=Partial and tpOrderType=Limit
    /// Spot(UTA): it is required when the order has takeProfit and "tpOrderType"=Limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_limit_price: Option<Decimal>,
    /// The limit order price when stop loss price is triggered
    /// linear & inverse: only works when tpslMode=Partial and slOrderType=Limit
    /// Spot(UTA): it is required when the order has stopLoss and "slOrderType"=Limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_limit_price: Option<Decimal>,
    /// The order type when take profit is triggered
    /// linear & inverse: Market(default), Limit. For tpslMode=Full, it only supports tpOrderType=Market
    /// Spot(UTA):
    /// Market: when you set "takeProfit",
    /// Limit: when you set "takeProfit" and "tpLimitPrice"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_order_type: Option<OrderType>,
    /// The order type when stop loss is triggered
    /// linear & inverse: Market(default), Limit. For tpslMode=Full, it only supports slOrderType=Market
    /// Spot(UTA):
    /// Market: when you set "stopLoss",
    /// Limit: when you set "stopLoss" and "slLimitPrice"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_order_type: Option<OrderType>,
    /// Queue: use the order price on the orderbook in the same direction as the side
    /// Counterparty: use the order price on the orderbook in the opposite direction as the side
    /// Valid for linear & inverse
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbo_side_type: Option<String>,
    /// 1,2,3,4,5 Valid for linear & inverse
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbo_level: Option<String>,
}

impl PlaceOrderRequest {
    pub fn new(
        category: Category,
        symbol: String,
        side: Side,
        order_type: OrderType,
        qty: Decimal,
    ) -> Self {
        Self {
            category,
            symbol,
            is_leverage: None,
            side,
            order_type,
            qty,
            market_unit: None,
            slippage_tolerance_type: None,
            slippage_tolerance: None,
            price: None,
            trigger_direction: None,
            order_filter: None,
            trigger_price: None,
            trigger_by: None,
            order_iv: None,
            time_in_force: None,
            position_idx: None,
            order_link_id: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            reduce_only: None,
            close_on_trigger: None,
            smp_type: None,
            mmp: None,
            tpsl_mode: None,
            tp_limit_price: None,
            sl_limit_price: None,
            tp_order_type: None,
            sl_order_type: None,
            bbo_side_type: None,
            bbo_level: None,
        }
    }

    pub fn with_is_leverage(mut self, v: i64) -> Self {
        self.is_leverage = Some(v);
        self
    }
    pub fn with_market_unit(mut self, v: String) -> Self {
        self.market_unit = Some(v);
        self
    }
    pub fn with_slippage_tolerance_type(mut self, v: Decimal) -> Self {
        self.slippage_tolerance_type = Some(v);
        self
    }
    pub fn with_slippage_tolerance(mut self, v: Decimal) -> Self {
        self.slippage_tolerance = Some(v);
        self
    }
    pub fn with_price(mut self, v: Decimal) -> Self {
        self.price = Some(v);
        self
    }
    pub fn with_trigger_direction(mut self, v: TriggerDirection) -> Self {
        self.trigger_direction = Some(v);
        self
    }
    pub fn with_order_filter(mut self, v: String) -> Self {
        self.order_filter = Some(v);
        self
    }
    pub fn with_trigger_price(mut self, v: Decimal) -> Self {
        self.trigger_price = Some(v);
        self
    }
    pub fn with_trigger_by(mut self, v: TriggerBy) -> Self {
        self.trigger_by = Some(v);
        self
    }
    pub fn with_order_iv(mut self, v: Decimal) -> Self {
        self.order_iv = Some(v);
        self
    }
    pub fn with_time_in_force(mut self, v: TimeInForce) -> Self {
        self.time_in_force = Some(v);
        self
    }
    pub fn with_position_idx(mut self, v: PositionIdx) -> Self {
        self.position_idx = Some(v);
        self
    }
    pub fn with_order_link_id(mut self, v: String) -> Self {
        self.order_link_id = Some(v);
        self
    }
    pub fn with_take_profit(mut self, v: Decimal) -> Self {
        self.take_profit = Some(v);
        self
    }
    pub fn with_stop_loss(mut self, v: Decimal) -> Self {
        self.stop_loss = Some(v);
        self
    }
    pub fn with_tp_trigger_by(mut self, v: TriggerBy) -> Self {
        self.tp_trigger_by = Some(v);
        self
    }
    pub fn with_sl_trigger_by(mut self, v: TriggerBy) -> Self {
        self.sl_trigger_by = Some(v);
        self
    }
    pub fn with_reduce_only(mut self, v: bool) -> Self {
        self.reduce_only = Some(v);
        self
    }
    pub fn with_close_on_trigger(mut self, v: bool) -> Self {
        self.close_on_trigger = Some(v);
        self
    }
    pub fn with_smp_type(mut self, v: SmpType) -> Self {
        self.smp_type = Some(v);
        self
    }
    pub fn with_mmp(mut self, v: bool) -> Self {
        self.mmp = Some(v);
        self
    }
    pub fn with_tpsl_mode(mut self, v: TpslMode) -> Self {
        self.tpsl_mode = Some(v);
        self
    }
    pub fn with_tp_limit_price(mut self, v: Decimal) -> Self {
        self.tp_limit_price = Some(v);
        self
    }
    pub fn with_sl_limit_price(mut self, v: Decimal) -> Self {
        self.sl_limit_price = Some(v);
        self
    }
    pub fn with_tp_order_type(mut self, v: OrderType) -> Self {
        self.tp_order_type = Some(v);
        self
    }
    pub fn with_sl_order_type(mut self, v: OrderType) -> Self {
        self.sl_order_type = Some(v);
        self
    }
    pub fn with_bbo_side_type(mut self, v: String) -> Self {
        self.bbo_side_type = Some(v);
        self
    }
    pub fn with_bbo_level(mut self, v: String) -> Self {
        self.bbo_level = Some(v);
        self
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderResponse {
    /// Order ID
    pub order_id: String,
    /// User customised order ID
    pub order_link_id: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderRequest {
    pub category: Category,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
}

impl AmendOrderRequest {
    pub fn new(category: Category, symbol: String) -> Self {
        Self {
            category,
            symbol,
            order_id: None,
            order_link_id: None,
            qty: None,
            price: None,
        }
    }

    pub fn with_order_id(mut self, v: String) -> Self {
        self.order_id = Some(v);
        self
    }
    pub fn with_order_link_id(mut self, v: String) -> Self {
        self.order_link_id = Some(v);
        self
    }
    pub fn with_qty(mut self, v: Decimal) -> Self {
        self.qty = Some(v);
        self
    }
    pub fn with_price(mut self, v: Decimal) -> Self {
        self.price = Some(v);
        self
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderResponse {
    /// Order ID
    pub order_id: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    /// User customised order ID
    pub order_link_id: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    /// Product type. linear, inverse, spot, option
    pub category: Category,
    /// Symbol name, like BTCUSDT, uppercase only
    pub symbol: String,
    /// Order ID. Either orderId or orderLinkId is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// User customised order ID. Either orderId or orderLinkId is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    /// Spot trading only
    /// Order, tpslOrder, StopOrder
    /// If not passed, Order by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<OrderFilter>,
}

impl CancelOrderRequest {
    pub fn new(category: Category, symbol: String) -> Self {
        Self {
            category,
            symbol,
            order_id: None,
            order_link_id: None,
            order_filter: None,
        }
    }

    pub fn with_order_id(mut self, v: String) -> Self {
        self.order_id = Some(v);
        self
    }
    pub fn with_order_link_id(mut self, v: String) -> Self {
        self.order_link_id = Some(v);
        self
    }
    pub fn with_order_filter(mut self, v: OrderFilter) -> Self {
        self.order_filter = Some(v);
        self
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    /// Order ID
    pub order_id: String,
    /// User customised order ID
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub order_link_id: Option<String>,
}

// ── Cancel All Orders ────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersRequest {
    pub category: Category,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<OrderFilter>,
    /// For futures only: TakeProfit, StopLoss, TrailingStop
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_order_type: Option<StopOrderType>,
}

impl CancelAllOrdersRequest {
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
            settle_coin: None,
            order_filter: None,
            stop_order_type: None,
        }
    }

    pub fn with_symbol(mut self, v: String) -> Self {
        self.symbol = Some(v);
        self
    }
    pub fn with_base_coin(mut self, v: String) -> Self {
        self.base_coin = Some(v);
        self
    }
    pub fn with_settle_coin(mut self, v: String) -> Self {
        self.settle_coin = Some(v);
        self
    }
    pub fn with_order_filter(mut self, v: OrderFilter) -> Self {
        self.order_filter = Some(v);
        self
    }
    pub fn with_stop_order_type(mut self, v: StopOrderType) -> Self {
        self.stop_order_type = Some(v);
        self
    }
}

/// Response for cancel-all: a flat list of cancelled order IDs.
pub type CancelAllOrdersResponse = List<CancelOrderResponse>;

// ── Order History ────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderHistoryParams {
    pub category: Category,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<OrderFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_status: Option<OrderStatus>,
    /// Start timestamp (ms)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Timestamp>,
    /// End timestamp (ms)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Timestamp>,
    /// [1, 50]. Default: 20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetOrderHistoryParams {
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
            settle_coin: None,
            order_id: None,
            order_link_id: None,
            order_filter: None,
            order_status: None,
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
    }

    pub fn with_symbol(mut self, v: String) -> Self {
        self.symbol = Some(v);
        self
    }
    pub fn with_base_coin(mut self, v: String) -> Self {
        self.base_coin = Some(v);
        self
    }
    pub fn with_settle_coin(mut self, v: String) -> Self {
        self.settle_coin = Some(v);
        self
    }
    pub fn with_order_id(mut self, v: String) -> Self {
        self.order_id = Some(v);
        self
    }
    pub fn with_order_link_id(mut self, v: String) -> Self {
        self.order_link_id = Some(v);
        self
    }
    pub fn with_order_filter(mut self, v: OrderFilter) -> Self {
        self.order_filter = Some(v);
        self
    }
    pub fn with_order_status(mut self, v: OrderStatus) -> Self {
        self.order_status = Some(v);
        self
    }
    pub fn with_start_time(mut self, v: Timestamp) -> Self {
        self.start_time = Some(v);
        self
    }
    pub fn with_end_time(mut self, v: Timestamp) -> Self {
        self.end_time = Some(v);
        self
    }
    pub fn with_limit(mut self, v: i32) -> Self {
        self.limit = Some(v);
        self
    }
    pub fn with_cursor(mut self, v: String) -> Self {
        self.cursor = Some(v);
        self
    }
}

// ── Batch Orders ─────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderBatchRequest {
    pub category: Category,
    pub request: Vec<PlaceOrderRequest>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderBatchRequest {
    pub category: Category,
    pub request: Vec<AmendOrderRequest>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderBatchRequest {
    pub category: Category,
    pub request: Vec<CancelOrderRequest>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderBatchResult {
    pub category: Category,
    pub symbol: String,
    pub order_id: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub order_link_id: Option<String>,
    /// Order creation timestamp (ms). Note: Bybit uses the non-standard key "createAt".
    #[serde(rename = "createAt", default)]
    pub create_at: Option<Timestamp>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderBatchResult {
    pub category: Category,
    pub symbol: String,
    pub order_id: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub order_link_id: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderBatchResult {
    pub category: Category,
    pub symbol: String,
    pub order_id: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub order_link_id: Option<String>,
}

// --- Spot Borrow Check ---

/// Query params for [`Client::get_spot_borrow_check`](crate::http::Client::get_spot_borrow_check).
///
/// Covers: spot only
#[derive(Debug, Serialize, Clone)]
pub struct GetSpotBorrowCheckParams {
    pub category: Category,
    pub symbol: String,
    pub side: Side,
}

/// Response for [`Client::get_spot_borrow_check`](crate::http::Client::get_spot_borrow_check).
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SpotBorrowCheck {
    pub symbol: String,
    pub side: Side,
    /// Maximum tradeable quantity with leverage (in base coin).
    pub max_trade_qty: Decimal,
    /// Maximum tradeable amount with leverage (in quote coin).
    pub max_trade_amount: Decimal,
    /// Maximum tradeable quantity without leverage (in base coin).
    pub spot_max_trade_qty: Decimal,
    /// Maximum tradeable amount without leverage (in quote coin).
    pub spot_max_trade_amount: Decimal,
    /// The coin to borrow.
    pub borrow_coin: String,
}

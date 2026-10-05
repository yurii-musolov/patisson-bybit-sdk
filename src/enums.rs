//! Enums Definitions
//!
//! Ref: <https://bybit-exchange.github.io/docs/v5/enum>

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::*;
use std::fmt;

/// Language of announcements (`locale`).
#[derive(Debug, Deserialize, Serialize)]
pub enum Locale {
    /// German.
    #[serde(rename = "de-DE")]
    DeDe,
    /// English (US).
    #[serde(rename = "en-US")]
    EnUs,
    /// Spanish (Argentina).
    #[serde(rename = "es-AR")]
    EsAr,
    /// Spanish (Spain).
    #[serde(rename = "es-ES")]
    EsEs,
    /// Spanish (Mexico).
    #[serde(rename = "es-MX")]
    EsMx,
    /// French.
    #[serde(rename = "fr-FR")]
    FrFr,
    /// Kazakh.
    #[serde(rename = "kk-KZ")]
    KkKz,
    /// Indonesian.
    #[serde(rename = "id-ID")]
    IdId,
    /// Ukrainian.
    #[serde(rename = "uk-UA")]
    UkUa,
    /// Japanese.
    #[serde(rename = "ja-JP")]
    JaJp,
    /// Russian.
    #[serde(rename = "ru-RU")]
    RuRu,
    /// Thai.
    #[serde(rename = "th-TH")]
    ThTh,
    /// Portuguese (Brazil).
    #[serde(rename = "pt-BR")]
    PtBr,
    /// Turkish.
    #[serde(rename = "tr-TR")]
    TrTr,
    /// Vietnamese.
    #[serde(rename = "vi-VN")]
    ViVn,
    /// Chinese (Traditional).
    #[serde(rename = "zh-TW")]
    ZhTw,
    /// Arabic.
    #[serde(rename = "ar-SA")]
    ArSa,
    /// Hindi.
    #[serde(rename = "hi-IN")]
    HiIn,
    /// Filipino.
    #[serde(rename = "fil-PH")]
    FilPh,
}

/// Announcement category (`announcementType`).
#[derive(Debug, Deserialize, Serialize)]
pub enum AnnouncementType {
    /// New crypto listings.
    #[serde(rename = "new_crypto")]
    NewCrypto,
    /// Latest Bybit news.
    #[serde(rename = "latest_bybit_news")]
    LatestBybitNews,
    /// Delistings.
    #[serde(rename = "delistings")]
    Delistings,
    /// Latest activities and campaigns.
    #[serde(rename = "latest_activities")]
    LatestActivities,
    /// Product updates.
    #[serde(rename = "product_updates")]
    ProductUpdates,
    /// Maintenance updates.
    #[serde(rename = "maintenance_updates")]
    MaintenanceUpdates,
    /// New fiat listings.
    #[serde(rename = "new_fiat_listings")]
    NewFiatListings,
    /// Other announcements.
    #[serde(rename = "other")]
    Other,
}

/// Unified Account: spot | linear | inverse | option
/// Classic Account: linear | inverse | spot
#[derive(PartialEq, Eq, Hash, Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    /// Inverse contract, including Inverse perp, Inverse futures.
    Inverse,
    /// USDT perpetual, and USDC contract, including USDC perp, USDC futures.
    Linear,
    /// Options.
    Option,
    /// Spot.
    Spot,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Inverse => "inverse",
            Self::Linear => "linear",
            Self::Option => "option",
            Self::Spot => "spot",
        };
        write!(f, "{value}")
    }
}

/// Order status (`orderStatus`).
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub enum OrderStatus {
    // open status
    /// order has been placed successfully
    New,
    /// Partially filled; the rest is still open.
    PartiallyFilled,
    /// Conditional orders are created
    Untriggered,
    // closed status
    /// Rejected (a closed status).
    Rejected,
    /// Only spot has this order status
    PartiallyFilledCanceled,
    /// Fully filled (a closed status).
    Filled,
    /// In derivatives, orders with this status may have an executed qty
    Cancelled,
    /// instantaneous state for conditional orders from Untriggered to New
    Triggered,
    /// UTA: Spot tp/sl order, conditional order, OCO order are cancelled before they are triggered
    Deactivated,
}

impl OrderStatus {
    /// The order is still active (`New`, `PartiallyFilled` or `Untriggered`).
    pub fn is_open(&self) -> bool {
        matches!(self, Self::New | Self::PartiallyFilled | Self::Untriggered)
    }
    /// The order reached a final status.
    pub fn is_closed(&self) -> bool {
        matches!(
            self,
            Self::Rejected
                | Self::PartiallyFilledCanceled
                | Self::Filled
                | Self::Cancelled
                | Self::Triggered
                | Self::Deactivated
        )
    }
}

/// How long an order stays active (`timeInForce`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum TimeInForce {
    /// GoodTillCancel
    GTC,
    /// ImmediateOrCancel
    IOC,
    /// FillOrKill
    FOK,
    /// Maker only: the order is cancelled if it would execute immediately.
    PostOnly,
    /// features:
    /// Exclusive Matching: Only match non-algorithmic users; no execution against orders from Open API.
    /// Post-Only Mechanism: Act as maker orders, adding liquidity
    /// Lower Priority: Execute after non-RPI orders at the same price level.
    /// Limited Access: Initially for select market makers across multiple pairs.
    /// Order Book Updates: Excluded from API but displayed on the GUI.
    RPI,
}

/// What created the order (`createType`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum CreateType {
    /// Placed by the user.
    CreateByUser,
    /// Spread order
    CreateByFutureSpread,
    /// Closing order placed by Bybit (admin).
    CreateByAdminClosing,
    /// USDC Futures delivery; position closed as a result of the delisting of a contract. This is recorded as a trade but not an order.
    CreateBySettle,
    /// Futures conditional order
    CreateByStopOrder,
    /// Futures take profit order
    CreateByTakeProfit,
    /// Futures partial take profit order
    CreateByPartialTakeProfit,
    /// Futures stop loss order
    CreateByStopLoss,
    /// Futures partial stop loss order
    CreateByPartialStopLoss,
    /// Futures trailing stop order
    CreateByTrailingStop,
    /// Laddered liquidation to reduce the required maintenance margin
    CreateByLiq,
    /// If the position is still subject to liquidation (i.e., does not meet the required maintenance margin level), the position shall be taken over by the liquidation engine and closed at the bankruptcy price.
    #[serde(rename = "CreateByTakeOver_PassThrough")]
    CreateByTakeOverPassThrough,
    /// Auto-Deleveraging(ADL)
    #[serde(rename = "CreateByAdl_PassThrough")]
    CreateByAdlPassThrough,
    /// Order placed via Paradigm
    #[serde(rename = "CreateByBlock_PassThrough")]
    CreateByBlockPassThrough,
    /// Order created by move position
    #[serde(rename = "CreateByBlockTradeMovePosition_PassThrough")]
    CreateByBlockTradeMovePositionPassThrough,
    /// The close order placed via web or app position area - web/app
    CreateByClosing,
    /// Order created via grid bot - web/app
    CreateByFGridBot,
    /// Order closed via grid bot - web/app
    CloseByFGridBot,
    /// Order created by TWAP - web/app
    CreateByTWAP,
    /// Order created by TV webhook - web/app
    CreateByTVSignal,
    /// Order created by Mm rate close function - web/app
    CreateByMmRateClose,
    /// Order created by Martingale bot - web/app
    CreateByMartingaleBot,
    /// Order closed by Martingale bot - web/app
    CloseByMartingaleBot,
    /// Order created by Ice berg strategy - web/app
    CreateByIceBerg,
    /// Order created by arbitrage - web/app
    CreateByArbitrage,
    /// Option dynamic delta hedge order - web/app
    CreateByDdh,
}

/// Kind of execution (`execType`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum ExecType {
    /// A regular trade.
    Trade,
    /// Auto-Deleveraging
    AdlTrade,
    /// Funding fee
    Funding,
    /// Takeover liquidation
    BustTrade,
    /// USDC futures delivery; Position closed by contract delisted
    Delivery,
    /// Inverse futures settlement; Position closed due to delisting
    Settle,
    /// A block trade.
    BlockTrade,
    /// A position moved between accounts.
    MovePosition,
    /// Spread leg execution
    FutureSpread,
    /// May be returned by a classic account. Cannot query by this type
    UNKNOWN,
}

/// Order type (`orderType`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum OrderType {
    /// Market order.
    Market,
    /// Limit order.
    Limit,
    /// is not a valid request parameter value. Is only used in some responses. Mainly, it is used when execType is Funding.
    UNKNOWN,
}

/// Type of a conditional or TP/SL order (`stopOrderType`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum StopOrderType {
    /// Take profit for the whole position.
    TakeProfit,
    /// Stop loss for the whole position.
    StopLoss,
    /// Trailing stop.
    TrailingStop,
    /// Conditional order triggered by price.
    Stop,
    /// Take profit for part of the position.
    PartialTakeProfit,
    /// Stop loss for part of the position.
    PartialStopLoss,
    /// spot TP/SL order
    #[serde(rename = "tpslOrder")]
    TpslOrder,
    /// spot Oco order
    OcoOrder,
    /// On web or app can set MMR to close position
    MmRateClose,
    /// Spot bidirectional tpsl order
    BidirectionalTpslOrder,
    /// Not a conditional order, or an unknown value.
    UNKNOWN,
}

/// Direction of the last price change (`tickDirection`).
#[derive(Debug, PartialEq, Deserialize, Clone, Copy)]
pub enum TickDirection {
    /// price rise
    PlusTick,
    /// trade occurs at the same price as the previous trade, which occurred at a price higher than that for the trade preceding it
    ZeroPlusTick,
    /// price drop
    MinusTick,
    /// trade occurs at the same price as the previous trade, which occurred at a price lower than that for the trade preceding it
    ZeroMinusTick,
}

/// Kline interval (`interval`).
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub enum Interval {
    /// 1 minute.
    #[serde(rename = "1")]
    Minute1,
    /// 3 minutes.
    #[serde(rename = "3")]
    Minute3,
    /// 5 minutes.
    #[serde(rename = "5")]
    Minute5,
    /// 15 minutes.
    #[serde(rename = "15")]
    Minute15,
    /// 30 minutes.
    #[serde(rename = "30")]
    Minute30,
    /// 1 hour.
    #[serde(rename = "60")]
    Hour1,
    /// 2 hours.
    #[serde(rename = "120")]
    Hour2,
    /// 4 hours.
    #[serde(rename = "240")]
    Hour4,
    /// 6 hours.
    #[serde(rename = "360")]
    Hour6,
    /// 12 hours.
    #[serde(rename = "720")]
    Hour12,
    /// 1 day.
    #[serde(rename = "D")]
    Day1,
    /// 1 week.
    #[serde(rename = "W")]
    Week1,
    /// 1 month.
    #[serde(rename = "M")]
    Month1,
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Minute1 => "1",
            Self::Minute3 => "3",
            Self::Minute5 => "5",
            Self::Minute15 => "15",
            Self::Minute30 => "30",
            Self::Hour1 => "60",
            Self::Hour2 => "120",
            Self::Hour4 => "240",
            Self::Hour6 => "360",
            Self::Hour12 => "720",
            Self::Day1 => "D",
            Self::Week1 => "W",
            Self::Month1 => "M",
        };
        write!(f, "{value}")
    }
}

/// Interval of open interest data (`intervalTime`).
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub enum IntervalTime {
    /// 5 minutes.
    #[serde(rename = "5min")]
    Minute5,
    /// 15 minutes.
    #[serde(rename = "15min")]
    Minute15,
    /// 30 minutes.
    #[serde(rename = "30min")]
    Minute30,
    /// 1 hour.
    #[serde(rename = "1h")]
    Hour1,
    /// 4 hours.
    #[serde(rename = "4h")]
    Hour4,
    /// 1 day.
    #[serde(rename = "1d")]
    Day1,
}

/// Position index: one-way mode or a side of hedge mode (`positionIdx`).
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(u8)]
pub enum PositionIdx {
    /// 0:one-way mode position
    OneWay = 0,
    /// 1:Buy side of hedge-mode position
    Buy = 1,
    /// 2:Sell side of hedge-mode position
    Sell = 2,
}

/// Position mode of a symbol or settle coin (`mode`).
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum PositionMode {
    /// 0: Merged Single (one-way)
    OneWay = 0,
    /// 3: Both Sides (hedge)
    Hedge = 3,
}

/// Position status (`positionStatus`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum PositionStatus {
    /// Normal.
    Normal,
    /// in the liquidation progress
    Liq,
    /// in the auto-deleverage progress
    Adl,
}

/// Why an order was rejected or cancelled by the matching engine (`rejectReason`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum RejectReason {
    /// No error.
    #[serde(rename = "EC_NoError")]
    EcNoError,
    /// Other reason.
    #[serde(rename = "EC_Others")]
    EcOthers,
    /// Unknown message type.
    #[serde(rename = "EC_UnknownMessageType")]
    EcUnknownMessageType,
    /// Missing client order id.
    #[serde(rename = "EC_MissingClOrdID")]
    EcMissingClOrdId,
    /// Missing original client order id.
    #[serde(rename = "EC_MissingOrigClOrdID")]
    EcMissingOrigClOrdId,
    /// Client order id and original client order id are the same.
    #[serde(rename = "EC_ClOrdIDOrigClOrdIDAreTheSame")]
    EcClOrdIdorigClOrdIdareTheSame,
    /// Duplicated client order id.
    #[serde(rename = "EC_DuplicatedClOrdID")]
    EcDuplicatedClOrdId,
    /// Original client order id does not exist.
    #[serde(rename = "EC_OrigClOrdIDDoesNotExist")]
    EcOrigClOrdIddoesNotExist,
    /// Too late to cancel.
    #[serde(rename = "EC_TooLateToCancel")]
    EcTooLateToCancel,
    /// Unknown order type.
    #[serde(rename = "EC_UnknownOrderType")]
    EcUnknownOrderType,
    /// Unknown side.
    #[serde(rename = "EC_UnknownSide")]
    EcUnknownSide,
    /// Unknown time in force.
    #[serde(rename = "EC_UnknownTimeInForce")]
    EcUnknownTimeInForce,
    /// Wrongly routed.
    #[serde(rename = "EC_WronglyRouted")]
    EcWronglyRouted,
    /// A market order has a non-zero price.
    #[serde(rename = "EC_MarketOrderPriceIsNotZero")]
    EcMarketOrderPriceIsNotZero,
    /// Invalid limit price.
    #[serde(rename = "EC_LimitOrderInvalidPrice")]
    EcLimitOrderInvalidPrice,
    /// Not enough quantity to fill.
    #[serde(rename = "EC_NoEnoughQtyToFill")]
    EcNoEnoughQtyToFill,
    /// No maker could be found to fill the order.
    #[serde(rename = "EC_NoImmediateQtyToFill")]
    EcNoImmediateQtyToFill,
    /// Cancelled by request.
    #[serde(rename = "EC_PerCancelRequest")]
    EcPerCancelRequest,
    /// A market order cannot be post-only.
    #[serde(rename = "EC_MarketOrderCannotBePostOnly")]
    EcMarketOrderCannotBePostOnly,
    /// The post-only order would have executed as a taker.
    #[serde(rename = "EC_PostOnlyWillTakeLiquidity")]
    EcPostOnlyWillTakeLiquidity,
    /// Cancelled by a replace (amend).
    #[serde(rename = "EC_CancelReplaceOrder")]
    EcCancelReplaceOrder,
    /// The symbol status does not allow it.
    #[serde(rename = "EC_InvalidSymbolStatus")]
    EcInvalidSymbolStatus,
    /// Cancelled because it could not be filled completely (FOK).
    #[serde(rename = "EC_CancelForNoFullFill")]
    EcCancelForNoFullFill,
    /// Cancelled by self-match prevention.
    #[serde(rename = "EC_BySelfMatch")]
    EcBySelfMatch,
    /// used for pre-market order operation, e.g., during 2nd phase of call auction, cancel order is not allowed, when the cancel request is failed to be rejected by trading server, the request will be rejected by matching box finally
    #[serde(rename = "EC_InCallAuctionStatus")]
    EcInCallAuctionStatus,
    /// Quantity cannot be zero.
    #[serde(rename = "EC_QtyCannotBeZero")]
    EcQtyCannotBeZero,
    /// Market orders do not support this time in force.
    #[serde(rename = "EC_MarketOrderNoSupportTIF")]
    EcMarketOrderNoSupportTif,
    /// Maximum number of trades reached.
    #[serde(rename = "EC_ReachMaxTradeNum")]
    EcReachMaxTradeNum,
    /// Invalid price precision.
    #[serde(rename = "EC_InvalidPriceScale")]
    EcInvalidPriceScale,
    /// Invalid bit index.
    #[serde(rename = "EC_BitIndexInvalid")]
    EcBitIndexInvalid,
    /// Stopped by self-match prevention.
    #[serde(rename = "EC_StopBySelfMatch")]
    EcStopBySelfMatch,
    /// Invalid self-match prevention type.
    #[serde(rename = "EC_InvalidSmpType")]
    EcInvalidSmpType,
    /// Cancelled by market maker protection.
    #[serde(rename = "EC_CancelByMMP")]
    EcCancelByMmp,
    /// Invalid user type.
    #[serde(rename = "EC_InvalidUserType")]
    EcInvalidUserType,
    /// Invalid mirror order id.
    #[serde(rename = "EC_InvalidMirrorOid")]
    EcInvalidMirrorOid,
    /// Invalid mirror user id.
    #[serde(rename = "EC_InvalidMirrorUid")]
    EcInvalidMirrorUid,
    /// Invalid quantity.
    #[serde(rename = "EC_EcInvalidQty")]
    EcEcInvalidQty,
    /// Invalid amount.
    #[serde(rename = "EC_InvalidAmount")]
    EcInvalidAmount,
    /// Cancelled while loading the order.
    #[serde(rename = "EC_LoadOrderCancel")]
    EcLoadOrderCancel,
    /// Market orders in quote quantity do not support sell.
    #[serde(rename = "EC_MarketQuoteNoSuppSell")]
    EcMarketQuoteNoSuppSell,
    /// Out-of-order order id.
    #[serde(rename = "EC_DisorderOrderID")]
    EcDisorderOrderId,
    /// Invalid base value.
    #[serde(rename = "EC_InvalidBaseValue")]
    EcInvalidBaseValue,
    /// The loaded order can match.
    #[serde(rename = "EC_LoadOrderCanMatch")]
    EcLoadOrderCanMatch,
    /// Security status check failed.
    #[serde(rename = "EC_SecurityStatusFail")]
    EcSecurityStatusFail,
    /// Risk price limit reached.
    #[serde(rename = "EC_ReachRiskPriceLimit")]
    EcReachRiskPriceLimit,
    /// The order does not exist.
    #[serde(rename = "EC_OrderNotExist")]
    EcOrderNotExist,
    /// Cancelled because its remaining value is zero.
    #[serde(rename = "EC_CancelByOrderValueZero")]
    EcCancelByOrderValueZero,
    /// Cancelled because the order it matched with has a remaining value of zero.
    #[serde(rename = "EC_CancelByMatchValueZero")]
    EcCancelByMatchValueZero,
    /// Market price limit reached.
    #[serde(rename = "EC_ReachMarketPriceLimit")]
    EcReachMarketPriceLimit,
}

/// Account type (`accountType`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum AccountType {
    /// Inverse Derivatives Account | Derivatives Account
    CONTRACT,
    /// Unified Trading Account
    UNIFIED,
    /// Funding Account
    FUND,
    /// Spot Account
    SPOT,
}

impl AccountType {
    /// Account type used by UTA 2.0 accounts.
    pub fn is_uta_2(&self) -> bool {
        matches!(self, Self::UNIFIED | Self::FUND)
    }
    /// Account type used by UTA 1.0 accounts.
    pub fn is_uta_1(&self) -> bool {
        matches!(self, Self::CONTRACT | Self::UNIFIED | Self::FUND)
    }
    /// Account type used by classic accounts.
    pub fn is_classic(&self) -> bool {
        matches!(self, Self::SPOT | Self::CONTRACT | Self::FUND)
    }
}

/// Status of a transfer (`transferStatus`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum TransferStatus {
    /// Completed.
    SUCCESS,
    /// In progress.
    PENDING,
    /// Failed.
    FAILED,
}

/// Status of a deposit (`depositStatus`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum DepositStatus {
    /// Unknown.
    #[serde(rename = "0")]
    Unknown,
    /// Waiting for confirmations.
    #[serde(rename = "1")]
    ToBeConfirmed,
    /// Processing.
    #[serde(rename = "2")]
    Processing,
    /// (finalised status of a success deposit)
    #[serde(rename = "3")]
    Success,
    /// Failed.
    #[serde(rename = "4")]
    DepositFailed,
    /// Pending to be credited to the funding pool.
    #[serde(rename = "10011")]
    PendingToBeCreditedToFundingPool,
    /// Credited to the funding pool.
    #[serde(rename = "10012")]
    CreditedToFundingPoolSuccessfully,
}

/// Status of a withdrawal (`withdrawStatus`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum WithdrawStatus {
    /// Under security check.
    SecurityCheck,
    /// Pending.
    Pending,
    /// Completed.
    #[serde(rename = "success")]
    Success,
    /// Cancelled by the user.
    CancelByUser,
    /// Rejected.
    Reject,
    /// Failed.
    Fail,
    /// Confirmed on the blockchain.
    BlockchainConfirmed,
    /// More information is required.
    MoreInformationRequired,
    /// a rare status
    Unknown,
}

/// Price used to trigger a conditional order or TP/SL (`triggerBy`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum TriggerBy {
    /// Last traded price.
    LastPrice,
    /// Index price.
    IndexPrice,
    /// Mark price.
    MarkPrice,
    /// Not set, or an unknown value.
    UNKNOWN,
}

/// Why an order was cancelled (`cancelType`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum CancelType {
    /// Cancelled by the user.
    CancelByUser,
    /// cancelled by reduceOnly
    CancelByReduceOnly,
    /// cancelled in order to attempt liquidation prevention by freeing up margin
    CancelByPrepareLiq,
    /// cancelled in order to attempt liquidation prevention by freeing up margin
    CancelAllBeforeLiq,
    /// cancelled due to ADL
    CancelByPrepareAdl,
    /// cancelled due to ADL
    CancelAllBeforeAdl,
    /// Cancelled by Bybit (admin).
    CancelByAdmin,
    /// cancelled due to delisting contract
    CancelBySettle,
    /// TP/SL order cancelled when the position is cleared
    CancelByTpSlTsClear,
    /// cancelled by SMP
    CancelBySmp,
    /// cancelled by DCP triggering
    CancelByDCP,
    /// Spread trading: the order price of a single leg order is outside the limit price range.
    CancelByRebalance,

    // Options:
    /// Cancelled because the account cannot afford the order cost.
    CancelByCannotAffordOrderCost,
    /// Cancelled because portfolio margin trial maintenance margin exceeded equity.
    CancelByPmTrialMmOverEquity,
    /// Cancelled because the account is blocked.
    CancelByAccountBlocking,
    /// Cancelled by delivery.
    CancelByDelivery,
    /// Cancelled by market maker protection.
    CancelByMmpTriggered,
    /// Cancelled by cross self-match.
    CancelByCrossSelfMuch,
    /// Cancelled because the cross reached the maximum number of trades.
    CancelByCrossReachMaxTradeNum,

    /// Not documented
    UNKNOWN,
}

/// Period of option historical volatility, days (`period`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum OptionPeriod {
    /// 7 days.
    #[serde(rename = "7")]
    Day7,
    /// 14 days.
    #[serde(rename = "14")]
    Day14,
    /// 21 days.
    #[serde(rename = "21")]
    Day21,
    /// 30 days.
    #[serde(rename = "30")]
    Day30,
    /// 60 days.
    #[serde(rename = "60")]
    Day60,
    /// 90 days.
    #[serde(rename = "90")]
    Day90,
    /// 180 days.
    #[serde(rename = "180")]
    Day180,
    /// 270 days.
    #[serde(rename = "270")]
    Day270,
}

/// Data recording period of long/short ratio data (`period`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum DataRecordingPeriod {
    /// 5 minutes.
    #[serde(rename = "5min")]
    Minute5,
    /// 15 minutes.
    #[serde(rename = "15min")]
    Minute15,
    /// 30 minutes.
    #[serde(rename = "30min")]
    Minute30,
    /// 1 hour.
    #[serde(rename = "1h")]
    Hour1,
    /// 4 hours.
    #[serde(rename = "4h")]
    Hour4,
    /// 4 days.
    #[serde(rename = "4d")]
    Day4,
}

/// Contract type of a derivatives instrument (`contractType`).
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub enum ContractType {
    /// Inverse perpetual.
    InversePerpetual,
    /// Linear (USDT/USDC) perpetual.
    LinearPerpetual,
    /// USDT/USDC Futures
    LinearFutures,
    /// Inverse futures.
    InverseFutures,
}

/// Trading status of an instrument (`status`).
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub enum Status {
    /// Pre-market (pre-launch) contract.
    PreLaunch,
    /// Trading.
    Trading,
    /// Being delivered (futures and options).
    Delivering,
    /// Closed.
    Closed,
}

/// Whether a spot pair supports margin trading (`marginTrading`).
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum MarginTrading {
    /// Regardless of normal account or UTA account, this trading pair does not support margin trading
    None,
    /// For both normal account and UTA account, this trading pair supports margin trading
    Both,
    /// Only for UTA account,this trading pair supports margin trading
    UtaOnly,
    /// Only for normal account, this trading pair supports margin trading
    NormalSpotOnly,
}

/// Whether a symbol supports copy trading (`copyTrading`).
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum CopyTrading {
    /// Regardless of normal account or UTA account, this trading pair does not support copy trading
    None,
    /// For both normal account and UTA account, this trading pair supports copy trading
    Both,
    /// Only for UTA account,this trading pair supports copy trading
    UtaOnly,
    /// Only for normal account, this trading pair supports copy trading
    NormalOnly,
}

/// Type of a transaction log entry (`type`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    /// Assets that transferred into Unified | (inverse) derivatives wallet
    TransferIn,
    /// Assets that transferred out from Unified | (inverse) derivatives wallet
    TransferOut,
    /// Trade.
    Trade,
    /// USDT Perp funding settlement, and USDC Perp funding settlement + USDC 8-hour session settlement
    /// USDT / Inverse Perp funding settlement
    Settlement,
    /// USDC Futures, Option delivery
    Delivery,
    /// Inverse Futures delivery
    Liquidation,
    /// Auto-Deleveraging
    ADL,
    /// Airdrop.
    Airdrop,
    /// Bonus claimed
    Bonus,
    /// Bonus expired
    BonusRecollect,
    /// Trading fee refunded
    FeeRefund,
    /// Interest occurred due to borrowing
    Interest,
    /// Currency convert, and the liquidation for borrowing asset(UTA loan)
    CurrencyBuy,
    /// Currency convert, and the liquidation for borrowing asset(UTA loan)
    CurrencySell,
    /// Amount borrowed by an institutional loan.
    BorrowedAmountInsLoan,
    /// Principal repayment of an institutional loan.
    PrincipleRepaymentInsLoan,
    /// Interest repayment of an institutional loan.
    InterestRepaymentInsLoan,
    /// the liquidation for borrowing asset(INS loan)
    AutoSoldCollateralInsLoan,
    /// the liquidation for borrowing asset(INS loan)
    AutoBuyLiabilityInsLoan,
    /// Automatic principal repayment of an institutional loan.
    AutoPrincipleRepaymentInsLoan,
    /// Automatic interest repayment of an institutional loan.
    AutoInterestRepaymentInsLoan,
    /// Transfer In when in the liquidation of OTC loan
    TransferInInsLoan,
    /// Transfer Out when in the liquidation of OTC loan
    TransferOutInsLoan,
    /// One-click repayment currency sell
    SpotRepaymentSell,
    /// One-click repayment currency buy
    SpotRepaymentBuy,
    /// Spot leverage token subscription
    TokensSubscription,
    /// Spot leverage token redemption
    TokensRedemption,
    /// Asset auto deducted by system (roll back)
    AutoDeduction,
    /// Byfi flexible stake subscription
    FlexibleStakingSubscription,
    /// Byfi flexible stake redemption
    FlexibleStakingRedemption,
    /// Byfi fixed stake subscription
    FixedStakingSubscription,
    /// Transfer out for pre-market trading.
    PremarketTransferOut,
    /// Pre-market delivery: new coins sold.
    PremarketDeliverySellNewCoin,
    /// Pre-market delivery: new coins bought.
    PremarketDeliveryBuyNewCoin,
    /// Pre-market delivery: pledge paid to the seller.
    PremarketDeliveryPledgePaySeller,
    /// Pre-market delivery: pledge returned.
    PremarketDeliveryPledgeBack,
    /// Pre-market rollback: pledge returned.
    PremarketRollbackPledgeBack,
    /// Pre-market rollback: pledge penalty paid to the buyer.
    PremarketRollbackPledgePenaltyToBuyer,
    /// fireblocks business
    CustodyNetworkFee,
    /// fireblocks business
    CustodySettleFee,
    /// fireblocks / copper business
    CustodyLock,
    /// fireblocks business
    CustodyUnlock,
    /// fireblocks business
    CustodyUnlockRefund,
    /// crypto loan
    LoansBorrowFunds,
    /// crypto loan repayment
    LoansPledgeAsset,
    /// Bonus transferred in.
    BonusTransferIn,
    /// Bonus transferred out.
    BonusTransferOut,
    /// Transfer in for PEF.
    PefTransferIn,
    /// Transfer out for PEF.
    PefTransferOut,
    /// PEF profit share.
    PefProfitShare,
    /// Any type not covered by the other variants.
    #[serde(rename = "Others")]
    Others,
}

impl Type {
    /// Transaction type that appears in the unified account log.
    pub fn is_uta(&self) -> bool {
        matches!(
            self,
            Self::TransferIn
                | Self::TransferOut
                | Self::Trade
                | Self::Settlement
                | Self::Delivery
                | Self::Liquidation
                | Self::ADL
                | Self::Airdrop
                | Self::Bonus
                | Self::BonusRecollect
                | Self::FeeRefund
                | Self::Interest
                | Self::CurrencyBuy
                | Self::CurrencySell
                | Self::BorrowedAmountInsLoan
                | Self::PrincipleRepaymentInsLoan
                | Self::InterestRepaymentInsLoan
                | Self::AutoSoldCollateralInsLoan
                | Self::AutoBuyLiabilityInsLoan
                | Self::AutoPrincipleRepaymentInsLoan
                | Self::AutoInterestRepaymentInsLoan
                | Self::TransferInInsLoan
                | Self::TransferOutInsLoan
                | Self::SpotRepaymentSell
                | Self::SpotRepaymentBuy
                | Self::TokensSubscription
                | Self::TokensRedemption
                | Self::AutoDeduction
                | Self::FlexibleStakingSubscription
                | Self::FlexibleStakingRedemption
                | Self::FixedStakingSubscription
                | Self::PremarketTransferOut
                | Self::PremarketDeliverySellNewCoin
                | Self::PremarketDeliveryBuyNewCoin
                | Self::PremarketDeliveryPledgePaySeller
                | Self::PremarketDeliveryPledgeBack
                | Self::PremarketRollbackPledgeBack
                | Self::PremarketRollbackPledgePenaltyToBuyer
                | Self::CustodyNetworkFee
                | Self::CustodySettleFee
                | Self::CustodyLock
                | Self::CustodyUnlock
                | Self::CustodyUnlockRefund
                | Self::LoansBorrowFunds
                | Self::LoansPledgeAsset
                | Self::BonusTransferIn
                | Self::BonusTransferOut
                | Self::PefTransferIn
                | Self::PefTransferOut
                | Self::PefProfitShare
        )
    }
    /// Transaction type that appears in the contract (classic) account log.
    pub fn is_contract(&self) -> bool {
        matches!(
            self,
            Self::TransferIn
                | Self::TransferOut
                | Self::Trade
                | Self::Settlement
                | Self::Delivery
                | Self::Liquidation
                | Self::ADL
                | Self::Airdrop
                | Self::Bonus
                | Self::BonusRecollect
                | Self::FeeRefund
                | Self::CurrencyBuy
                | Self::CurrencySell
                | Self::AutoDeduction
                | Self::Others
        )
    }
}

/// Account mode (`unifiedMarginStatus`).
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum UnifiedMarginStatus {
    /// Classic account.
    ClassicAccount = 1,
    /// 1.0
    UnifiedTradingAccount1 = 3,
    /// 1.0 (pro version)
    UnifiedTradingAccount1Pro = 4,
    /// 2.0
    UnifiedTradingAccount2 = 5,
    /// 2.0 (pro version)
    UnifiedTradingAccount2Pro = 6,
}

/// Margin mode of a unified account (`marginMode`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarginMode {
    /// Isolated margin.
    IsolatedMargin,
    /// Regular (cross) margin.
    RegularMargin,
    /// Portfolio margin.
    PortfolioMargin,
}

/// Whether Spot Margin Trade (UTA) is turned on for the account.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum SpotMarginMode {
    /// Spot margin trading off.
    #[serde(rename = "0")]
    Disabled,
    /// Spot margin trading on.
    #[serde(rename = "1")]
    Enabled,
}

/// Whether spot hedging is on (`spotHedgingStatus`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SpotHedgingStatus {
    /// On.
    On,
    /// Off.
    Off,
}

/// Status of a leveraged token (`ltStatus`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum LtStatus {
    /// LT can be purchased and redeemed
    #[serde(rename = "1")]
    CanBePurchasedAndRedeemed,
    /// LT can be purchased, but not redeemed
    #[serde(rename = "2")]
    CanBePurchasedButNotRedeemed,
    /// LT can be redeemed, but not purchased
    #[serde(rename = "3")]
    CanBeRedeemedButNotPurchased,
    /// LT cannot be purchased nor redeemed
    #[serde(rename = "4")]
    CannotBePurchasedNorRedeemed,
    /// Adjusting position
    #[serde(rename = "5")]
    AdjustingPosition,
}

/// Status of a leveraged token purchase or redemption order.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum LtOrderStatus {
    /// Completed.
    #[serde(rename = "1")]
    Completed,
    /// In progress.
    #[serde(rename = "2")]
    InProgress,
    /// Failed.
    #[serde(rename = "3")]
    Failed,
}

/// Whether a leveraged token order record is a purchase or a redemption.
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum LtOrderType {
    /// Purchase.
    Purchase = 1,
    /// Redemption.
    Redemption = 2,
}

/// Account type of a convert (`accountType`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum ConvertAccountType {
    /// Unified Trading Account
    #[serde(rename = "eb_convert_uta")]
    Uta,
    /// Funding Account
    #[serde(rename = "eb_convert_funding")]
    Funding,
    /// Inverse Derivatives Account (no USDT in this wallet))
    #[serde(rename = "eb_convert_inverse")]
    Inverse,
    /// Spot Account
    #[serde(rename = "eb_convert_spot")]
    Spot,
    /// Derivatives Account (contain USDT in this wallet)
    #[serde(rename = "eb_convert_contract")]
    Contract,
}

/// VIP or PRO level of the account (`vipLevel`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum VipLevel {
    /// No VIP level.
    #[serde(rename = "No VIP")]
    NoVIP,
    /// VIP level 1.
    #[serde(rename = "VIP-1")]
    VIP1,
    /// VIP level 2.
    #[serde(rename = "VIP-2")]
    VIP2,
    /// VIP level 3.
    #[serde(rename = "VIP-3")]
    VIP3,
    /// VIP level 4.
    #[serde(rename = "VIP-4")]
    VIP4,
    /// VIP level 5.
    #[serde(rename = "VIP-5")]
    VIP5,
    /// VIP Supreme.
    #[serde(rename = "VIP-Supreme")]
    VIPSupreme,
    /// PRO level 1.
    #[serde(rename = "PRO-1")]
    PRO1,
    /// PRO level 2.
    #[serde(rename = "PRO-2")]
    PRO2,
    /// PRO level 3.
    #[serde(rename = "PRO-3")]
    PRO3,
    /// PRO level 4.
    #[serde(rename = "PRO-4")]
    PRO4,
    /// PRO level 5.
    #[serde(rename = "PRO-5")]
    PRO5,
}

/// Auto-deleveraging rank, 0 (lowest priority) to 5 (`adlRankIndicator`).
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum AdlRankIndicator {
    /// default value of empty position
    Zero = 0,
    /// Rank 1.
    One = 1,
    /// Rank 2.
    Two = 2,
    /// Rank 3.
    Three = 3,
    /// Rank 4.
    Four = 4,
    /// Rank 5.
    Five = 5,
}

/// Self-match prevention type (`smpType`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum SmpType {
    /// default
    None,
    /// Cancel the maker order.
    CancelMaker,
    /// Cancel the taker order.
    CancelTaker,
    /// Cancel both orders.
    CancelBoth,
}

/// Type of an extra fee (`feeType`), charged only on some regional sites.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExtraFeeType {
    /// Unknown.
    Unknown,
    /// Government tax. Only for Indonesian site
    Tax,
    /// Indonesian foreign exchange tax. Only for Indonesian site
    Cfx,
    /// EU withholding tax. Only for EU site
    Wht,
    /// Indian GST tax. Only for kyc=Indian users
    Gst,
    /// ARE VAT tax. Only for kyc=ARE users
    Vat,
}

/// Subtype of an extra fee (`subFeeType`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExtraSubFeeType {
    /// Unknown.
    Unknown,
    /// Tax fee, fiat currency to digital currency. Only for Indonesian site
    TaxPnn,
    /// Tax fee, digital currency to fiat currency. Only for Indonesian site
    TaxPph,
    /// CFX fee, fiat currency to digital currency. Only for Indonesian site
    CfxFiee,
    /// EU site withholding tax. Only for EU site
    AutWithholdingTax,
    /// Indian GST tax. Only for kyc=Indian users
    IndGst,
    /// ARE VAT tax. Only for kyc=ARE users
    AreVat,
}

/// State of a system maintenance (`state`).
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum State {
    /// Scheduled.
    Scheduled,
    /// In progress.
    Ongoing,
    /// Completed.
    Completed,
    /// Cancelled.
    Canceled,
}

/// Service affected by a maintenance (`serviceTypes`).
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum ServiceTypes {
    /// Trading service
    TradingService = 1,
    /// Trading service via http request
    TradingServiceViaHttpRequest = 2,
    /// Trading service via websocket
    TradingServiceViaWebsocket = 3,
    /// Private websocket stream
    PrivateWebsocketStream = 4,
    /// Market data service
    MarketDataService = 5,
}

/// Product affected by a maintenance (`product`).
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Product {
    /// Futures.
    Futures = 1,
    /// Spot.
    Spot = 2,
    /// Option.
    Option = 3,
    /// Spread.
    Spread = 4,
}

/// Product of Disconnection Cancellation Protection (`product`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DCPProduct {
    /// Spot.
    Spot,
    /// Derivatives.
    Derivatives,
    /// Options.
    Option,
}

/// Outcome of an Upgrade to Unified Account (Pro) request.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UnifiedUpdateStatus {
    /// The upgrade failed.
    Fail,
    /// The upgrade is in progress.
    Process,
    /// The upgrade succeeded.
    Success,
}

/// Kind of system maintenance (`maintainType`).
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum MaintainType {
    /// Planned maintenance.
    PlannedMaintenance = 1,
    /// Temporary maintenance.
    TemporaryMaintenance = 2,
    /// Incident.
    Incident = 3,
}

/// Environment affected by a maintenance (`env`).
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Env {
    /// Production.
    Product = 1,
    /// Production demo service.
    ProductDemoService = 2,
}

/// TP/SL mode (`tpslMode`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum TpslMode {
    /// TP/SL for the whole position.
    Full,
    /// TP/SL for part of the position.
    Partial,
    /// Not set, or an unknown value.
    UNKNOWN,
}

/// Which leg triggered a spot OCO order (`ocoTriggerBy`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum OcoTriggerBy {
    /// Not triggered, or an unknown value.
    #[serde(rename = "OcoTriggerByUnknown")]
    Unknown,
    /// Triggered by the take profit.
    #[serde(rename = "OcoTriggerByTp")]
    Tp,
    /// Triggered by the stop loss.
    #[serde(rename = "OcoTriggerByBySl")]
    BySl,
}

/// Direction the price must move to trigger a conditional order (`triggerDirection`).
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum TriggerDirection {
    /// Not a conditional order.
    UNKNOWN = 0,
    /// Triggered when the price rises to the trigger price.
    Rise = 1,
    /// Triggered when the price falls to the trigger price.
    Fall = 2,
}

/// Auction phase of a pre-market contract (`curAuctionPhase`).
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub enum CurAuctionPhase {
    /// Pre-market trading is not started
    NotStarted,
    /// Pre-market trading is finished
    /// After the auction, if the pre-market contract fails to enter continues trading phase, it will be delisted and phase="Finished"
    /// After the continuous trading, if the pre-market contract fails to be converted to official contract, it will be delisted and phase="Finished"
    Finished,
    /// Auction phase of pre-market trading
    /// only timeInForce=GTC, orderType=Limit order is allowed to submit
    /// TP/SL are not supported; Conditional orders are not supported
    /// cannot modify the order at this stage
    /// order price range: [preOpenPrice x 0.5, maxPrice]
    CallAuction,
    /// Auction no cancel phase of pre-market trading
    /// only timeInForce=GTC, orderType=Limit order is allowed to submit
    /// TP/SL are not supported; Conditional orders are not supported
    /// cannot modify and cancel the order at this stage
    /// order price range: Buy [lastPrice x 0.5, markPrice x 1.1], Sell [markPrice x 0.9, maxPrice]
    CallAuctionNoCancel,
    /// cross matching phase
    /// cannot create, modify and cancel the order at this stage
    /// Candle data is released from this stage
    CrossMatching,
    /// Continuous trading phase
    /// There is no restriction to create, amend, cancel orders
    /// orderbook, public trade data is released from this stage
    ContinuousTrading,
}

/// How an option order is placed (`placeType`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum PlaceType {
    /// Default.
    #[serde(rename = "option")]
    Option,
    /// By implied volatility.
    #[serde(rename = "iv")]
    Iv,
    /// By price.
    #[serde(rename = "price")]
    Price,
}

/// Option type.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone, Copy)]
pub enum OptionType {
    /// Call option.
    Call,
    /// Put option.
    Put,
}

/// Order or trade side (`side`).
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub enum Side {
    /// Buy.
    Buy,
    /// Sell.
    Sell,
}
impl Side {
    /// The opposite side.
    pub fn reverse(&self) -> Self {
        match self {
            Side::Buy => Self::Sell,
            Side::Sell => Self::Buy,
        }
    }
}

/// Coin of a spot pair in which a fee is charged (see [`spot_fee_currency`]).
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Pair {
    // example of BTCUSDT
    /// Base coin, e.g. BTC of BTCUSDT.
    Base, // BTC
    /// Quote coin, e.g. USDT of BTCUSDT.
    Quote, // USDT
}

/// Unit of the slippage tolerance of a market order (`slippageToleranceType`).
#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
pub enum SlippageToleranceType {
    /// Number of ticks.
    TickSize,
    /// Percentage.
    Percent,
    /// default
    UNKNOWN,
}

/// Margin mode of a classic account position (`tradeMode`).
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum TradeMode {
    /// Cross margin.
    CrossMargin = 0,
    /// Isolated margin.
    IsolatedMargin = 1,
}

/// A WebSocket topic; serialized as Bybit expects, e.g. `tickers.BTCUSDT` or `order.linear`.
#[derive(Debug, Clone, PartialEq)]
pub enum Topic {
    /// Order book of a symbol (`orderbook.{depth}.{symbol}`).
    Orderbook {
        /// Symbol, e.g. `BTCUSDT`.
        symbol: String,
        /// Depth.
        depth: DepthLevel,
    },
    /// Public trades of a symbol (`publicTrade.{symbol}`).
    Trade(String),
    /// Ticker of a symbol (`tickers.{symbol}`).
    Ticker(String),
    /// Klines of a symbol (`kline.{interval}.{symbol}`).
    Kline {
        /// Symbol, e.g. `BTCUSDT`.
        symbol: String,
        /// Kline interval.
        interval: Interval,
    },
    /// All liquidations of a symbol (`allLiquidation.{symbol}`).
    AllLiquidation(String),
    /// Positions of a category (`position.{category}`).
    Position(Category),
    /// Positions of all categories (`position`).
    PositionAllCategory,
    /// Executions of a category (`execution.{category}`).
    Execution(Category),
    /// Executions of all categories (`execution`).
    ExecutionAllCategory,
    /// Fast executions of a category (`execution.fast.{category}`).
    FastExecution(Category),
    /// Fast executions of all categories (`execution.fast`).
    FastExecutionAllCategory,
    /// Orders of a category (`order.{category}`).
    Order(Category),
    /// Orders of all categories (`order`).
    OrderAllCategory,
    /// Wallet balance (`wallet`).
    Wallet,
    /// option only.
    Greek,
    /// Disconnection Cancellation Protection status (`dcp.{function}`).
    Dcp(DcpFunction),
}

impl fmt::Display for Topic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Orderbook { symbol, depth } => format!("orderbook.{depth}.{symbol}"),
            Self::Trade(symbol) => format!("publicTrade.{symbol}"),
            Self::Ticker(symbol) => format!("tickers.{symbol}"),
            Self::Kline { symbol, interval } => format!("kline.{interval}.{symbol}"),
            Self::AllLiquidation(symbol) => format!("allLiquidation.{symbol}"),
            Self::Position(category) => format!("position.{category}"),
            Self::PositionAllCategory => "position".to_string(),
            Self::Execution(category) => format!("execution.{category}"),
            Self::ExecutionAllCategory => "execution".to_string(),
            Self::FastExecution(category) => format!("execution.fast.{category}"),
            Self::FastExecutionAllCategory => "execution.fast".to_string(),
            Self::Order(category) => format!("order.{category}"),
            Self::OrderAllCategory => "order".to_string(),
            Self::Wallet => "wallet".to_string(),
            Self::Greek => "greek".to_string(),
            Self::Dcp(function) => format!("dcp.{function}"),
        };
        write!(f, "{value}")
    }
}

impl Serialize for Topic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Topic {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: std::borrow::Cow<'de, str> = Deserialize::deserialize(deserializer)?;
        let s = s.as_ref();
        if let Some((kind, args)) = s.split_once('.') {
            match kind {
                "orderbook" => {
                    if let Some((depth, symbol)) = args.split_once('.') {
                        let depth =
                            serde_json::from_str(&format!("\"{depth}\"")).map_err(|err| {
                                serde::de::Error::custom(format!(
                                    "DepthLevel: {depth}, error: {err}"
                                ))
                            })?;
                        Ok(Self::Orderbook {
                            depth,
                            symbol: symbol.to_owned(),
                        })
                    } else {
                        Err(serde::de::Error::custom("invalid stream format"))
                    }
                }
                "publicTrade" => Ok(Self::Trade(args.to_owned())),
                "tickers" => Ok(Self::Ticker(args.to_owned())),
                "kline" => {
                    if let Some((interval, symbol)) = args.split_once('.') {
                        let interval = serde_json::from_str(&format!("\"{interval}\""))
                            .map_err(|err| serde::de::Error::custom(format!("Interval: {err}")))?;
                        Ok(Self::Kline {
                            symbol: symbol.to_owned(),
                            interval,
                        })
                    } else {
                        Err(serde::de::Error::custom("invalid stream format"))
                    }
                }
                "allLiquidation" => Ok(Self::AllLiquidation(args.to_owned())),
                "position" => {
                    let category = serde_json::from_str(&format!("\"{args}\""))
                        .map_err(|err| serde::de::Error::custom(format!("category: {err}")))?;
                    Ok(Self::Position(category))
                }
                "execution" => {
                    if args == "fast" {
                        Ok(Self::FastExecutionAllCategory)
                    } else if let Some((fast, category)) = args.split_once('.') {
                        let category = serde_json::from_str(&format!("\"{category}\""))
                            .map_err(|err| serde::de::Error::custom(format!("category: {err}")))?;
                        if fast == "fast" {
                            Ok(Self::FastExecution(category))
                        } else {
                            Ok(Self::Execution(category))
                        }
                    } else {
                        let category = serde_json::from_str(&format!("\"{args}\""))
                            .map_err(|err| serde::de::Error::custom(format!("category: {err}")))?;
                        Ok(Self::Execution(category))
                    }
                }
                "order" => {
                    let category = serde_json::from_str(&format!("\"{args}\""))
                        .map_err(|err| serde::de::Error::custom(format!("category: {err}")))?;
                    Ok(Self::Order(category))
                }
                "dcp" => {
                    let function = serde_json::from_str(&format!("\"{args}\""))
                        .map_err(|err| serde::de::Error::custom(format!("DcpFunction: {err}")))?;
                    Ok(Self::Dcp(function))
                }
                _ => Err(serde::de::Error::custom("invalid stream format")),
            }
        } else {
            match s {
                "position" => Ok(Self::PositionAllCategory),
                "execution" => Ok(Self::ExecutionAllCategory),
                "order" => Ok(Self::OrderAllCategory),
                "wallet" => Ok(Self::Wallet),
                "greek" => Ok(Self::Greek),
                _ => {
                    let msg = String::from("invalid stream format");
                    Err(serde::de::Error::custom(msg))
                }
            }
        }
    }
}

/// Product of a DCP topic (`dcp.{function}`).
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum DcpFunction {
    /// Futures.
    Future,
    /// Options.
    Option,
    /// Spot.
    Spot,
}

impl fmt::Display for DcpFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Future => "future",
            Self::Option => "option",
            Self::Spot => "spot",
        };
        write!(f, "{value}")
    }
}

/// Depth of the order book stream (`orderbook.{depth}.{symbol}`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum DepthLevel {
    /// 1 levels.
    #[serde(rename = "1")]
    Level1,
    /// 25 levels.
    #[serde(rename = "25")]
    Level25,
    /// 50 levels.
    #[serde(rename = "50")]
    Level50,
    /// 100 levels.
    #[serde(rename = "100")]
    Level100,
    /// 200 levels.
    #[serde(rename = "200")]
    Level200,
    /// Perpetuals and futures only.
    #[serde(rename = "500")]
    Level500,
    /// Spot only.
    #[serde(rename = "1000")]
    Level1000,
}

impl fmt::Display for DepthLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Level1 => "1",
            Self::Level25 => "25",
            Self::Level50 => "50",
            Self::Level100 => "100",
            Self::Level200 => "200",
            Self::Level500 => "500",
            Self::Level1000 => "1000",
        };
        write!(f, "{value}")
    }
}

/// Coin in which the fee of a spot trade is charged: normally the coin
/// received; with a negative maker fee rate (rebate), a maker receives the
/// rebate in the coin paid.
pub fn spot_fee_currency(side: Side, is_maker_order: bool, maker_fee_rate: f64) -> Pair {
    if maker_fee_rate >= 0.0 {
        match side {
            Side::Buy => Pair::Base,
            Side::Sell => Pair::Quote,
        }
    } else if is_maker_order {
        match side {
            Side::Buy => Pair::Quote,
            Side::Sell => Pair::Base,
        }
    } else {
        match side {
            Side::Buy => Pair::Base,
            Side::Sell => Pair::Quote,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::serde::deserialize_json;

    use super::*;

    #[test]
    fn serialize_category() {
        let cases = [
            (Category::Inverse, r#""inverse""#),
            (Category::Linear, r#""linear""#),
            (Category::Option, r#""option""#),
            (Category::Spot, r#""spot""#),
        ];
        cases.iter().for_each(|(category, expected)| {
            let json = serde_json::to_string(category).unwrap();
            assert_eq!(json, *expected);
        });
    }

    #[test]
    fn deserialize_category() {
        let cases = [
            (r#""inverse""#, Category::Inverse),
            (r#""linear""#, Category::Linear),
            (r#""option""#, Category::Option),
            (r#""spot""#, Category::Spot),
        ];
        cases.iter().for_each(|(json, expected)| {
            let message: Category = deserialize_json(json).unwrap();
            assert_eq!(message, *expected);
        });
    }

    #[test]
    fn serialize_interval() {
        let cases = vec![
            (Interval::Minute1, r#""1""#),
            (Interval::Minute3, r#""3""#),
            (Interval::Minute5, r#""5""#),
            (Interval::Minute15, r#""15""#),
            (Interval::Minute30, r#""30""#),
            (Interval::Hour1, r#""60""#),
            (Interval::Hour2, r#""120""#),
            (Interval::Hour4, r#""240""#),
            (Interval::Hour6, r#""360""#),
            (Interval::Hour12, r#""720""#),
            (Interval::Day1, r#""D""#),
            (Interval::Week1, r#""W""#),
            (Interval::Month1, r#""M""#),
        ];
        cases.iter().for_each(|(category, expected)| {
            let json = serde_json::to_string(category).unwrap();
            assert_eq!(json, *expected);
        });
    }

    #[test]
    fn deserialize_interval() {
        let cases = vec![
            (r#""1""#, Interval::Minute1),
            (r#""3""#, Interval::Minute3),
            (r#""5""#, Interval::Minute5),
            (r#""15""#, Interval::Minute15),
            (r#""30""#, Interval::Minute30),
            (r#""60""#, Interval::Hour1),
            (r#""120""#, Interval::Hour2),
            (r#""240""#, Interval::Hour4),
            (r#""360""#, Interval::Hour6),
            (r#""720""#, Interval::Hour12),
            (r#""D""#, Interval::Day1),
            (r#""W""#, Interval::Week1),
            (r#""M""#, Interval::Month1),
        ];
        cases.iter().for_each(|(json, expected)| {
            let message: Interval = deserialize_json(json).unwrap();
            assert_eq!(message, *expected);
        });
    }

    #[test]
    fn deserialize_topic_with_escaped_characters() {
        let topic: Topic = serde_json::from_str(r#""tickers.BTC\u0055SDT""#).unwrap();

        assert_eq!(topic, Topic::Ticker(String::from("BTCUSDT")));
    }

    #[test]
    fn serialize_topic() {
        let symbol = String::from("BTCUSDT");
        let cases = vec![
            (
                Topic::Orderbook {
                    symbol: symbol.clone(),
                    depth: DepthLevel::Level1,
                },
                r#""orderbook.1.BTCUSDT""#,
            ),
            (Topic::Trade(symbol.clone()), r#""publicTrade.BTCUSDT""#),
            (Topic::Ticker(symbol.clone()), r#""tickers.BTCUSDT""#),
            (
                Topic::Kline {
                    symbol: symbol.clone(),
                    interval: Interval::Minute1,
                },
                r#""kline.1.BTCUSDT""#,
            ),
            (
                Topic::AllLiquidation(symbol.clone()),
                r#""allLiquidation.BTCUSDT""#,
            ),
            (Topic::Position(Category::Linear), r#""position.linear""#),
            (Topic::PositionAllCategory, r#""position""#),
            (Topic::Execution(Category::Linear), r#""execution.linear""#),
            (Topic::ExecutionAllCategory, r#""execution""#),
            (
                Topic::FastExecution(Category::Linear),
                r#""execution.fast.linear""#,
            ),
            (Topic::FastExecutionAllCategory, r#""execution.fast""#),
            (Topic::Order(Category::Linear), r#""order.linear""#),
            (Topic::OrderAllCategory, r#""order""#),
            (Topic::Wallet, r#""wallet""#),
            (Topic::Greek, r#""greek""#),
            (Topic::Dcp(DcpFunction::Future), r#""dcp.future""#),
        ];
        cases.iter().for_each(|(category, expected)| {
            let json = serde_json::to_string(category).unwrap();
            assert_eq!(*expected, json);
        });
    }

    #[test]
    fn deserialize_topic() {
        let symbol = String::from("BTCUSDT");
        let cases = vec![
            (
                r#""orderbook.1.BTCUSDT""#,
                Topic::Orderbook {
                    symbol: symbol.clone(),
                    depth: DepthLevel::Level1,
                },
            ),
            (r#""publicTrade.BTCUSDT""#, Topic::Trade(symbol.clone())),
            (r#""tickers.BTCUSDT""#, Topic::Ticker(symbol.clone())),
            (
                r#""kline.1.BTCUSDT""#,
                Topic::Kline {
                    symbol: symbol.clone(),
                    interval: Interval::Minute1,
                },
            ),
            (
                r#""allLiquidation.BTCUSDT""#,
                Topic::AllLiquidation(symbol.clone()),
            ),
            (r#""position.linear""#, Topic::Position(Category::Linear)),
            (r#""position""#, Topic::PositionAllCategory),
            (r#""execution.linear""#, Topic::Execution(Category::Linear)),
            (r#""execution""#, Topic::ExecutionAllCategory),
            (
                r#""execution.fast.linear""#,
                Topic::FastExecution(Category::Linear),
            ),
            (r#""execution.fast""#, Topic::FastExecutionAllCategory),
            (r#""order.linear""#, Topic::Order(Category::Linear)),
            (r#""order""#, Topic::OrderAllCategory),
            (r#""wallet""#, Topic::Wallet),
            (r#""greek""#, Topic::Greek),
            (r#""dcp.future""#, Topic::Dcp(DcpFunction::Future)),
        ];
        cases.iter().for_each(|(json, expected)| {
            let message = deserialize_json(json).unwrap();
            assert_eq!(*expected, message);
        });
    }
}

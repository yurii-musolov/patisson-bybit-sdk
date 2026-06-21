//! Enums Definitions
//!
//! Ref: https://bybit-exchange.github.io/docs/v5/enum

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::*;
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub enum Locale {
    #[serde(rename = "de-DE")]
    DeDe,
    #[serde(rename = "en-US")]
    EnUs,
    #[serde(rename = "es-AR")]
    EsAr,
    #[serde(rename = "es-ES")]
    EsEs,
    #[serde(rename = "es-MX")]
    EsMx,
    #[serde(rename = "fr-FR")]
    FrFr,
    #[serde(rename = "kk-KZ")]
    KkKz,
    #[serde(rename = "id-ID")]
    IdId,
    #[serde(rename = "uk-UA")]
    UkUa,
    #[serde(rename = "ja-JP")]
    JaJp,
    #[serde(rename = "ru-RU")]
    RuRu,
    #[serde(rename = "th-TH")]
    ThTh,
    #[serde(rename = "pt-BR")]
    PtBr,
    #[serde(rename = "tr-TR")]
    TrTr,
    #[serde(rename = "vi-VN")]
    ViVn,
    #[serde(rename = "zh-TW")]
    ZhTw,
    #[serde(rename = "ar-SA")]
    ArSa,
    #[serde(rename = "hi-IN")]
    HiIn,
    #[serde(rename = "fil-PH")]
    FilPh,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AnnouncementType {
    #[serde(rename = "new_crypto")]
    NewCrypto,
    #[serde(rename = "latest_bybit_news")]
    LatestBybitNews,
    #[serde(rename = "delistings")]
    Delistings,
    #[serde(rename = "latest_activities")]
    LatestActivities,
    #[serde(rename = "product_updates")]
    ProductUpdates,
    #[serde(rename = "maintenance_updates")]
    MaintenanceUpdates,
    #[serde(rename = "new_fiat_listings")]
    NewFiatListings,
    #[serde(rename = "other")]
    Other,
}

/// Unified Account: spot | linear | inverse | option
/// Classic Account: linear | inverse | spot
#[derive(PartialEq, Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    /// Inverse contract, including Inverse perp, Inverse futures.
    Inverse,
    /// USDT perpetual, and USDC contract, including USDC perp, USDC futures.
    Linear,
    Option,
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

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub enum OrderStatus {
    // open status
    /// order has been placed successfully
    New,
    PartiallyFilled,
    /// Conditional orders are created
    Untriggered,
    // closed status
    Rejected,
    /// Only spot has this order status
    PartiallyFilledCanceled,
    Filled,
    /// In derivatives, orders with this status may have an executed qty
    Cancelled,
    /// instantaneous state for conditional orders from Untriggered to New
    Triggered,
    /// UTA: Spot tp/sl order, conditional order, OCO order are cancelled before they are triggered
    Deactivated,
}

impl OrderStatus {
    pub fn is_open(&self) -> bool {
        matches!(self, Self::New | Self::PartiallyFilled | Self::Untriggered)
    }
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum TimeInForce {
    /// GoodTillCancel
    GTC,
    /// ImmediateOrCancel
    IOC,
    /// FillOrKill
    FOK,
    PostOnly,
    /// features:
    /// Exclusive Matching: Only match non-algorithmic users; no execution against orders from Open API.
    /// Post-Only Mechanism: Act as maker orders, adding liquidity
    /// Lower Priority: Execute after non-RPI orders at the same price level.
    /// Limited Access: Initially for select market makers across multiple pairs.
    /// Order Book Updates: Excluded from API but displayed on the GUI.
    RPI,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum CreateType {
    CreateByUser,
    /// Spread order
    CreateByFutureSpread,
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum ExecType {
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
    BlockTrade,
    MovePosition,
    /// Spread leg execution
    FutureSpread,
    /// May be returned by a classic account. Cannot query by this type
    UNKNOWN,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum OrderType {
    Market,
    Limit,
    /// is not a valid request parameter value. Is only used in some responses. Mainly, it is used when execType is Funding.
    UNKNOWN,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum StopOrderType {
    TakeProfit,
    StopLoss,
    TrailingStop,
    Stop,
    PartialTakeProfit,
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
    UNKNOWN,
}

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

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub enum Interval {
    #[serde(rename = "1")]
    Minute1,
    #[serde(rename = "3")]
    Minute3,
    #[serde(rename = "5")]
    Minute5,
    #[serde(rename = "15")]
    Minute15,
    #[serde(rename = "30")]
    Minute30,
    #[serde(rename = "60")]
    Hour1,
    #[serde(rename = "120")]
    Hour2,
    #[serde(rename = "240")]
    Hour4,
    #[serde(rename = "360")]
    Hour6,
    #[serde(rename = "720")]
    Hour12,
    #[serde(rename = "D")]
    Day1,
    #[serde(rename = "W")]
    Week1,
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

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub enum IntervalTime {
    #[serde(rename = "5min")]
    Minute5,
    #[serde(rename = "15min")]
    Minute15,
    #[serde(rename = "30min")]
    Minute30,
    #[serde(rename = "1h")]
    Hour1,
    #[serde(rename = "4h")]
    Hour4,
    #[serde(rename = "1d")]
    Day1,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum PositionIdx {
    /// 0:one-way mode position
    OneWay = 0,
    /// 1:Buy side of hedge-mode position
    Buy = 1,
    /// 2:Sell side of hedge-mode position
    Sell = 2,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum PositionMode {
    /// 0: Merged Single (one-way)
    OneWay = 0,
    /// 3: Both Sides (hedge)
    Hedge = 3,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum PositionStatus {
    Normal,
    /// in the liquidation progress
    Liq,
    /// in the auto-deleverage progress
    Adl,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum RejectReason {
    #[serde(rename = "EC_NoError")]
    EcNoError,
    #[serde(rename = "EC_Others")]
    EcOthers,
    #[serde(rename = "EC_UnknownMessageType")]
    EcUnknownMessageType,
    #[serde(rename = "EC_MissingClOrdID")]
    EcMissingClOrdId,
    #[serde(rename = "EC_MissingOrigClOrdID")]
    EcMissingOrigClOrdId,
    #[serde(rename = "EC_ClOrdIDOrigClOrdIDAreTheSame")]
    EcClOrdIdorigClOrdIdareTheSame,
    #[serde(rename = "EC_DuplicatedClOrdID")]
    EcDuplicatedClOrdId,
    #[serde(rename = "EC_OrigClOrdIDDoesNotExist")]
    EcOrigClOrdIddoesNotExist,
    #[serde(rename = "EC_TooLateToCancel")]
    EcTooLateToCancel,
    #[serde(rename = "EC_UnknownOrderType")]
    EcUnknownOrderType,
    #[serde(rename = "EC_UnknownSide")]
    EcUnknownSide,
    #[serde(rename = "EC_UnknownTimeInForce")]
    EcUnknownTimeInForce,
    #[serde(rename = "EC_WronglyRouted")]
    EcWronglyRouted,
    #[serde(rename = "EC_MarketOrderPriceIsNotZero")]
    EcMarketOrderPriceIsNotZero,
    #[serde(rename = "EC_LimitOrderInvalidPrice")]
    EcLimitOrderInvalidPrice,
    #[serde(rename = "EC_NoEnoughQtyToFill")]
    EcNoEnoughQtyToFill,
    #[serde(rename = "EC_NoImmediateQtyToFill")]
    EcNoImmediateQtyToFill,
    #[serde(rename = "EC_PerCancelRequest")]
    EcPerCancelRequest,
    #[serde(rename = "EC_MarketOrderCannotBePostOnly")]
    EcMarketOrderCannotBePostOnly,
    #[serde(rename = "EC_PostOnlyWillTakeLiquidity")]
    EcPostOnlyWillTakeLiquidity,
    #[serde(rename = "EC_CancelReplaceOrder")]
    EcCancelReplaceOrder,
    #[serde(rename = "EC_InvalidSymbolStatus")]
    EcInvalidSymbolStatus,
    #[serde(rename = "EC_CancelForNoFullFill")]
    EcCancelForNoFullFill,
    #[serde(rename = "EC_BySelfMatch")]
    EcBySelfMatch,
    /// used for pre-market order operation, e.g., during 2nd phase of call auction, cancel order is not allowed, when the cancel request is failed to be rejected by trading server, the request will be rejected by matching box finally
    #[serde(rename = "EC_InCallAuctionStatus")]
    EcInCallAuctionStatus,
    #[serde(rename = "EC_QtyCannotBeZero")]
    EcQtyCannotBeZero,
    #[serde(rename = "EC_MarketOrderNoSupportTIF")]
    EcMarketOrderNoSupportTif,
    #[serde(rename = "EC_ReachMaxTradeNum")]
    EcReachMaxTradeNum,
    #[serde(rename = "EC_InvalidPriceScale")]
    EcInvalidPriceScale,
    #[serde(rename = "EC_BitIndexInvalid")]
    EcBitIndexInvalid,
    #[serde(rename = "EC_StopBySelfMatch")]
    EcStopBySelfMatch,
    #[serde(rename = "EC_InvalidSmpType")]
    EcInvalidSmpType,
    #[serde(rename = "EC_CancelByMMP")]
    EcCancelByMmp,
    #[serde(rename = "EC_InvalidUserType")]
    EcInvalidUserType,
    #[serde(rename = "EC_InvalidMirrorOid")]
    EcInvalidMirrorOid,
    #[serde(rename = "EC_InvalidMirrorUid")]
    EcInvalidMirrorUid,
    #[serde(rename = "EC_EcInvalidQty")]
    EcEcInvalidQty,
    #[serde(rename = "EC_InvalidAmount")]
    EcInvalidAmount,
    #[serde(rename = "EC_LoadOrderCancel")]
    EcLoadOrderCancel,
    #[serde(rename = "EC_MarketQuoteNoSuppSell")]
    EcMarketQuoteNoSuppSell,
    #[serde(rename = "EC_DisorderOrderID")]
    EcDisorderOrderId,
    #[serde(rename = "EC_InvalidBaseValue")]
    EcInvalidBaseValue,
    #[serde(rename = "EC_LoadOrderCanMatch")]
    EcLoadOrderCanMatch,
    #[serde(rename = "EC_SecurityStatusFail")]
    EcSecurityStatusFail,
    #[serde(rename = "EC_ReachRiskPriceLimit")]
    EcReachRiskPriceLimit,
    #[serde(rename = "EC_OrderNotExist")]
    EcOrderNotExist,
    #[serde(rename = "EC_CancelByOrderValueZero")]
    EcCancelByOrderValueZero,
    #[serde(rename = "EC_CancelByMatchValueZero")]
    EcCancelByMatchValueZero,
    #[serde(rename = "EC_ReachMarketPriceLimit")]
    EcReachMarketPriceLimit,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
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
    pub fn is_uta_2(&self) -> bool {
        matches!(self, Self::UNIFIED | Self::FUND)
    }
    pub fn is_uta_1(&self) -> bool {
        matches!(self, Self::CONTRACT | Self::UNIFIED | Self::FUND)
    }
    pub fn is_classic(&self) -> bool {
        matches!(self, Self::SPOT | Self::CONTRACT | Self::FUND)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum TransferStatus {
    SUCCESS,
    PENDING,
    FAILED,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum DepositStatus {
    #[serde(rename = "0")]
    Unknown,
    #[serde(rename = "1")]
    ToBeConfirmed,
    #[serde(rename = "2")]
    Processing,
    /// (finalised status of a success deposit)
    #[serde(rename = "3")]
    Success,
    #[serde(rename = "4")]
    DepositFailed,
    #[serde(rename = "10011")]
    PendingToBeCreditedToFundingPool,
    #[serde(rename = "10012")]
    CreditedToFundingPoolSuccessfully,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum WithdrawStatus {
    SecurityCheck,
    Pending,
    #[serde(rename = "success")]
    Success,
    CancelByUser,
    Reject,
    Fail,
    BlockchainConfirmed,
    MoreInformationRequired,
    /// a rare status
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum TriggerBy {
    LastPrice,
    IndexPrice,
    MarkPrice,
    UNKNOWN,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum CancelType {
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
    CancelByCannotAffordOrderCost,
    CancelByPmTrialMmOverEquity,
    CancelByAccountBlocking,
    CancelByDelivery,
    CancelByMmpTriggered,
    CancelByCrossSelfMuch,
    CancelByCrossReachMaxTradeNum,

    /// Not documented
    UNKNOWN,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum OptionPeriod {
    #[serde(rename = "7")]
    Day7,
    #[serde(rename = "14")]
    Day14,
    #[serde(rename = "21")]
    Day21,
    #[serde(rename = "30")]
    Day30,
    #[serde(rename = "60")]
    Day60,
    #[serde(rename = "90")]
    Day90,
    #[serde(rename = "180")]
    Day180,
    #[serde(rename = "270")]
    Day270,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum DataRecordingPeriod {
    #[serde(rename = "5min")]
    Minute5,
    #[serde(rename = "15min")]
    Minute15,
    #[serde(rename = "30min")]
    Minute30,
    #[serde(rename = "1h")]
    Hour1,
    #[serde(rename = "4h")]
    Hour4,
    #[serde(rename = "4d")]
    Day4,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub enum ContractType {
    InversePerpetual,
    LinearPerpetual,
    /// USDT/USDC Futures
    LinearFutures,
    InverseFutures,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub enum Status {
    PreLaunch,
    Trading,
    Delivering,
    Closed,
}

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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    /// Assets that transferred into Unified | (inverse) derivatives wallet
    TransferIn,
    /// Assets that transferred out from Unified | (inverse) derivatives wallet
    TransferOut,
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
    BorrowedAmountInsLoan,
    PrincipleRepaymentInsLoan,
    InterestRepaymentInsLoan,
    /// the liquidation for borrowing asset(INS loan)
    AutoSoldCollateralInsLoan,
    /// the liquidation for borrowing asset(INS loan)
    AutoBuyLiabilityInsLoan,
    AutoPrincipleRepaymentInsLoan,
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
    PremarketTransferOut,
    PremarketDeliverySellNewCoin,
    PremarketDeliveryBuyNewCoin,
    PremarketDeliveryPledgePaySeller,
    PremarketDeliveryPledgeBack,
    PremarketRollbackPledgeBack,
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
    BonusTransferIn,
    BonusTransferOut,
    PefTransferIn,
    PefTransferOut,
    PefProfitShare,
    #[serde(rename = "Others")]
    Others,
}

impl Type {
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

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum UnifiedMarginStatus {
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarginMode {
    IsolatedMargin,
    RegularMargin,
    PortfolioMargin,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SpotHedgingStatus {
    On,
    Off,
}

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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum VipLevel {
    #[serde(rename = "No VIP")]
    NoVIP,
    #[serde(rename = "VIP-1")]
    VIP1,
    #[serde(rename = "VIP-2")]
    VIP2,
    #[serde(rename = "VIP-3")]
    VIP3,
    #[serde(rename = "VIP-4")]
    VIP4,
    #[serde(rename = "VIP-5")]
    VIP5,
    #[serde(rename = "VIP-Supreme")]
    VIPSupreme,
    #[serde(rename = "PRO-1")]
    PRO1,
    #[serde(rename = "PRO-2")]
    PRO2,
    #[serde(rename = "PRO-3")]
    PRO3,
    #[serde(rename = "PRO-4")]
    PRO4,
    #[serde(rename = "PRO-5")]
    PRO5,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum AdlRankIndicator {
    /// default value of empty position
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum SmpType {
    /// default
    None,
    CancelMaker,
    CancelTaker,
    CancelBoth,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExtraFeeType {
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExtraSubFeeType {
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

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum State {
    Scheduled,
    Ongoing,
    Completed,
    Canceled,
}

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

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Product {
    Futures = 1,
    Spot = 2,
    Option = 3,
    Spread = 4,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DCPProduct {
    Spot,
    Derivatives,
    Option,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum MaintainType {
    PlannedMaintenance = 1,
    TemporaryMaintenance = 2,
    Incident = 3,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Env {
    Product = 1,
    ProductDemoService = 2,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum TpslMode {
    Full,
    Partial,
    UNKNOWN,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum OcoTriggerBy {
    #[serde(rename = "OcoTriggerByUnknown")]
    Unknown,
    #[serde(rename = "OcoTriggerByTp")]
    Tp,
    #[serde(rename = "OcoTriggerByBySl")]
    BySl,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum TriggerDirection {
    UNKNOWN = 0,
    Rise = 1,
    Fall = 2,
}

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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum PlaceType {
    #[serde(rename = "option")]
    Option,
    #[serde(rename = "iv")]
    Iv,
    #[serde(rename = "price")]
    Price,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub enum Side {
    Buy,
    Sell,
}
impl Side {
    pub fn reverse(&self) -> Self {
        match self {
            Side::Buy => Self::Sell,
            Side::Sell => Self::Buy,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Pair {
    // example of BTCUSDT
    Base,  // BTC
    Quote, // USDT
}

#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
pub enum SlippageToleranceType {
    TickSize,
    Percent,
    /// default
    UNKNOWN,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum TradeMode {
    CrossMargin = 0,
    IsolatedMargin = 1,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Topic {
    Orderbook {
        symbol: String,
        depth: DepthLevel,
    },
    Trade(String),
    Ticker(String),
    Kline {
        symbol: String,
        interval: Interval,
    },
    AllLiquidation(String),
    Position(Category),
    PositionAllCategory,
    Execution(Category),
    ExecutionAllCategory,
    FastExecution(Category),
    FastExecutionAllCategory,
    Order(Category),
    OrderAllCategory,
    Wallet,
    /// option only.
    Greek,
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
        let s = match self {
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
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Topic {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
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

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum DcpFunction {
    Future,
    Option,
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum DepthLevel {
    #[serde(rename = "1")]
    Level1,
    #[serde(rename = "25")]
    Level25,
    #[serde(rename = "50")]
    Level50,
    #[serde(rename = "100")]
    Level100,
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
        let cases = vec![
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
        let cases = vec![
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

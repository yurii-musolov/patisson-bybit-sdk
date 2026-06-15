mod account;
mod client;
mod common;
mod market;
mod orders;
mod positions;
mod user;

pub use account::*;
pub use client::*;
pub use common::*;
pub use market::*;
pub use orders::*;
pub use positions::*;
pub use user::*;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rust_decimal::dec;

    use crate::{
        AccountType, AdlRankIndicator, CancelType, CreateType, MarginMode, OrderStatus, OrderType,
        PositionIdx, PositionStatus, RejectReason, Side, SmpType, SpotHedgingStatus, TimeInForce,
        TpslMode, TriggerBy, TriggerDirection, UnifiedMarginStatus,
        enums::{Category, StopOrderType},
        serde::{Unique, deserialize_json},
    };

    use super::*;

    #[test]
    fn deserialize_response_kline_inverse() {
        let json = r#"{
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "symbol": "BTCUSD",
                "category": "inverse",
                "list": [
                    [
                        "1670608800000",
                        "17071",
                        "17073",
                        "17027",
                        "17055.5",
                        "268611",
                        "15.74462667"
                    ],
                    [
                        "1670605200000",
                        "17071.5",
                        "17071.5",
                        "17061",
                        "17071",
                        "4177",
                        "0.24469757"
                    ],
                    [
                        "1670601600000",
                        "17086.5",
                        "17088",
                        "16978",
                        "17071.5",
                        "6356",
                        "0.37288112"
                    ]
                ]
            },
            "retExtInfo": {},
            "time": 1672025956592
        }"#;
        let expected = Resp {
            ret_code: 0,
            ret_msg: String::from("OK"),
            result: KLine::Inverse {
                symbol: String::from("BTCUSD"),
                list: vec![
                    KLineRow {
                        start_time: 1670608800000,
                        open_price: dec!(17071),
                        high_price: dec!(17073),
                        low_price: dec!(17027),
                        close_price: dec!(17055.5),
                        volume: dec!(268611),
                        turnover: dec!(15.74462667),
                    },
                    KLineRow {
                        start_time: 1670605200000,
                        open_price: dec!(17071.5),
                        high_price: dec!(17071.5),
                        low_price: dec!(17061),
                        close_price: dec!(17071),
                        volume: dec!(4177),
                        turnover: dec!(0.24469757),
                    },
                    KLineRow {
                        start_time: 1670601600000,
                        open_price: dec!(17086.5),
                        high_price: dec!(17088),
                        low_price: dec!(16978),
                        close_price: dec!(17071.5),
                        volume: dec!(6356),
                        turnover: dec!(0.37288112),
                    },
                ],
            },
            time: Some(1672025956592),
            ret_ext_info: Some(RetExtInfo::default()),
        };

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn deserialize_response_ticker_inverse() {
        let json = r#"{
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "category": "inverse",
                "list": [
                    {
                        "symbol": "BTCUSD",
                        "lastPrice": "16597.00",
                        "indexPrice": "16598.54",
                        "markPrice": "16596.00",
                        "prevPrice24h": "16464.50",
                        "price24hPcnt": "0.008047",
                        "highPrice24h": "30912.50",
                        "lowPrice24h": "15700.00",
                        "prevPrice1h": "16595.50",
                        "openInterest": "373504107",
                        "openInterestValue": "22505.67",
                        "turnover24h": "2352.94950046",
                        "volume24h": "49337318",
                        "fundingRate": "-0.001034",
                        "nextFundingTime": "1672387200000",
                        "predictedDeliveryPrice": "",
                        "basisRate": "",
                        "deliveryFeeRate": "",
                        "deliveryTime": "0",
                        "ask1Size": "1",
                        "bid1Price": "16596.00",
                        "ask1Price": "16597.50",
                        "bid1Size": "1",
                        "basis": ""
                    }
                ]
            },
            "retExtInfo": {},
            "time": 1672376496682
        }"#;
        let expected = Resp {
            ret_code: 0,
            ret_msg: String::from("OK"),
            result: Ticker::Inverse {
                list: vec![LinearInverseTicker {
                    symbol: String::from("BTCUSD"),
                    last_price: dec!(16597.00),
                    mark_price: dec!(16596.00),
                    index_price: dec!(16598.54),
                    prev_price24h: dec!(16464.50),
                    price24h_pcnt: dec!(0.008047),
                    high_price24h: dec!(30912.50),
                    low_price24h: dec!(15700.00),
                    prev_price1h: dec!(16595.50),
                    open_interest: dec!(373504107),
                    open_interest_value: dec!(22505.67),
                    turnover24h: dec!(2352.94950046),
                    volume24h: dec!(49337318),
                    funding_rate: Some(dec!(-0.001034)),
                    next_funding_time: 1672387200000,
                    predicted_delivery_price: None,
                    basis_rate: None,
                    basis: None,
                    delivery_fee_rate: None,
                    delivery_time: Some(0),
                    bid1_price: dec!(16596.00),
                    bid1_size: dec!(1),
                    ask1_price: dec!(16597.50),
                    ask1_size: dec!(1),
                    pre_open_price: None,
                    pre_qty: None,
                    cur_pre_listing_phase: None,
                }],
            },
            time: Some(1672376496682),
            ret_ext_info: Some(RetExtInfo::default()),
        };

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn deserialize_response_trad_spot() {
        let json = r#"{
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "category": "spot",
                "list": [
                    {
                        "execId": "2100000000007764263",
                        "symbol": "BTCUSDT",
                        "price": "16618.49",
                        "size": "0.00012",
                        "side": "Buy",
                        "time": "1672052955758",
                        "isBlockTrade": false,
                        "isRPITrade": true
                    }
                ]
            },
            "retExtInfo": {},
            "time": 1672053054358
        }"#;
        let expected = Resp {
            ret_code: 0,
            ret_msg: String::from("OK"),
            result: Trade::Spot {
                list: vec![InverseLinearSpotTrade {
                    exec_id: String::from("2100000000007764263"),
                    symbol: String::from("BTCUSDT"),
                    price: dec!(16618.49),
                    size: dec!(0.00012),
                    side: Side::Buy,
                    time: 1672052955758,
                    is_block_trade: false,
                    is_rpi_trade: true,
                }],
            },
            time: Some(1672053054358),
            ret_ext_info: Some(RetExtInfo::default()),
        };

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn deserialize_response_orderbook_linear() {
        let json = r#"{
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "s": "BTCUSDT",
                "b": [
                    ["30190.65", "0.165956"],
                    ["30190.10", "0.020000"]
                ],
                "a": [
                    ["30201.42", "1.038467"],
                    ["30201.50", "0.500000"]
                ],
                "ts": 1675418560614,
                "u": 177400507,
                "seq": 66544703342
            },
            "retExtInfo": {},
            "time": 1675418560633
        }"#;
        let expected = Resp {
            ret_code: 0,
            ret_msg: String::from("OK"),
            result: Orderbook {
                symbol: String::from("BTCUSDT"),
                bids: vec![
                    OrderbookLevel {
                        price: dec!(30190.65),
                        size: dec!(0.165956),
                    },
                    OrderbookLevel {
                        price: dec!(30190.10),
                        size: dec!(0.020000),
                    },
                ],
                asks: vec![
                    OrderbookLevel {
                        price: dec!(30201.42),
                        size: dec!(1.038467),
                    },
                    OrderbookLevel {
                        price: dec!(30201.50),
                        size: dec!(0.500000),
                    },
                ],
                ts: 1675418560614,
                update_id: 177400507,
                seq: 66544703342,
                cts: None,
            },
            time: Some(1675418560633),
            ret_ext_info: Some(RetExtInfo::default()),
        };

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn deserialize_response_get_open_closed_orders_linear() {
        let json = r#"{
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "list": [
                    {
                        "orderId": "fd4300ae-7847-404e-b947-b46980a4d140",
                        "orderLinkId": "test-000005",
                        "blockTradeId": "",
                        "symbol": "ETHUSDT",
                        "price": "1600.00",
                        "qty": "0.10",
                        "side": "Buy",
                        "isLeverage": "",
                        "positionIdx": 1,
                        "orderStatus": "New",
                        "cancelType": "UNKNOWN",
                        "rejectReason": "EC_NoError",
                        "avgPrice": "0",
                        "leavesQty": "0.10",
                        "leavesValue": "160",
                        "cumExecQty": "0.00",
                        "cumExecValue": "0",
                        "cumExecFee": "0",
                        "timeInForce": "GTC",
                        "orderType": "Limit",
                        "stopOrderType": "UNKNOWN",
                        "orderIv": "",
                        "triggerPrice": "0.00",
                        "takeProfit": "2500.00",
                        "stopLoss": "1500.00",
                        "tpTriggerBy": "LastPrice",
                        "slTriggerBy": "LastPrice",
                        "triggerDirection": 0,
                        "triggerBy": "UNKNOWN",
                        "lastPriceOnCreated": "",
                        "reduceOnly": false,
                        "closeOnTrigger": false,
                        "smpType": "None",
                        "smpGroup": 0,
                        "smpOrderId": "",
                        "tpslMode": "Full",
                        "tpLimitPrice": "",
                        "slLimitPrice": "",
                        "placeType": "",
                        "createdTime": "1684738540559",
                        "updatedTime": "1684738540561"
                    }
                ],
                "nextPageCursor": "page_args%3Dfd4300ae-7847-404e-b947-b46980a4d140%26symbol%3D6%26",
                "category": "linear"
            },
            "retExtInfo": {},
            "time": 1684765770483
        }"#;
        let expected = Resp {
            ret_code: 0,
            ret_msg: String::from("OK"),
            result: CursorPagination {
                category: Some(Category::Linear),
                next_page_cursor: Some(String::from(
                    "page_args%3Dfd4300ae-7847-404e-b947-b46980a4d140%26symbol%3D6%26",
                )),
                list: vec![Order {
                    order_id: String::from("fd4300ae-7847-404e-b947-b46980a4d140"),
                    order_link_id: Some(String::from("test-000005")),
                    block_trade_id: None,
                    symbol: String::from("ETHUSDT"),
                    price: dec!(1600.00),
                    qty: dec!(0.10),
                    side: Side::Buy,
                    is_leverage: None,
                    position_idx: PositionIdx::Buy,
                    order_status: OrderStatus::New,
                    create_type: None,
                    cancel_type: CancelType::UNKNOWN,
                    reject_reason: RejectReason::EcNoError,
                    avg_price: Some(dec!(0.0)),
                    leaves_qty: dec!(0.10),
                    leaves_value: dec!(160),
                    cum_exec_qty: dec!(0.00),
                    cum_exec_value: dec!(0),
                    cum_exec_fee: dec!(0),
                    time_in_force: TimeInForce::GTC,
                    order_type: OrderType::Limit,
                    stop_order_type: Some(StopOrderType::UNKNOWN),
                    order_iv: None,
                    market_unit: None,
                    trigger_price: Some(dec!(0.00)),
                    take_profit: Some(dec!(2500.00)),
                    stop_loss: Some(dec!(1500.00)),
                    tpsl_mode: Some(TpslMode::Full),
                    oco_trigger_by: None,
                    tp_limit_price: None,
                    sl_limit_price: None,
                    tp_trigger_by: Some(TriggerBy::LastPrice),
                    sl_trigger_by: Some(TriggerBy::LastPrice),
                    trigger_direction: TriggerDirection::UNKNOWN,
                    trigger_by: Some(TriggerBy::UNKNOWN),
                    last_price_on_created: None,
                    base_price: None,
                    reduce_only: false,
                    close_on_trigger: false,
                    place_type: None,
                    smp_type: SmpType::None,
                    smp_group: 0,
                    smp_order_id: None,
                    created_time: 1684738540559,
                    updated_time: 1684738540561,
                }],
            },
            time: Some(1684765770483),
            ret_ext_info: Some(RetExtInfo::default()),
        };

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn deserialize_response_get_open_closed_orders_linear2() {
        let json = r#"{
            "retCode":0,
            "retMsg":"OK",
            "result":{
                "nextPageCursor":"aed77e97-492f-45be-8ada-4ff350ec07a5%3A1762701687113%2Caed77e97-492f-45be-8ada-4ff350ec07a5%3A1762701687113",
                "category":"linear",
                "list":[
                    {
                        "symbol":"BTCUSDT",
                        "orderType":"Limit",
                        "orderLinkId":"",
                        "slLimitPrice":"0",
                        "orderId":"aed77e97-492f-45be-8ada-4ff350ec07a5",
                        "cancelType":"UNKNOWN",
                        "avgPrice":"",
                        "stopOrderType":"",
                        "lastPriceOnCreated":"103550",
                        "orderStatus":"New",
                        "createType":"CreateByUser",
                        "takeProfit":"",
                        "cumExecValue":"0",
                        "tpslMode":"",
                        "smpType":"None",
                        "triggerDirection":0,
                        "blockTradeId":"",
                        "isLeverage":"",
                        "rejectReason":"EC_NoError",
                        "price":"103000",
                        "orderIv":"",
                        "createdTime":"1762701687113",
                        "tpTriggerBy":"",
                        "positionIdx":1,
                        "timeInForce":"GTC",
                        "leavesValue":"1030",
                        "updatedTime":"1762701687113",
                        "side":"Buy",
                        "smpGroup":0,
                        "triggerPrice":"",
                        "tpLimitPrice":"0",
                        "cumExecFee":"0",
                        "leavesQty":"0.01",
                        "slTriggerBy":"",
                        "closeOnTrigger":false,
                        "placeType":"",
                        "cumExecQty":"0",
                        "reduceOnly":false,
                        "qty":"0.01",
                        "stopLoss":"",
                        "marketUnit":"",
                        "smpOrderId":"",
                        "triggerBy":""
                    }
                ]
            },
            "retExtInfo":{},
            "time":1762711342768
        }"#;
        let expected = Resp {
            ret_code: 0,
            ret_msg: String::from("OK"),
            result: CursorPagination {
                category: Some(Category::Linear),
                next_page_cursor: Some(String::from(
                    "aed77e97-492f-45be-8ada-4ff350ec07a5%3A1762701687113%2Caed77e97-492f-45be-8ada-4ff350ec07a5%3A1762701687113",
                )),
                list: vec![Order {
                    order_id: String::from("aed77e97-492f-45be-8ada-4ff350ec07a5"),
                    order_link_id: None,
                    block_trade_id: None,
                    symbol: String::from("BTCUSDT"),
                    price: dec!(103000),
                    qty: dec!(0.01),
                    side: Side::Buy,
                    is_leverage: None,
                    position_idx: PositionIdx::Buy,
                    order_status: OrderStatus::New,
                    create_type: Some(CreateType::CreateByUser),
                    cancel_type: CancelType::UNKNOWN,
                    reject_reason: RejectReason::EcNoError,
                    avg_price: None,
                    leaves_qty: dec!(0.01),
                    leaves_value: dec!(1030),
                    cum_exec_qty: dec!(0),
                    cum_exec_value: dec!(0),
                    cum_exec_fee: dec!(0),
                    time_in_force: TimeInForce::GTC,
                    order_type: OrderType::Limit,
                    stop_order_type: None,
                    order_iv: None,
                    market_unit: None,
                    trigger_price: None,
                    take_profit: None,
                    stop_loss: None,
                    tpsl_mode: None,
                    oco_trigger_by: None,
                    tp_limit_price: Some(dec!(0)),
                    sl_limit_price: Some(dec!(0)),
                    tp_trigger_by: None,
                    sl_trigger_by: None,
                    trigger_direction: TriggerDirection::UNKNOWN,
                    trigger_by: None,
                    last_price_on_created: Some(dec!(103550)),
                    base_price: None,
                    reduce_only: false,
                    close_on_trigger: false,
                    place_type: None,
                    smp_type: SmpType::None,
                    smp_group: 0,
                    smp_order_id: None,
                    created_time: 1762701687113,
                    updated_time: 1762701687113,
                }],
            },
            time: Some(1762711342768),
            ret_ext_info: Some(RetExtInfo::default()),
        };

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn deserialize_response_get_position_info_inverse() {
        let json = r#"{
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "list": [
                    {
                        "positionIdx": 0,
                        "riskId": 1,
                        "riskLimitValue": "150",
                        "symbol": "BTCUSD",
                        "side": "Sell",
                        "size": "300",
                        "avgPrice": "27464.50441675",
                        "positionValue": "0.01092319",
                        "tradeMode": 0,
                        "positionStatus": "Normal",
                        "autoAddMargin": 1,
                        "adlRankIndicator": 2,
                        "leverage": "10",
                        "positionBalance": "0.00139186",
                        "markPrice": "28224.50",
                        "liqPrice": "",
                        "bustPrice": "999999.00",
                        "positionMM": "0.0000015",
                        "positionMMByMp": "0.0000015",
                        "positionIM": "0.00010923",
                        "positionIMByMp": "0.00010923",
                        "tpslMode": "Full",
                        "takeProfit": "0.00",
                        "stopLoss": "0.00",
                        "trailingStop": "0.00",
                        "unrealisedPnl": "-0.00029413",
                        "curRealisedPnl": "0.00013123",
                        "cumRealisedPnl": "-0.00096902",
                        "seq": 5723621632,
                        "isReduceOnly": false,
                        "mmrSysUpdatedTime": "",
                        "leverageSysUpdatedTime": "",
                        "sessionAvgPrice": "",
                        "createdTime": "1676538056258",
                        "updatedTime": "1697673600012"
                    }
                ],
                "nextPageCursor": "",
                "category": "inverse"
            },
            "retExtInfo": {},
            "time": 1697684980172
        }"#;
        let expected = Resp {
            ret_code: 0,
            ret_msg: String::from("OK"),
            result: CursorPagination {
                category: Some(Category::Inverse),
                next_page_cursor: None,
                list: vec![Position {
                    position_idx: PositionIdx::OneWay,
                    risk_id: 1,
                    risk_limit_value: Some(dec!(150)),
                    symbol: String::from("BTCUSD"),
                    side: Some(Side::Sell),
                    size: dec!(300),
                    avg_price: dec!(27464.50441675),
                    position_value: Some(dec!(0.01092319)),
                    auto_add_margin: true,
                    position_status: PositionStatus::Normal,
                    leverage: dec!(10),
                    mark_price: dec!(28224.50),
                    liq_price: None,
                    position_im: Some(dec!(0.00010923)),
                    position_im_by_mp: Some(dec!(0.00010923)),
                    position_mm: Some(dec!(0.0000015)),
                    position_mm_by_mp: Some(dec!(0.0000015)),
                    take_profit: Some(dec!(0.00)),
                    stop_loss: Some(dec!(0.00)),
                    trailing_stop: Some(dec!(0.00)),
                    session_avg_price: None,
                    delta: None,
                    gamma: None,
                    vega: None,
                    theta: None,
                    unrealised_pnl: Some(dec!(-0.00029413)),
                    cur_realised_pnl: dec!(0.00013123),
                    cum_realised_pnl: dec!(-0.00096902),
                    adl_rank_indicator: AdlRankIndicator::Two,
                    created_time: 1676538056258,
                    updated_time: 1697673600012,
                    seq: 5723621632,
                    is_reduce_only: false,
                    mmr_sys_updated_time: None,
                    leverage_sys_updated_time: None,
                }],
            },
            time: Some(1697684980172),
            ret_ext_info: Some(RetExtInfo::default()),
        };

        let message: Resp<CursorPagination<Position>> = deserialize_json(json).unwrap();

        assert_eq!(message, expected);
    }

    #[test]
    fn deserialize_response_get_wallet_balance() {
        let json = r#"{
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "list": [
                    {
                        "totalEquity": "3.31216591",
                        "accountIMRate": "0",
                        "accountIMRateByMp": "0",
                        "totalMarginBalance": "3.00326056",
                        "totalInitialMargin": "0",
                        "totalInitialMarginByMp": "0",
                        "accountType": "UNIFIED",
                        "totalAvailableBalance": "3.00326056",
                        "accountMMRate": "0",
                        "accountMMRateByMp": "0",
                        "totalPerpUPL": "0",
                        "totalWalletBalance": "3.00326056",
                        "accountLTV": "0",
                        "totalMaintenanceMargin": "0",
                        "totalMaintenanceMarginByMp": "0",
                        "coin": [
                            {
                                "availableToBorrow": "3",
                                "bonus": "0",
                                "accruedInterest": "0",
                                "availableToWithdraw": "0",
                                "totalOrderIM": "0",
                                "equity": "0",
                                "totalPositionMM": "0",
                                "usdValue": "0",
                                "spotHedgingQty": "0.01592413",
                                "unrealisedPnl": "0",
                                "collateralSwitch": true,
                                "borrowAmount": "0.0",
                                "totalPositionIM": "0",
                                "walletBalance": "0",
                                "cumRealisedPnl": "0",
                                "locked": "0",
                                "marginCollateral": true,
                                "coin": "BTC",
                                "spotBorrow": "0"
                            }
                        ]
                    }
                ]
            },
            "retExtInfo": {},
            "time": 1690872862481
        }"#;
        let coin = WalletCoin {
            coin: String::from("BTC"),
            equity: dec!(0),
            usd_value: dec!(0),
            wallet_balance: dec!(0),
            locked: dec!(0),
            spot_hedging_qty: dec!(0.01592413),
            borrow_amount: dec!(0.0),
            accrued_interest: dec!(0),
            total_order_im: Some(dec!(0)),
            total_position_im: Some(dec!(0)),
            total_position_mm: Some(dec!(0)),
            unrealised_pnl: dec!(0),
            cum_realised_pnl: dec!(0),
            bonus: dec!(0),
            margin_collateral: true,
            collateral_switch: true,
            spot_borrow: Some(dec!(0)),
        };
        let coin = HashMap::from([(Unique::unique_key(&coin), coin)]);
        let expected = Resp {
            ret_code: 0,
            ret_msg: String::from("OK"),
            result: List {
                list: vec![WalletBalance {
                    account_type: AccountType::UNIFIED,
                    account_im_rate: dec!(0),
                    account_im_rate_by_mp: dec!(0),
                    account_mm_rate: dec!(0),
                    account_mm_rate_by_mp: dec!(0),
                    total_equity: dec!(3.31216591),
                    total_wallet_balance: dec!(3.00326056),
                    total_margin_balance: dec!(3.00326056),
                    total_available_balance: dec!(3.00326056),
                    total_perp_upl: dec!(0),
                    total_initial_margin: dec!(0),
                    total_initial_margin_by_mp: dec!(0),
                    total_maintenance_margin: dec!(0),
                    total_maintenance_margin_by_mp: dec!(0),
                    coin,
                }],
            },
            time: Some(1690872862481),
            ret_ext_info: Some(RetExtInfo::default()),
        };

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn deserialize_response_get_wallet_balance_2() {
        let json = r#"{
            "retCode":0,
            "retMsg":"OK",
            "result":{
                "list":[
                    {
                        "totalEquity":"36.42053792",
                        "accountIMRate":"0",
                        "accountIMRateByMp":"0",
                        "totalMarginBalance":"36.42053792",
                        "totalInitialMargin":"0",
                        "totalInitialMarginByMp":"0",
                        "accountType":"UNIFIED",
                        "totalAvailableBalance":"36.42053792",
                        "accountMMRate":"0",
                        "accountMMRateByMp":"0",
                        "totalPerpUPL":"0",
                        "totalWalletBalance":"36.42053792",
                        "accountLTV":"0",
                        "totalMaintenanceMargin":"0",
                        "totalMaintenanceMarginByMp":"0",
                        "coin":[
                            {
                                "availableToBorrow":"",
                                "bonus":"0",
                                "accruedInterest":"0",
                                "availableToWithdraw":"",
                                "totalOrderIM":"0",
                                "equity":"36.4061211",
                                "totalPositionMM":"0",
                                "usdValue":"36.42053792",
                                "unrealisedPnl":"0",
                                "collateralSwitch":true,
                                "spotHedgingQty":"0",
                                "borrowAmount":"0.000000000000000000",
                                "totalPositionIM":"0",
                                "walletBalance":"36.4061211",
                                "cumRealisedPnl":"-2084.9938789",
                                "locked":"0",
                                "marginCollateral":true,
                                "coin":"USDT",
                                "spotBorrow": "0"
                            }
                        ]
                    }
                ]
            },
            "retExtInfo":{},
            "time":1751570498412
        }"#;
        let coin = WalletCoin {
            coin: String::from("USDT"),
            equity: dec!(36.4061211),
            usd_value: dec!(36.42053792),
            wallet_balance: dec!(36.4061211),
            locked: dec!(0),
            spot_hedging_qty: dec!(0),
            borrow_amount: dec!(0.000000000000000000),
            accrued_interest: dec!(0),
            total_order_im: Some(dec!(0)),
            total_position_im: Some(dec!(0)),
            total_position_mm: Some(dec!(0)),
            unrealised_pnl: dec!(0),
            cum_realised_pnl: dec!(-2084.9938789),
            bonus: dec!(0),
            margin_collateral: true,
            collateral_switch: true,
            spot_borrow: Some(dec!(0)),
        };
        let coin = HashMap::from([(Unique::unique_key(&coin), coin)]);
        let expected = Resp {
            ret_code: 0,
            ret_msg: String::from("OK"),
            result: List {
                list: vec![WalletBalance {
                    account_type: AccountType::UNIFIED,
                    account_im_rate: dec!(0),
                    account_im_rate_by_mp: dec!(0),
                    account_mm_rate: dec!(0),
                    account_mm_rate_by_mp: dec!(0),
                    total_equity: dec!(36.42053792),
                    total_wallet_balance: dec!(36.42053792),
                    total_margin_balance: dec!(36.42053792),
                    total_available_balance: dec!(36.42053792),
                    total_perp_upl: dec!(0),
                    total_initial_margin: dec!(0),
                    total_initial_margin_by_mp: dec!(0),
                    total_maintenance_margin: dec!(0),
                    total_maintenance_margin_by_mp: dec!(0),
                    coin,
                }],
            },
            time: Some(1751570498412),
            ret_ext_info: Some(RetExtInfo::default()),
        };

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn deserialize_response_get_account_info() {
        let json = r#"{
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "marginMode": "REGULAR_MARGIN",
                "updatedTime": "1697078946000",
                "unifiedMarginStatus": 4,
                "dcpStatus": "OFF",
                "timeWindow": 10,
                "smpGroup": 0,
                "isMasterTrader": false,
                "spotHedgingStatus": "OFF"
            }
        }"#;
        let expected = Resp {
            ret_code: 0,
            ret_msg: String::from("OK"),
            result: AccountInfo {
                unified_margin_status: UnifiedMarginStatus::UnifiedTradingAccount1Pro,
                margin_mode: MarginMode::RegularMargin,
                is_master_trader: false,
                spot_hedging_status: SpotHedgingStatus::Off,
                updated_time: 1697078946000,
            },
            time: None,
            ret_ext_info: None,
        };

        let message = deserialize_json(json).unwrap();

        assert_eq!(expected, message);
    }
}

use serde::{Deserialize, Serialize};

use crate::Symbol;

#[derive(Debug, Serialize)]
pub struct PEmpty;

#[derive(Debug, Serialize)]
pub struct PSymbol<'a> {
    pub symbol: &'a Symbol,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PKline<'a> {
    pub symbol: &'a Symbol,
    pub interval: Interval,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Interval {
    #[serde(rename = "1m")]
    Min1,
    #[serde(rename = "3m")]
    Min3,
    #[serde(rename = "5m")]
    Min5,
    #[serde(rename = "15m")]
    Min15,
    #[serde(rename = "30m")]
    Min30,
    #[serde(rename = "1h")]
    Hour1,
    #[serde(rename = "2h")]
    Hour2,
    #[serde(rename = "4h")]
    Hour4,
    #[serde(rename = "6h")]
    Hour6,
    #[serde(rename = "8h")]
    Hour8,
    #[serde(rename = "12h")]
    Hour12,
    #[serde(rename = "1d")]
    Day1,
    #[serde(rename = "3d")]
    Day3,
    #[serde(rename = "1w")]
    Week1,
    #[serde(rename = "1M")]
    Month1,
}

impl ToString for Interval {
    fn to_string(&self) -> String {
        serde_json::to_string(self)
            .unwrap()
            .trim_matches('"')
            .to_string()
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PQuerySpotOrder<'a> {
    pub symbol: &'a Symbol,
    pub order_id: Option<usize>,
    pub orig_client_order_id: Option<String>,
    pub new_client_order_id: Option<String>,
    #[serde(flatten)]
    pub ts: PTimestamp,
}

impl<'a> PQuerySpotOrder<'a> {
    pub fn new(symbol: &'a Symbol) -> Self {
        PQuerySpotOrder {
            symbol,
            order_id: None,
            orig_client_order_id: None,
            new_client_order_id: None,
            ts: PTimestamp::now(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PSpotOrder<'a> {
    #[serde(flatten)]
    pub spec: PSpotOrderSpec<'a>,
    #[serde(flatten)]
    pub ts: PTimestamp,
}

impl<'a> PSpotOrder<'a> {
    pub fn new(symbol: &'a Symbol, side: OrderSide, quantity: f64, price: Option<f64>) -> Self {
        Self {
            spec: PSpotOrderSpec::new(symbol, side, quantity, price),
            ts: PTimestamp::now(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PSpotOrderSpec<'a> {
    pub symbol: &'a Symbol,
    pub side: OrderSide,
    #[serde(rename = "type")]
    pub order_type: SpotOrderType,
    pub time_in_force: Option<TimeInForce>,
    pub quantity: Option<f64>,
    pub quote_order_qty: Option<f64>,
    pub price: Option<f64>,
    pub new_client_order_id: Option<String>,
    pub stop_price: Option<f64>,
    pub iceberg_qty: Option<f64>,
    // TODO make it enum
    pub new_order_resp_type: Option<String>,
}

impl<'a> PSpotOrderSpec<'a> {
    pub fn new(symbol: &'a Symbol, side: OrderSide, quantity: f64, price: Option<f64>) -> Self {
        match price {
            Some(_) => PSpotOrderSpec {
                symbol,
                side,
                order_type: SpotOrderType::Limit,
                time_in_force: Some(TimeInForce::GTC),
                quantity: Some(quantity),
                quote_order_qty: None,
                price,
                new_client_order_id: None,
                stop_price: None,
                iceberg_qty: None,
                new_order_resp_type: None,
            },
            None => PSpotOrderSpec {
                symbol,
                side,
                order_type: SpotOrderType::Market,
                time_in_force: None,
                quantity: Some(quantity),
                quote_order_qty: None,
                price,
                new_client_order_id: None,
                stop_price: None,
                iceberg_qty: None,
                new_order_resp_type: None,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderSide {
    Buy,
    Sell,
}

/// 现货订单种类
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SpotOrderType {
    ///限价单
    Limit,
    ///市价单
    Market,
    ///止损单
    StopLoss,
    ///限价止损单
    StopLossLimit,
    ///止盈单
    TakeProfit,
    ///限价止盈单
    TakeProfitLimit,
    ///限价只挂单
    LimitMaker,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
    GTX,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PTimestamp {
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

impl PTimestamp {
    pub fn now() -> Self {
        let now = chrono::Utc::now();
        PTimestamp {
            timestamp: now.timestamp_millis(),
            recv_window: Some(5000),
        }
    }
}

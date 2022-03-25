use std::fmt;

use serde::de::{SeqAccess, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

use crate::serde::string_as_f64;
use crate::trade::binance_api_params::{OrderSide, SpotOrderType, TimeInForce};
use crate::Symbol;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RSpotPrice {
    pub symbol: Symbol,
    #[serde(deserialize_with = "string_as_f64")]
    pub price: f64,
    pub time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RH24ticker {
    pub symbol: Symbol,
    #[serde(deserialize_with = "string_as_f64")]
    pub price_change: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub price_change_percent: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub weighted_avg_price: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub last_price: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub last_qty: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub open_price: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub high_price: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub low_price: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub volume: f64,
    #[serde(deserialize_with = "string_as_f64")]
    pub quote_volume: f64,
    pub open_time: i64,
    pub close_time: i64,
    pub first_id: i64,
    pub last_id: i64,
    pub count: usize,
}

#[derive(Debug)]
pub struct RKline {
    pub open_time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub close_time: i64,
    pub count: usize,
}

impl<'de> Deserialize<'de> for RKline {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_tuple(12, RKlineVisitor)
    }
}

struct RKlineVisitor;

impl<'de> Visitor<'de> for RKlineVisitor {
    type Value = RKline;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a tuple of (i64, String, String, String, String, String, i64, String, usize, String, String, String)")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let open_time: i64 = seq.next_element()?.ok_or_else(|| {
            serde::de::Error::invalid_value(Unexpected::Option, &"expect open time")
        })?;
        let open_str: &'de str = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_value(Unexpected::Option, &"open price"))?;
        let open = open_str.parse::<f64>().map_err(|_| {
            serde::de::Error::invalid_value(Unexpected::Str(open_str), &"f64 string")
        })?;
        let high_str: &'de str = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_value(Unexpected::Option, &"highest price"))?;
        let high = high_str.parse::<f64>().map_err(|_| {
            serde::de::Error::invalid_value(Unexpected::Str(high_str), &"f64 string")
        })?;
        let low_str: &'de str = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_value(Unexpected::Option, &"lowest price"))?;
        let low = low_str.parse::<f64>().map_err(|_| {
            serde::de::Error::invalid_value(Unexpected::Str(low_str), &"f64 string")
        })?;
        let close_str: &'de str = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_value(Unexpected::Option, &"close price"))?;
        let close = close_str.parse::<f64>().map_err(|_| {
            serde::de::Error::invalid_value(Unexpected::Str(close_str), &"f64 string")
        })?;
        seq.next_element::<&'de str>()?.ok_or_else(|| {
            serde::de::Error::invalid_value(Unexpected::Option, &"expect ignored padded field")
        })?;
        let close_time: i64 = seq.next_element()?.ok_or_else(|| {
            serde::de::Error::invalid_value(Unexpected::Option, &"expect close time")
        })?;
        seq.next_element::<&'de str>()?.ok_or_else(|| {
            serde::de::Error::invalid_value(Unexpected::Option, &"expect ignored padded field")
        })?;
        let count: usize = seq.next_element()?.ok_or_else(|| {
            serde::de::Error::invalid_value(Unexpected::Option, &"expect close time")
        })?;
        for _ in 0..3 {
            seq.next_element::<&'de str>()?.ok_or_else(|| {
                serde::de::Error::invalid_value(Unexpected::Option, &"expect ignored padded field")
            })?;
        }
        Ok(RKline {
            open_time,
            open,
            high,
            low,
            close,
            close_time,
            count,
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum SpotOrder {
    Ack(SpotOrderAck),
    Result(SpotOrderResult),
    Full(SpotOrderFull),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotOrderAck {
    /// 交易对
    pub symbol: String,
    /// 系统订单ID
    pub order_id: usize,
    /// OCO订单ID,否则为-1
    pub order_list_id: i64,
    /// 客户自己设置的ID
    pub client_order_id: String,
    /// 交易时间戳
    pub transact_time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotOrderResult {
    /// 交易对
    pub symbol: String,
    /// 系统订单ID
    pub order_id: usize,
    /// OCO订单ID,否则为-1
    pub order_list_id: i64,
    /// 客户自己设置的ID
    pub client_order_id: String,
    /// 交易时间戳
    pub transact_time: i64,
    /// 订单价格
    #[serde(deserialize_with = "string_as_f64")]
    pub price: f64,
    /// 用户设置的原始订单数量
    #[serde(deserialize_with = "string_as_f64")]
    pub orig_qty: f64,
    /// 交易的订单数量
    #[serde(deserialize_with = "string_as_f64")]
    pub executed_qty: f64,
    /// 累计交易的金额
    #[serde(deserialize_with = "string_as_f64")]
    pub cummulative_quote_qty: f64,
    /// 订单状态
    pub status: OrderStatus,
    /// 订单的时效方式
    pub time_in_force: TimeInForce,
    /// 订单类型， 比如市价单，现价单等
    #[serde(rename = "type")]
    pub order_type: SpotOrderType,
    /// 订单方向
    pub side: OrderSide,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotOrderFull {
    /// 交易对
    pub symbol: String,
    /// 系统订单ID
    pub order_id: usize,
    /// OCO订单ID,否则为-1
    pub order_list_id: i64,
    /// 客户自己设置的ID
    pub client_order_id: String,
    /// 交易时间戳
    pub transact_time: i64,
    /// 订单价格
    #[serde(deserialize_with = "string_as_f64")]
    pub price: f64,
    /// 用户设置的原始订单数量
    #[serde(deserialize_with = "string_as_f64")]
    pub orig_qty: f64,
    /// 交易的订单数量
    #[serde(deserialize_with = "string_as_f64")]
    pub executed_qty: f64,
    /// 累计交易的金额
    #[serde(deserialize_with = "string_as_f64")]
    pub cummulative_quote_qty: f64,
    /// 订单状态
    pub status: OrderStatus,
    /// 订单的时效方式
    pub time_in_force: TimeInForce,
    /// 订单类型， 比如市价单，现价单等
    #[serde(rename = "type")]
    pub order_type: SpotOrderType,
    /// 订单方向
    pub side: OrderSide,
    /// 订单中交易的信息
    pub fills: Vec<OrderFill>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderFill {
    /// 交易的价格
    #[serde(deserialize_with = "string_as_f64")]
    pub price: f64,
    /// 交易的数量
    #[serde(deserialize_with = "string_as_f64")]
    pub qty: f64,
    /// 手续费金额
    #[serde(deserialize_with = "string_as_f64")]
    pub commission: f64,
    /// 手续费的币种
    pub commission_asset: String,
}

/// 订单状态
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    /// 新建订单
    New,
    /// 部分成交
    PartiallyFilled,
    /// 全部成交
    Filled,
    /// 已撤销
    Canceled,
    /// 订单被拒绝
    Rejected,
    /// 订单过期(根据timeInForce参数规则)
    Expired,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuerySpotOrder {
    /// 交易对
    pub symbol: String,
    /// 系统订单ID
    pub order_id: usize,
    /// OCO订单ID,否则为-1
    pub order_list_id: i64,
    /// 客户自己设置的ID
    pub client_order_id: String,
    /// 订单价格
    #[serde(deserialize_with = "string_as_f64")]
    pub price: f64,
    /// 用户设置的原始订单数量
    #[serde(deserialize_with = "string_as_f64")]
    pub orig_qty: f64,
    /// 交易的订单数量
    #[serde(deserialize_with = "string_as_f64")]
    pub executed_qty: f64,
    /// 累计交易的金额
    #[serde(deserialize_with = "string_as_f64")]
    pub cummulative_quote_qty: f64,
    /// 订单状态
    pub status: OrderStatus,
    /// 订单的时效方式
    pub time_in_force: TimeInForce,
    /// 订单类型， 比如市价单，现价单等
    #[serde(rename = "type")]
    pub order_type: SpotOrderType,
    /// 订单方向
    pub side: OrderSide,
    /// 止损价格
    #[serde(deserialize_with = "string_as_f64")]
    pub stop_price: f64,
    /// 冰山数量
    #[serde(deserialize_with = "string_as_f64")]
    pub iceberg_qty: f64,
    /// 订单时间
    pub time: i64,
    /// 最后更新时间
    pub update_time: i64,
    /// 订单是否出现的 order book 中
    pub is_working: bool,
    /// 原始交易金额
    #[serde(deserialize_with = "string_as_f64")]
    pub orig_quote_order_qty: f64,
}

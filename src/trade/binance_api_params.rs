use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PKline {
    pub symbol: String,
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

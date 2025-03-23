use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")] // ???
pub enum Granularity {
    All,
    None,
    Second,
    Minute,
    FifteenMinute,
    ThirtyMinute,
    Hour,
    Day,
    Week,
    Month,
    Quarter,
    Year,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GranularitySpec {
    #[serde(rename = "type")]
    type_: String,
    segment_granularity: Granularity,
    query_granularity: Granularity,
    rollup: Option<bool>,
    intervals: Option<Vec<String>>,
}

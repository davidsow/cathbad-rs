use crate::query::components::model::QueryComponent;
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
#[serde(rename_all = "camelCase")]
pub enum GranularitySpecType {
    GranularitySpec,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GranularitySpec {
    #[serde(rename = "type")]
    type_: GranularitySpecType,
    segment_granularity: Granularity,
    query_granularity: Granularity,
    rollup: Option<bool>,
    intervals: Option<Vec<String>>,
}

impl QueryComponent for GranularitySpec {
    fn validate_type(&self) -> bool {
        match self.type_ {
            GranularitySpecType::GranularitySpec => true,
        }
    }
}

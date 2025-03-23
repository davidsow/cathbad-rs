use crate::query::components::model::QueryComponent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TopNMetricSpecType {
    Numeric,
    Dimension,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TopNMetricSpec {
    Numeric {
        #[serde(rename = "type")]
        type_: TopNMetricSpecType,
        metric: String,
    },
    Dimension {
        #[serde(rename = "type")]
        type_: TopNMetricSpecType,
        ordering: Option<String>,
        previous_stop: Option<String>,
    },
}

impl QueryComponent for TopNMetricSpec {
    fn validate_type(&self) -> bool {
        match self {
            TopNMetricSpec::Numeric { type_, .. } => *type_ == TopNMetricSpecType::Numeric,
            TopNMetricSpec::Dimension { type_, .. } => *type_ == TopNMetricSpecType::Dimension,
        }
    }
}

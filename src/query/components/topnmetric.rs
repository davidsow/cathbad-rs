use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TopNMetricSpecType {
    Numeric,
    Dimension,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TopNMetricSpec {
    Numeric {
        #[serde(rename = "type")]
        _type: TopNMetricSpecType,
        metric: String,
    },
    Dimension {
        #[serde(rename = "type")]
        type_: TopNMetricSpecType,
        ordering: Option<String>,
        previous_stop: Option<String>,
    },
}

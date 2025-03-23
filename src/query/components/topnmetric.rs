use crate::query::components::model::QueryComponent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag="type", rename_all="camelCase")]
pub enum TopNMetricSpec {
    Numeric {
        metric: String,
    },
    Dimension {
        ordering: Option<String>,
        previous_stop: Option<String>,
    },
}

impl QueryComponent for TopNMetricSpec {
    fn validate_type(&self) -> bool {
        match self {
            TopNMetricSpec::Numeric { .. } => true,
            TopNMetricSpec::Dimension { .. } => true,
        }
    }
}

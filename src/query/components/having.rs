use crate::query::components::model::QueryComponent;
use crate::query::{DimensionSpec, Filter, FloatingPointNumber};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag="type", rename_all="camelCase")]
pub enum Having {
    Filter {
        filter: Filter,
    },
    EqualTo {
        aggregation: String,
        value: FloatingPointNumber,
    },
    GreaterThan {
        aggregation: String,
        value: FloatingPointNumber,
    },
    LessThan {
        aggregation: String,
        value: FloatingPointNumber,
    },
    DimSelector {
        dimension: String,
        value: DimensionSpec,
    },
    #[serde(rename_all = "camelCase")]
    And {
        having_specs: Vec<Box<Having>>,
    },
    #[serde(rename_all = "camelCase")]
    Or {
        having_specs: Vec<Box<Having>>,
    },
    #[serde(rename_all = "camelCase")]
    Not {
        having_specs: Vec<Box<Having>>,
    },
}

impl QueryComponent for Having {
    fn validate_type(&self) -> bool {
        match self {
            // TODO
            Having::Filter { .. } => true,
            Having::EqualTo { .. } => true,
            Having::GreaterThan { .. } => true,
            Having::LessThan { .. } => true,
            Having::DimSelector { .. } => true,
            Having::And { .. } => true,
            Having::Or { .. } => true,
            Having::Not { .. } => true,
        }
    }
}

impl QueryComponent for Option<Having> {
    fn validate_type(&self) -> bool {
        self.clone().is_none_or(|having| having.validate_type())
    }
}

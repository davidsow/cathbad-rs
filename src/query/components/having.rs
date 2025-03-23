use crate::query::{DimensionSpec, Filter, FloatingPointNumber};
use serde::{Deserialize, Serialize};
use crate::query::components::model::QueryComponent;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum HavingType {
    Filter,
    EqualTo,
    GreaterThan,
    LessThan,
    DimSelector,
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Having {
    Filter {
        #[serde(rename = "type")]
        type_: HavingType,
        filter: Filter,
    },
    EqualTo {
        #[serde(rename = "type")]
        type_: HavingType,
        aggregation: String,
        value: FloatingPointNumber,
    },
    GreaterThan {
        #[serde(rename = "type")]
        type_: HavingType,
        aggregation: String,
        value: FloatingPointNumber,
    },
    LessThan {
        #[serde(rename = "type")]
        type_: HavingType,
        aggregation: String,
        value: FloatingPointNumber,
    },
    DimSelector {
        #[serde(rename = "type")]
        type_: HavingType,
        dimension: String,
        value: DimensionSpec,
    },
    #[serde(rename_all = "camelCase")]
    And {
        #[serde(rename = "type")]
        type_: HavingType,
        having_specs: Vec<Box<Having>>,
    },
    #[serde(rename_all = "camelCase")]
    Or {
        #[serde(rename = "type")]
        type_: HavingType,
        having_specs: Vec<Box<Having>>,
    },
    #[serde(rename_all = "camelCase")]
    Not {
        #[serde(rename = "type")]
        type_: HavingType,
        having_specs: Vec<Box<Having>>,
    },
}

impl QueryComponent for Having {
    fn validate_type(&self) -> bool {
        match self {
            Having::Filter { type_,  .. } => {
                *type_ == HavingType::Filter
            }
            Having::EqualTo { type_,  .. } => {
                *type_ == HavingType::EqualTo
            }
            Having::GreaterThan { type_,  .. } => {
                *type_ == HavingType::GreaterThan
            }
            Having::LessThan { type_,  .. } => {
                *type_ == HavingType::LessThan
            }
            Having::DimSelector { type_,  .. } => {
                *type_ == HavingType::DimSelector
            }
            Having::And { type_,  .. } => {
                *type_ == HavingType::And
            }
            Having::Or { type_,  .. } => {
                *type_ == HavingType::Or
            }
            Having::Not { type_,  .. } => {
                *type_ == HavingType::Not
            }
        }
    }
}

impl QueryComponent for Option<Having> {
    fn validate_type(&self) -> bool {
        self.clone().is_none_or(|having| {
            having.validate_type()
        })
    }
}
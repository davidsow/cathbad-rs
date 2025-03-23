use crate::query::{DimensionSpec, Filter, FloatingPointNumber};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
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

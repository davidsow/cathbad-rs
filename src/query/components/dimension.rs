use crate::query::{ExtractionFunction, OutputType};
use serde::{Deserialize, Serialize};
use crate::query::components::model::QueryComponent;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum DimensionSpecType {
    Default,
    Extraction,
    ListFiltered,
    RegexFiltered,
    PrefixFiltered,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DimensionSpec {
    Default {
        #[serde(rename = "type")]
        type_: DimensionSpecType,
        dimension: String,
        output_name: Option<String>,
        output_type: Option<OutputType>,
    },

    #[serde(rename_all = "camelCase")]
    Extraction {
        #[serde(rename = "type")]
        type_: DimensionSpecType,
        dimension: String,
        output_name: Option<String>,
        output_type: Option<OutputType>,
        extraction_fn: ExtractionFunction,
    },

    #[serde(rename_all = "camelCase")]
    ListFiltered {
        #[serde(rename = "type")]
        type_: DimensionSpecType,
        delegate: Box<DimensionSpec>,
        values: Vec<String>,
        is_whitelist: Option<bool>,
    },

    RegexFiltered {
        #[serde(rename = "type")]
        type_: DimensionSpecType,
        delegate: Box<DimensionSpec>,
        pattern: String,
    },

    PrefixFiltered {
        #[serde(rename = "type")]
        type_: DimensionSpecType,
        delegate: Box<DimensionSpec>,
        prefix: String,
    },
}

impl QueryComponent for DimensionSpec {
    fn validate_type(&self) -> bool {
        match self {
            DimensionSpec::Default { type_,  .. } => {
                *type_ == DimensionSpecType::Default
            }
            DimensionSpec::Extraction { type_,  .. } => {
                *type_ == DimensionSpecType::Extraction
            }
            DimensionSpec::ListFiltered { type_,  .. } => {
                *type_ == DimensionSpecType::ListFiltered
            }
            DimensionSpec::RegexFiltered { type_,  .. } => {
                *type_ == DimensionSpecType::RegexFiltered
            }
            DimensionSpec::PrefixFiltered { type_,  .. } => {
                *type_ == DimensionSpecType::PrefixFiltered
            }
        }
    }
}
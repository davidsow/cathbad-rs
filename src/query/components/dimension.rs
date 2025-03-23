use crate::query::{ExtractionFunction, OutputType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
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

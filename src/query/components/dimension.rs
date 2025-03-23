use crate::query::components::model::QueryComponent;
use crate::query::{ExtractionFunction, OutputType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag="type", rename_all="camelCase")]
pub enum DimensionSpec {
    Default {
        dimension: String,
        output_name: Option<String>,
        output_type: Option<OutputType>,
    },

    #[serde(rename_all = "camelCase")]
    Extraction {
        dimension: String,
        output_name: Option<String>,
        output_type: Option<OutputType>,
        extraction_fn: ExtractionFunction,
    },

    #[serde(rename_all = "camelCase")]
    ListFiltered {
        delegate: Box<DimensionSpec>,
        values: Vec<String>,
        is_whitelist: Option<bool>,
    },

    RegexFiltered {
        delegate: Box<DimensionSpec>,
        pattern: String,
    },

    PrefixFiltered {
        delegate: Box<DimensionSpec>,
        prefix: String,
    },
}

impl QueryComponent for DimensionSpec {
    fn validate_type(&self) -> bool {
        match self {
            //TODO
            DimensionSpec::Default { .. } => true,
            DimensionSpec::Extraction { .. } => true,
            DimensionSpec::ListFiltered { .. } => true,
            DimensionSpec::RegexFiltered { .. } => true,
            DimensionSpec::PrefixFiltered { .. } => true,
        }
    }
}

impl QueryComponent for Vec<DimensionSpec> {
    fn validate_type(&self) -> bool {
        for dim in self {
            if !dim.validate_type() {
                return false;
            }
        }
        true
    }
}

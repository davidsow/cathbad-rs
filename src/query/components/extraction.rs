use crate::query::{Granularity, IntegerNumber, SearchQuery};
use serde::{Deserialize, Serialize};
use crate::query::components::model::QueryComponent;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub enum ExtractionFunctionType {
    Regex,
    Partial,
    SearchQuery,
    Substring,
    Strlen,
    TimeFormat,
    TimeParsing,
    JavaScript,
    Cascade,
    StringFormat,
    Upper,
    Lower,
    Bucket,
    // RegisteredLookup,
    // InlineLookup,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ExtractionFunction {
    #[serde(rename_all = "camelCase")]
    Regex {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
        expr: String,
        index: IntegerNumber,
        replace_missing_value: bool,
        replace_missing_value_with: Option<String>,
    },
    Partial {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
        expr: String,
    },
    SearchQuery {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
        query: SearchQuery,
    },
    Substring {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
        index: IntegerNumber,
        length: Option<IntegerNumber>,
    },
    Strlen {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
    },
    #[serde(rename_all = "camelCase")]
    TimeFormat {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
        format: Option<String>,
        time_zone: Option<String>,
        locale: Option<String>,
        granularity: Option<Granularity>,
        as_millis: Option<bool>,
    },
    #[serde(rename_all = "camelCase")]
    TimeParsing {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
        time_format: String,
        result_format: String,
        joda: Option<bool>,
    },
    JavaScript {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
        function: String,
        injective: Option<bool>,
    },
    #[serde(rename_all = "camelCase")]
    Cascade {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
        extraction_fns: Vec<ExtractionFunction>,
    },
    #[serde(rename_all = "camelCase")]
    StringFormat {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
        format: String,
        null_handling: Option<String>,
    },
    Upper {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
        locale: Option<String>,
    },
    Lower {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
        locale: Option<String>,
    },
    Bucket {
        #[serde(rename = "type")]
        type_: ExtractionFunctionType,
        size: IntegerNumber,
        offset: IntegerNumber,
    },
    //
    // I am consciously deciding not to support Lookup operations at this time
    //
    // #[serde(rename_all = "camelCase")]
    // RegisteredLookup {
    //     #[serde(rename = "type")]
    //     type_: ExtractionFunctionType,
    //     lookup: String,
    //     retain_missing_value: Option<bool>,
    //     replace_missing_value_with: Option<String>,
    // },
    // #[serde(rename_all = "camelCase")]
    // InlineLookup {
    //     #[serde(rename = "type")]
    //     type_: ExtractionFunctionType,
    //     lookup: Lookup,
    //     retain_missing_value: Option<bool>,
    //     replace_missing_value_with: Option<String>,
    //     injective: Option<bool>,
    // },
}

impl QueryComponent for ExtractionFunction {
    fn validate_type(&self) -> bool {
        match self {
            ExtractionFunction::Regex { type_,  .. } => {
                *type_ == ExtractionFunctionType::Regex
            }
            ExtractionFunction::Partial { type_,  .. } => {
                *type_ == ExtractionFunctionType::Partial
            }
            ExtractionFunction::SearchQuery { type_,  .. } => {
                *type_ == ExtractionFunctionType::SearchQuery
            }
            ExtractionFunction::Substring { type_,  .. } => {
                *type_ == ExtractionFunctionType::Substring
            }
            ExtractionFunction::Strlen { type_,  .. } => {
                *type_ == ExtractionFunctionType::Strlen
            }
            ExtractionFunction::TimeFormat { type_,  .. } => {
                *type_ == ExtractionFunctionType::TimeFormat
            }
            ExtractionFunction::TimeParsing { type_,  .. } => {
                *type_ == ExtractionFunctionType::TimeParsing
            }
            ExtractionFunction::JavaScript { type_,  .. } => {
                *type_ == ExtractionFunctionType::JavaScript
            }
            ExtractionFunction::Cascade { type_,  .. } => {
                *type_ == ExtractionFunctionType::Cascade
            }
            ExtractionFunction::StringFormat { type_,  .. } => {
                *type_ == ExtractionFunctionType::StringFormat
            }
            ExtractionFunction::Upper { type_,  .. } => {
                *type_ == ExtractionFunctionType::Upper
            }
            ExtractionFunction::Lower { type_,  .. } => {
                *type_ == ExtractionFunctionType::Lower
            }
            ExtractionFunction::Bucket { type_,  .. } => {
                *type_ == ExtractionFunctionType::Bucket
            }
        }
    }
}

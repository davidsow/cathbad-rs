use crate::query::components::model::QueryComponent;
use crate::query::{Granularity, IntegerNumber, SearchQuery};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag="type", rename_all="camelCase")]
pub enum ExtractionFunction {
    #[serde(rename_all = "camelCase")]
    Regex {
        expr: String,
        index: IntegerNumber,
        replace_missing_value: bool,
        replace_missing_value_with: Option<String>,
    },
    Partial {
        expr: String,
    },
    SearchQuery {
        query: SearchQuery,
    },
    Substring {
        index: IntegerNumber,
        length: Option<IntegerNumber>,
    },
    Strlen {},
    #[serde(rename_all = "camelCase")]
    TimeFormat {
        format: Option<String>,
        time_zone: Option<String>,
        locale: Option<String>,
        granularity: Option<Granularity>,
        as_millis: Option<bool>,
    },
    #[serde(rename_all = "camelCase")]
    TimeParsing {
        time_format: String,
        result_format: String,
        joda: Option<bool>,
    },
    JavaScript {
        function: String,
        injective: Option<bool>,
    },
    #[serde(rename_all = "camelCase")]
    Cascade {
        extraction_fns: Vec<ExtractionFunction>,
    },
    #[serde(rename_all = "camelCase")]
    StringFormat {
        format: String,
        null_handling: Option<String>,
    },
    Upper {
        locale: Option<String>,
    },
    Lower {
        locale: Option<String>,
    },
    Bucket {
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
            //TODO
            ExtractionFunction::Regex { .. } => true,
            ExtractionFunction::Partial { .. } => true,
            ExtractionFunction::SearchQuery { .. } => true,
            ExtractionFunction::Substring { .. } => true,
            ExtractionFunction::Strlen { .. } => true,
            ExtractionFunction::TimeFormat { .. } => true,
            ExtractionFunction::TimeParsing { .. } => true,
            ExtractionFunction::JavaScript { .. } => true,
            ExtractionFunction::Cascade { .. } => true,
            ExtractionFunction::StringFormat { .. } => true,
            ExtractionFunction::Upper { .. } => true,
            ExtractionFunction::Lower { .. } => true,
            ExtractionFunction::Bucket { .. } => true,
        }
    }
}

use crate::query::{Granularity, IntegerNumber, SearchQuery};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
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

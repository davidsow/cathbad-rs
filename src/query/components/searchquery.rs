use crate::query::components::model::QueryComponent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SearchQueryType {
    InsensitiveContains,
    Fragment,
    Contains,
    Regex,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SearchQuery {
    InsensitiveContains {
        #[serde(rename = "type")]
        type_: SearchQueryType,
        value: String,
    },
    Fragment {
        #[serde(rename = "type")]
        type_: SearchQueryType,
        case_sensitive: bool,
        values: Vec<String>,
    },
    Contains {
        #[serde(rename = "type")]
        type_: SearchQueryType,
        case_sensitive: bool,
        value: String,
    },
    Regex {
        #[serde(rename = "type")]
        type_: SearchQueryType,
        pattern: String,
    },
}

impl QueryComponent for SearchQuery {
    fn validate_type(&self) -> bool {
        match self {
            SearchQuery::InsensitiveContains { type_, .. } => {
                *type_ == SearchQueryType::InsensitiveContains
            }
            SearchQuery::Fragment { type_, .. } => *type_ == SearchQueryType::Fragment,
            SearchQuery::Contains { type_, .. } => *type_ == SearchQueryType::Contains,
            SearchQuery::Regex { type_, .. } => *type_ == SearchQueryType::Regex,
        }
    }
}

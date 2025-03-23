use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
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

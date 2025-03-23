use crate::query::components::model::QueryComponent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SearchQuery {
    InsensitiveContains {
        value: String,
    },
    Fragment {
        case_sensitive: bool,
        values: Vec<String>,
    },
    Contains {
        case_sensitive: bool,
        value: String,
    },
    Regex {
        pattern: String,
    },
}

impl QueryComponent for SearchQuery {
    fn validate_type(&self) -> bool {
        match self {
            //TODO
            SearchQuery::InsensitiveContains { .. } => true,
            SearchQuery::Fragment { .. } => true,
            SearchQuery::Contains { .. } => true,
            SearchQuery::Regex { .. } => true,
        }
    }
}

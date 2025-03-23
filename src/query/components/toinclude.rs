use serde::{Deserialize, Serialize};
use crate::query::components::model::QueryComponent;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ToIncludeType {
    All,
    None,
    List,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ToInclude {
    All {
        #[serde(rename = "type")]
        type_: ToIncludeType,
    },
    None {
        #[serde(rename = "type")]
        type_: ToIncludeType,
    },
    List {
        #[serde(rename = "type")]
        type_: ToIncludeType,
        columns: Vec<String>,
    },
}

impl QueryComponent for ToInclude {
    fn validate_type(&self) -> bool {
        match self {
            ToInclude::All { type_, .. } => {
                *type_ == ToIncludeType::All
            }
            ToInclude::None { type_, .. } => {
                *type_ == ToIncludeType::None
            }
            ToInclude::List { type_, .. } => {
                *type_ == ToIncludeType::List
            }
        }
    }
}
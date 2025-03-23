use crate::query::components::model::QueryComponent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ToIncludeType {
    All,
    None,
    List,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
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
            ToInclude::All { type_, .. } => *type_ == ToIncludeType::All,
            ToInclude::None { type_, .. } => *type_ == ToIncludeType::None,
            ToInclude::List { type_, .. } => *type_ == ToIncludeType::List,
        }
    }
}

impl QueryComponent for Option<Vec<ToInclude>> {
    fn validate_type(&self) -> bool {
        self.clone().is_none_or(|include_vector| {
            for to_include in include_vector {
                if !to_include.validate_type() {
                    return false;
                }
            }
            true
        })
    }
}

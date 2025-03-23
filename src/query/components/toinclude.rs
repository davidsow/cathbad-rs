use crate::query::components::model::QueryComponent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ToInclude {
    All,
    None,
    List { columns: Vec<String> },
}

impl QueryComponent for ToInclude {
    fn validate_type(&self) -> bool {
        match self {
            //TODO
            ToInclude::All { .. } => true,
            ToInclude::None { .. } => true,
            ToInclude::List { .. } => true,
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

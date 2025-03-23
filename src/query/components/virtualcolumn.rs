use crate::query::components::model::QueryComponent;
use crate::query::{Expression, OutputType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename = "camelCase")]
pub struct VirtualColumn {
    name: String,
    expression: Expression,
    output_type: OutputType,
}

impl QueryComponent for VirtualColumn {
    fn validate_type(&self) -> bool {
        //TODO
        true
    }
}

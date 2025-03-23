use crate::query::components::model::QueryComponent;
use crate::query::{Expression, OutputType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum VirtualColumnType {
    VirtualColumn,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VirtualColumn {
    #[serde(rename = "type")]
    type_: VirtualColumnType,
    name: String,
    expression: Expression,
    output_type: OutputType,
}

impl QueryComponent for VirtualColumn {
    fn validate_type(&self) -> bool {
        match self.type_ {
            VirtualColumnType::VirtualColumn => true, // this is redundant now but is future-proofing against new variants
        }
    }
}

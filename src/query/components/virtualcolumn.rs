use crate::query::{Expression, OutputType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VirtualColumn {
    #[serde(rename = "type")]
    type_: String,
    name: String,
    expression: Expression,
    output_type: OutputType,
}

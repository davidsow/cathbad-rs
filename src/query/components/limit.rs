use crate::query::IntegerNumber;
use crate::query::components::helpers::OrderByColumnSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LimitSpecType {
    Default,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LimitSpec {
    #[serde(rename = "type")]
    type_: LimitSpecType,
    limit: Option<IntegerNumber>,
    offset: Option<IntegerNumber>,
    columns: Option<Vec<OrderByColumnSpec>>,
}

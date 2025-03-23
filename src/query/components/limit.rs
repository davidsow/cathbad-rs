use crate::query::IntegerNumber;
use crate::query::components::helpers::OrderByColumnSpec;
use serde::{Deserialize, Serialize};
use crate::query::components::model::QueryComponent;

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

impl QueryComponent for LimitSpec {
    fn validate_type(&self) -> bool {
        match self.type_ {
            LimitSpecType::Default => {true} // this is redundant now but is future-proofing against new variants
        }
    }
}
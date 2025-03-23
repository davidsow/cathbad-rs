use crate::query::IntegerNumber;
use crate::query::components::helpers::OrderByColumnSpec;
use crate::query::components::model::QueryComponent;
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

impl QueryComponent for LimitSpec {
    fn validate_type(&self) -> bool {
        match self.type_ {
            LimitSpecType::Default => true, // this is redundant now but is future-proofing against new variants
        }
    }
}

impl QueryComponent for Option<LimitSpec> {
    fn validate_type(&self) -> bool {
        self.clone().is_none_or(|limit| limit.validate_type())
    }
}

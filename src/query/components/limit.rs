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
#[serde(tag="type", rename_all="camelCase")]
pub struct LimitSpec {
    limit: Option<IntegerNumber>,
    offset: Option<IntegerNumber>,
    columns: Option<Vec<OrderByColumnSpec>>,
}

impl QueryComponent for LimitSpec {
    fn validate_type(&self) -> bool {
        //TODO
        true
    }
}

impl QueryComponent for Option<LimitSpec> {
    fn validate_type(&self) -> bool {
        self.clone().is_none_or(|limit| limit.validate_type())
    }
}

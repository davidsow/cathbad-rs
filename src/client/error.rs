use crate::query::DruidQueryResponse;

pub enum CathbadClientError {
    InvalidQuery,
    QueryMarshal { serde_error: serde_json::Error },
    Reqwest { reqwest_error: reqwest::Error },
    Druid { druid_error: DruidError },
    DruidErrorUnmarshal,
}

impl From<serde_json::Error> for CathbadClientError {
    fn from(value: serde_json::Error) -> Self {
        Self::QueryMarshal { serde_error: value }
    }
}

impl From<reqwest::Error> for CathbadClientError {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest {
            reqwest_error: value,
        }
    }
}

impl From<DruidError> for CathbadClientError {
    fn from(value: DruidError) -> Self {
        Self::Druid { druid_error: value }
    }
}

impl TryFrom<DruidQueryResponse> for DruidError {
    type Error = CathbadClientError;

    fn try_from(value: DruidQueryResponse) -> Result<Self, Self::Error> {
        use DruidError::*;
        match value {
            DruidQueryResponse::Success {} => Err(CathbadClientError::DruidErrorUnmarshal),
            DruidQueryResponse::Error { error, .. } => match error.as_str() {
                "SQL parse failed" => Ok(SQLParseFailed),
                "Plan validation failed" => Ok(PlanValidationFailed),
                "Resource limit exceeded" => Ok(ResourceLimitExceeded),
                "Query capacity exceeded" => Ok(QueryCapacityExceeded),
                "Unsupported operation" => Ok(UnsupportedOperation),
                "Query timeout" => Ok(QueryTimeout),
                "Query interrupted" => Ok(QueryInterrupted),
                "Query cancelled" => Ok(QueryCancelled),
                "Truncated response context" => Ok(TruncatedResponseContext),
                "Unknown exception" => Ok(UnknownException),
                _ => Err(CathbadClientError::DruidErrorUnmarshal),
            },
        }
    }
}

pub enum DruidError {
    SQLParseFailed,           // 400
    PlanValidationFailed,     // 400
    ResourceLimitExceeded,    // 400
    QueryCapacityExceeded,    // 429
    UnsupportedOperation,     // 501
    QueryTimeout,             // 504
    QueryInterrupted,         // 500
    QueryCancelled,           // 500
    TruncatedResponseContext, // 500
    UnknownException,         // 500
}

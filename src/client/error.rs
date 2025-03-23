pub enum CathbadClientError {
    InvalidQuery,
    QueryMarshalError { serde_error: serde_json::Error },
}

impl From<serde_json::Error> for CathbadClientError {
    fn from(value: serde_json::Error) -> Self {
        Self::QueryMarshalError { serde_error: value }
    }
}

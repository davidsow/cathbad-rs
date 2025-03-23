use crate::client::{CathbadClientConfig, CathbadClientError};
use crate::query::TypeConstrainedQuery;

pub trait DruidClient {
    fn query(query: impl TypeConstrainedQuery) -> Result<(), CathbadClientError>;
}

#[derive(Debug, Clone, Default)]
pub struct CathbadClient {
    config: CathbadClientConfig,
}

impl CathbadClient {
    pub fn new(config: CathbadClientConfig) -> Self {
        Self { config }
    }
}

impl DruidClient for CathbadClient {
    fn query(query: impl TypeConstrainedQuery) -> Result<(), CathbadClientError> {
        if !query.validate_type() || !query.validate_subcomponents() {
            return Err(CathbadClientError::InvalidQuery);
        }

        let _ = serde_json::to_string(&query)?;

        Ok(())
    }
}

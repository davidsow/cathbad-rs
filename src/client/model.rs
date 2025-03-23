use crate::client::{CathbadClientConfig, CathbadClientError};
use crate::query::TypeConstrainedQuery;

pub trait DruidClient {
    fn query(&self, query: impl TypeConstrainedQuery) -> Result<(), CathbadClientError>;
}

#[derive(Debug, Clone, Default)]
pub struct CathbadClient {
    config: CathbadClientConfig,
}

impl CathbadClient {
    pub fn new(config: CathbadClientConfig) -> Self {
        Self { config }
    }

    fn format_endpoint(&self) -> String {
        format!("{}:{}", self.config.druid_endpoint, self.config.druid_port)
    }
}

impl DruidClient for CathbadClient {
    fn query(&self, query: impl TypeConstrainedQuery) -> Result<(), CathbadClientError> {
        if !query.validate_type() || !query.validate_subcomponents() {
            return Err(CathbadClientError::InvalidQuery);
        }

        let _endpoint = self.format_endpoint();

        let _payload = serde_json::to_string(&query)?;

        Ok(())
    }
}

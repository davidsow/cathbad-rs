use crate::client::{CathbadClientConfig, CathbadClientError};
use crate::query::{DruidQueryResponse, TypeConstrainedQuery};
use reqwest::Client;
use reqwest::header::{ACCEPT, CONTENT_TYPE, HeaderMap};
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

trait DruidClient {
    async fn query(
        &self,
        query: impl TypeConstrainedQuery,
    ) -> Result<DruidQueryResponse, CathbadClientError>;
}

#[derive(Debug, Clone)]
pub struct CathbadClient {
    config: CathbadClientConfig,
    client: Client,
}

impl CathbadClient {
    pub fn new(config: CathbadClientConfig) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        Self {
            config,
            client: Client::builder().default_headers(headers).build().unwrap(),
        }
    }
    fn format_endpoint(&self) -> String {
        format!("{}:{}", self.config.druid_endpoint, self.config.druid_port)
    }
}

impl Default for CathbadClient {
    fn default() -> Self {
        let default_config = CathbadClientConfig::default();
        Self::new(default_config)
    }
}

impl DruidClient for CathbadClient {
    async fn query(
        &self,
        query: impl TypeConstrainedQuery,
    ) -> Result<DruidQueryResponse, CathbadClientError> {
        if !query.validate_type() || !query.validate_subcomponents() {
            return Err(CathbadClientError::InvalidQuery);
        }

        let endpoint = self.format_endpoint();
        let payload = serde_json::to_string(&query)?;
        let req = self.client.post(endpoint).body(payload).build()?;
        let resp = self.client.execute(req).await?;
        if !resp.status().is_success() {
            return match resp.status().as_u16() {
                400 | 429 | 500 | 501 | 504 => {
                    let druid_resp: DruidQueryResponse = serde_json::from_str(&resp.text().await?)?;
                    if druid_resp == (DruidQueryResponse::Success {}) {
                        return Err(CathbadClientError::DruidErrorUnmarshal);
                    }
                    Ok(druid_resp)
                }
                _ => Err(CathbadClientError::DruidErrorUnmarshal),
            };
        }
        Ok(DruidQueryResponse::Success {}) // you didn't want that data, did you? >;-P
    }
}

#[cfg(test)]
mod tests {
    use crate::client::CathbadClient;

    #[test]
    fn test_default_client_creation() {
        let client = CathbadClient::default();

        assert_eq!(client.config.druid_endpoint, "http://localhost");
        assert_eq!(client.config.druid_port, 8888);

        assert_eq!(client.format_endpoint(), "http://localhost:8888");
    }
}

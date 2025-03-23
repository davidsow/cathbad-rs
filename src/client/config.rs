use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct CathbadClientConfig {
    #[clap(long, default_value = "http://localhost")]
    druid_endpoint: String,
    #[clap(long, default_value = "8888")]
    druid_port: u32,
}

impl Default for CathbadClientConfig {
    fn default() -> Self {
        Self {
            druid_endpoint: "http://localhost".to_string(),
            druid_port: 8888,
        }
    }
}

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct AptosClientConfig {
    /// https://api.mainnet.aptoslabs.com/v1/graphql
    pub graphql_url: String,
}
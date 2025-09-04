use serde::{Deserialize, Serialize};
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Path to file containing list of addresses (one per line)
    pub address_file_path: String,
    /// Alchemy API key
    pub alchemy_api_key: String,
}

#[derive(Serialize,Debug)]
pub struct AddressInput {
    pub address: String,
    pub networks: Vec<String>,
}

#[derive(Serialize,Debug)]
pub struct AlchemyRequest {
    pub addresses: Vec<AddressInput>
}

#[derive(Deserialize, Debug)]
pub struct TokenMetadata {
    pub decimals: Option<u8>,
    pub name:Option<String>,
    pub symbol: Option<String>
}


#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub tokenBalance: String,
    pub tokenMetadata: TokenMetadata,
}

#[derive(Deserialize, Debug)]
pub struct AlchemyTokensResponse {
    pub tokens: Vec<TokenResponse>,
}

#[derive(Deserialize, Debug)]
pub struct AlchemyResponse {
    pub data: AlchemyTokensResponse,
}

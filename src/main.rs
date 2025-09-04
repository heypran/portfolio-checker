use clap::Parser;
use std::fs;
use reqwest::Client;
use serde::{Deserialize,Serialize};

#[derive(Parser)]
struct Args {
    address_file_path:String,
    alchemy_api_key:String
}

#[derive(serde::Serialize)]
struct AddressInput {
    address:String,
    network:Vec<String>
}

#[derive(serde::Serialize)]
struct AlchemyRequest {
    addresses:Vec<AddressInput>
}

#[derive(serde::Deserialize)]
struct TokenMetadata {
    currency:String,
    value:u128
}

#[derive(serde::Deserialize)]
struct TokenPrices {
    decimals: u8,
    name:String,
    symbol:String
}

#[derive(serde::Deserialize)]
struct TokenResponse {
    address:String,
    network:String,
    token_address:String,
    token_balance:u128,
    token_metadata:TokenMetadata,
    token_prices:TokenPrices,
}

#[derive(serde::Deserialize,Debug)]
struct AlchemyResponse {
    tokens:Vec<TokenResponse>
}

#[tokio::main]
fn main() {
    let user_inputs = Args::parse();
    let api_key = user_inputs.alchemy_api_key;
    let network_ids:Vec<String> = vec!["eth-mainnet", "opt-mainnet","base-mainnet"].iter().map(|id| id.to_string()).collect();
    let address_list_str:Result<String, _> = fs::read_to_string(&user_inputs.address_file_path);
    
    let address_list = match &address_list_str {
        Ok(val) => {
            let addresses = val.split(",");
            addresses
        }
        Err(err)  => {
            // println!("Error readling address file {err}")
            panic!("Error readling address file {err}")
        }
    };

    // Print addresses using clone    
    // for addr in address_list.clone() {
    //     println!("Address {addr}")
    // }

    // for addr in address_list {
    //     println!("Address {addr}")
    // }

    // prepare params 
    let mut request_params :AlchemyRequest= AlchemyRequest { addresses: vec![] };
      for addr in address_list {
        println!("Address {addr}");
        let networks = network_ids.clone();
        let alchemy_addr = AddressInput{address:addr.to_string(),network: networks};
        request_params.addresses.push(alchemy_addr);
    }
    
    
    let alchemy_response = get_token_bal(&request_params, api_key);
    println!("alchemy_response {:?}",alchemy_response);
    
    println!("Hello, world!");
}

async fn get_token_bal(&request_params: &AlchemyRequest,api_key:String)->AlchemyResponse{


    // create client
    let alchemy_client = Client::new();
    let token_api_url= format!("https://api.g.alchemy.com/data/v1/{api_key}/assets/tokens/balances/by-address");
    
    let api_response = alchemy_client.post(token_api_url).json(&request_params).send().await;

     if api_response.status().is_success() {
        let json_body: AlchemyResponse = response.json().await?;
        println!("Received JSON: {:#?}", json_body);
        json_body
    } else {
        panic!("POST request failed with status: {}", response.status());
    }


}
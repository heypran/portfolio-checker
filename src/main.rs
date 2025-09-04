use clap::Parser;
use std::{collections::{hash_map, HashMap}, fs};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use serde_json::{Value, json};
use prettytable::{Table, Row, Cell};

#[derive(Parser)]
struct Args {
    /// Path to file containing list of addresses (one per line)
    address_file_path: String,
    /// Alchemy API key
    alchemy_api_key: String,
}

#[derive(Serialize,Debug)]
struct AddressInput {
    address: String,
    networks: Vec<String>,
}

#[derive(Serialize,Debug)]
struct AlchemyRequest {
    addresses: Vec<AddressInput>,
    withMetadata:bool
}

#[derive(Deserialize, Debug)]
struct TokenMetadata {
    currency: Option<String>,
    value: Option<u128>,
    decimals: Option<u8>,
    name:Option<String>,
    symbol: Option<String>
}

#[derive(Deserialize, Debug)]
struct TokenPrices {
    decimals: Option<u8>,
    name: Option<String>,
    symbol: Option<String>,
}

#[derive(Deserialize, Debug)]
struct TokenResponse {
    address: String,
    network: String,
    tokenAddress: Option<String>,
    tokenBalance: String,
    tokenMetadata: TokenMetadata,
    tokenPrices: Vec<TokenPrices>,
}

#[derive(Deserialize, Debug)]
struct AlchemyTokensResponse {
    tokens: Vec<TokenResponse>,
}

#[derive(Deserialize, Debug)]
struct AlchemyResponse {
    data: AlchemyTokensResponse,
}

#[tokio::main]
async fn main() -> Result<()> {
    let user_inputs = Args::parse();
    let api_key = &user_inputs.alchemy_api_key;
    let network_ids: Vec<String> = vec!["eth-mainnet", "opt-mainnet", "base-mainnet"]
        .iter()
        .map(|id| id.to_string())
        .collect();

    let address_list_str = fs::read_to_string(&user_inputs.address_file_path)
        .with_context(|| format!("Failed to read addresses file: {}", user_inputs.address_file_path))?;

    let address_list: Vec<String> = address_list_str
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    if address_list.is_empty() {
        eprintln!("No addresses found in file.");
        return Ok(());
    }

    // Prepare params
    let mut request_params: AlchemyRequest = AlchemyRequest { addresses: vec![],withMetadata:true };
    for addr in &address_list {
        println!("Processing address: {}", addr);
        let networks = network_ids.clone();
        let alchemy_addr = AddressInput {
            address: addr.to_string(),
            networks,
        };
        request_params.addresses.push(alchemy_addr);
    }

    let alchemy_response = get_token_bal(&request_params, api_key).await?;
    // println!("Alchemy response: {:?}", alchemy_response);

    let mut token_balance_map :HashMap<String,u128> =  HashMap::new();
    // Display balances
    for token in alchemy_response.data.tokens {
        // let token_price = token.tokenPrices.get(0);
        // let token_name = match token_price {
        //     Some(tp) => {match &tp.name {
        //         Some(name)=>name,
        //         None => &"Unknown Token".to_string(),    
        //     } },
        //     None => &"Unknown Token".to_string(),
        // };

        //    let token_decimal = match token_price {
        //     Some(tp) => {match &tp.decimals {
        //            Some(dec) => dec,
        //     None => &0, 
        //     } },
        //     None => &0,
        // };

        //     let token_symbol = match token_price {
        //         Some(tp) => {match &tp.symbol {
        //         Some(sym)=> sym,
        //         None => &"No Symbol".to_string()
        //         } },
        //         None => &"No Symbol".to_string()
        //     };
            
            let token_name = match &token.tokenMetadata.name {
                Some(name)=>name,
                None =>"no name"
            };

              let token_decimal = match &token.tokenMetadata.decimals {
                Some(dec)=>dec,
                None =>&0
            };

            let token_symbol = match &token.tokenMetadata.symbol {
                Some(sym)=>sym,
                None =>"no symbol"
            };
    

        let balance_hex = &token.tokenBalance;
        let balance_wei = u128::from_str_radix(&balance_hex[2..], 16)?;  // Skip "0x"

        let key = format!(
            "{} ({})",
            token_name,
            token_symbol
        );

        

        if token_balance_map.contains_key(&key) {
            let existing_balance = match token_balance_map.get(&key) {
                Some(val)=>val,
                None => &0
            };
            token_balance_map.insert(key, balance_wei + existing_balance);
        }else{
            token_balance_map.insert(key, balance_wei);   
        }
        // println!(
        //     "Address: {}, Network: {}, Token: {} ({}), Balance: {} (decimals: {})",
        //     token.address,
        //     token.network,
        //     token_name,
        //     token_symbol,
        //     balance_wei, // token.tokenBalance,
        //     token_decimal
        // );
    }

    // for k_v in token_balance_map.iter(){
    //     println!("{} -------- {}",k_v.0,k_v.1)
    // }
    print_balances(token_balance_map);

    Ok(())
}

async fn get_token_bal(request_params: &AlchemyRequest, api_key: &str) -> Result<AlchemyResponse> {
    // Create client
    let alchemy_client = Client::new();
    let token_api_url = format!("https://api.g.alchemy.com/data/v1/{}/assets/tokens/by-address", api_key);

    // println!("request_params....{:?}",request_params);

    let api_response = alchemy_client
        .post(&token_api_url)
        .json(request_params)
        .send()
        .await
        .with_context(|| "Failed to send request to Alchemy API")?;

    if api_response.status().is_success() {
        // let json_body: AlchemyResponse = api_response
        //     .json()
        //     .await
        //     .with_context(|| "Failed to parse JSON response")?;

        let response_text = api_response.text().await?;
        // println!("Raw response: {}", response_text);
        
        // Then parse the JSON from the text
        let json_body: AlchemyResponse = serde_json::from_str(&response_text)
            .with_context(|| "Failed to parse JSON response")?;
        

        // println!("Received JSON: {:#?}", json_body);
        Ok(json_body)
    } else {
        anyhow::bail!("POST request failed with status: {}", api_response.status());
    }
}


fn print_balances(token_balance_map:HashMap<String,u128>){
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Token"),
        Cell::new("Balance"),
    ]));

    // Add data rows
    for (token, balance) in &token_balance_map {
        table.add_row(Row::new(vec![
            Cell::new(token),
            Cell::new(&format!("{:.1}", balance)),  // Format balance
        ]));
    }

    // Print to terminal
    table.printstd();
}
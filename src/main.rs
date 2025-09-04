

use std::{collections::{ HashMap}, fs};
use anyhow::{Result, Context};
use clap::Parser;
use portfolio_checker::processor::aggregate_balances;
use portfolio_checker::models::{AlchemyRequest, AddressInput,Args};
use portfolio_checker::api::get_token_bal;
use portfolio_checker::output::print_balances;

#[tokio::main]
async fn main() -> Result<()> {
    let user_inputs = Args::parse();
    let api_key = &user_inputs.alchemy_api_key;

    // Fetch balance for these networks
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
    let mut request_params: AlchemyRequest = AlchemyRequest { addresses: vec![] };
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

    let token_balance_map :HashMap<String,u128> =  match aggregate_balances(alchemy_response.data.tokens) {
        Ok(bal)=>bal,
        Err(err)=>panic!("Balance cannot be aggregated!")
    };
    
    // Display balances
    print_balances(token_balance_map);

    Ok(())
}
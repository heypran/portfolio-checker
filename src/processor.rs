use std::collections::HashMap;
use crate::models::TokenResponse;
use anyhow::Result;

fn parse_balance(balance_hex: &str) -> Result<u128> {
    let balance_wei = u128::from_str_radix(&balance_hex[2..], 16)?;
    Ok(balance_wei)
}

pub fn aggregate_balances(tokens: Vec<TokenResponse>) -> Result<HashMap<String, u128>> {
    
      let mut token_balance_map :HashMap<String,u128> =  HashMap::new();
    // Display balances
    for token in tokens {
            
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
        let balance_wei = parse_balance(&balance_hex)?;

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
    }
    Ok(token_balance_map)

}

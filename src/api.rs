use crate::models::{AlchemyRequest, AlchemyResponse};
use anyhow::{Result, Context};
use reqwest::Client;

pub async fn get_token_bal(request_params: &AlchemyRequest, api_key: &str) -> Result<AlchemyResponse> {
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

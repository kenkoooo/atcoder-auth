use atcoder_auth::{lambda_start, AuthToken};
use lambda_runtime::error::HandlerError;
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient};
use serde::Deserialize;
use std::env;
use std::error::Error;

#[derive(Deserialize)]
struct Request {
    user_id: String,
    token: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    lambda_start(handler);
    Ok(())
}

fn handler(request: Request) -> Result<&'static str, HandlerError> {
    log::info!("verify={}", request.user_id);
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let client = DynamoDbClient::new(Region::ApNortheast1);
    let result = client.get_item(AuthToken::get_item_input(&request.user_id));
    let result = rt
        .block_on(result)
        .map_err(|e| HandlerError::from(format!("{:?}", e).as_str()))?;
    let token = AuthToken::get_token(&result).ok_or_else(|| HandlerError::from("not found"))?;

    if request.token != token {
        Err(HandlerError::from("Token unmatched"))
    } else {
        Ok("Ok")
    }
}

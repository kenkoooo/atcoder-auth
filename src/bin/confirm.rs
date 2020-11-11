use atcoder_auth::{
    generate_random_string, get_affiliation, lambda_start, AuthToken, VerificationCode,
};
use lambda_runtime::error::HandlerError;
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Deserialize)]
struct Request {
    user_id: String,
    secret: String,
}

#[derive(Serialize)]
struct Response {
    token: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    lambda_start(handler);
    Ok(())
}

fn handler(request: Request) -> Result<Response, HandlerError> {
    log::info!("confirm={}", request.user_id);
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let client = DynamoDbClient::new(Region::ApNortheast1);
    let result = client.get_item(VerificationCode::get_item_input(&request.user_id));
    let result = rt
        .block_on(result)
        .map_err(|e| HandlerError::from(format!("{:?}", e).as_str()))?;
    let verification_hash = VerificationCode::get_verification_hash(&result)
        .ok_or_else(|| HandlerError::from("not found"))?;

    let verification_code = rt
        .block_on(get_affiliation(&request.user_id))
        .map_err(|e| HandlerError::from(format!("{:?}", e).as_str()))?
        .ok_or_else(|| HandlerError::from("Empty affiliation"))?;
    let verification_pass = verification_code + &request.secret;

    if !bcrypt::verify(verification_pass, &verification_hash).unwrap_or(false) {
        return Err(HandlerError::from("Verification code unmatched"));
    }

    let token = generate_random_string(30);

    let auth_token = AuthToken::new(&request.user_id, &token);
    rt.block_on(client.put_item(auth_token.to_put_item_input()))
        .map_err(|e| HandlerError::from(format!("{:?}", e).as_str()))?;

    let response = Response { token };
    Ok(response)
}

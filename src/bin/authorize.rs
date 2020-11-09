use atcoder_auth::{generate_random_string, lambda_start, VerificationCode};
use lambda_runtime::error::HandlerError;
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Request {
    user_id: String,
}

#[derive(Serialize)]
struct Response {
    verification_code: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    lambda_start(handler);
    Ok(())
}

fn handler(request: Request) -> Result<Response, HandlerError> {
    let verification_code: String = generate_random_string(30);
    let item = VerificationCode::new(&request.user_id, &verification_code);

    let client = DynamoDbClient::new(Region::ApNortheast1);
    let result = client.put_item(item.to_put_item_input());

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(result)
        .map_err(|e| HandlerError::from(format!("{:?}", e).as_str()))?;

    let response = Response { verification_code };
    Ok(response)
}

use lambda_runtime::error::HandlerError;
use lambda_runtime::Context;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Deserialize)]
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
    lambda_runtime::lambda!(handler);
    Ok(())
}

fn handler(request: Request, c: Context) -> Result<Response, HandlerError> {
    Ok(Response {
        verification_code: request.user_id,
    })
}

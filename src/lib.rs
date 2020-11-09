mod model;
pub use model::{AuthToken, VerificationCode};

mod scraping;
pub use scraping::get_affiliation;

use lambda_http::RequestExt;
use lambda_runtime::error::HandlerError;
use lambda_runtime::Context;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub fn generate_random_string() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(30).collect()
}

pub trait LambdaHandler<Request, Response> {
    fn execute(&mut self, request: Request) -> Result<Response, HandlerError>;
}

impl<F, Request, Response> LambdaHandler<Request, Response> for F
where
    F: FnMut(Request) -> Result<Response, HandlerError>,
{
    fn execute(&mut self, request: Request) -> Result<Response, HandlerError> {
        (*self)(request)
    }
}

pub fn lambda_start<Request, Response>(f: impl LambdaHandler<Request, Response>)
where
    for<'de> Request: Deserialize<'de>,
    Response: Serialize,
{
    let mut f = f;
    lambda_http::start(
        |request: lambda_http::Request, _: Context| {
            let result: Result<Option<Request>, _> = request.payload::<Request>();
            let request = result.map_err(|e| {
                log::error!("{:?}", e);
                HandlerError::from(format!("{:?}", e).as_str())
            })?;
            let request = request.ok_or_else(|| {
                log::error!("Empty Body");
                HandlerError::from("Empty Body")
            })?;

            match f.execute(request) {
                Ok(response) => {
                    let value: Value = serde_json::to_value(response)?;
                    Ok(value)
                }
                Err(e) => {
                    log::error!("{:?}", e);
                    Err(e)
                }
            }
        },
        None,
    );
}

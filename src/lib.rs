mod model;
pub use model::{AuthToken, VerificationCode};

mod scraping;
pub use scraping::get_affiliation;

use lambda_http::http::header::{ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE};
use lambda_http::RequestExt;
use lambda_runtime::error::HandlerError;
use lambda_runtime::Context;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

pub fn generate_random_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .collect()
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
                    let body = serde_json::to_string(&response)?;
                    let response = lambda_http::Response::builder()
                        .header(CONTENT_TYPE, "application/json")
                        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                        .body(body)
                        .map_err(|e| {
                            log::error!("{:?}", e);
                            HandlerError::from(format!("{:?}", e).as_str())
                        })?;
                    Ok(response)
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

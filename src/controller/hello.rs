use crate::error::Error;
use lambda_http::{Request, Response, Body, RequestExt, http::StatusCode};
use serde::Serialize;

use super::json_response;

#[derive(Serialize)]
struct HelloResponse {
    pub hello: String,
}

pub async fn controller(req: Request) -> Result<Response<Body>, Error> {
    let params = req.query_string_parameters();
    let target = params.get("name").ok_or(Error::ParamMissing("name".to_string()))?;

    json_response(StatusCode::OK, &HelloResponse {
        hello: target.to_string(),
    })
}

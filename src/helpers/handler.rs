use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::{dev, HttpResponse, Result};
use std::error::Error;
use validator::ValidationErrors;

#[macro_export]
macro_rules! validate_errors {
    ($var:expr) => {
        match handler::to_errors($var.validate()) {
            Some(res) => {
                return res;
            }
            None => (),
        }
    };
}

pub fn to_errors(result: Result<(), ValidationErrors>) -> Option<HttpResponse> {
    match result {
        Ok(_) => None,
        Err(val_errors) => {
            let mut errors: Vec<String> = vec![];
            for (key, value) in &val_errors.field_errors() {
                for inner in value {
                    errors.push(format!("Field {} failed with {} error", key, inner.code));
                }
            }
            let res = json::object! { "success" => false, "errors" => errors }.dump();
            Some(
                HttpResponse::InternalServerError()
                    .content_type("application/json")
                    .body(res),
            )
        }
    }
}

pub fn to_json<T>(result: Result<T, Box<dyn Error>>) -> HttpResponse
where
    T: Sized + serde::Serialize,
{
    match result {
        Ok(data) => {
            let des = &serde_json::to_string(&data).expect("FATAL: Failed to deserialize data");
            let json_data = json::parse(des).expect("FATAL: Failed to parse data");
            let res = json::object! { "success" => true, "data" => json_data }.dump();
            HttpResponse::Ok()
                .content_type("application/json")
                .body(res)
        }
        Err(err) => {
            let res =
                json::object! { "success" => false, "errors" => vec![err.to_string()] }.dump();
            HttpResponse::InternalServerError()
                .content_type("application/json")
                .body(res)
        }
    }
}

pub fn bad_request_handler<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    // Todo: Handle 400 errors - send body as json {"success": false, "errors": [...]}
    // let err = format!("Error {:?}", tmp);
    // res.response_mut().headers_mut().insert(
    //     http::header::CONTENT_TYPE,
    //     http::HeaderValue::from_str(&err).unwrap(),
    // );
    Ok(ErrorHandlerResponse::Response(res))
}

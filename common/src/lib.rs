use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
struct JsonResponse {
    message: String,
    data: Option<Value>,
}

struct ResponseOptions {
    data: Option<Value>,
    message: String,
    status_code: StatusCode,
}

struct ResponseHandler {
    options: ResponseOptions,
}

impl ResponseHandler {
    fn new(options: ResponseOptions) -> Self {
        ResponseHandler { options }
    }

    fn create_json_response(self) -> JsonResponse {
        JsonResponse {
            message: self.options.message,
            data: self.options.data,
        }
    }

    fn create_http_response(self) -> HttpResponse {
        let status_code = self.options.status_code;
        let json_resp = self.create_json_response();
        HttpResponse::build(status_code).json(json_resp)
    }
}

//Generate response
pub fn response(
    message: &str,
    status_code: StatusCode,
    data: Option<serde_json::Value>,
) -> HttpResponse {
    let options = ResponseOptions {
        data,
        message: message.to_string(),
        status_code,
    };

    let response_handler = ResponseHandler::new(options);
    response_handler.create_http_response()
}

#[derive(Serialize, Debug, Deserialize)]
pub struct InputPayload {
    pub public: String,
    #[cfg(feature = "confidential")]
    pub secrets: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct CheckInputResponse {
    pub valid: bool,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct GenerateProofResponse {
    pub proof: String,
}

#[derive(Serialize)]
pub struct InvalidInputPayload {
    pub ask_id: String,
    pub public: String,
    #[cfg(feature = "confidential")]
    pub secrets: String,
}
pub mod secret_inputs_helpers;

#[derive(Serialize, Debug, Deserialize)]
pub struct EncryptedInputPayload {
    pub acl: String,
    pub encrypted_secrets: String,
    pub me_decryption_url: String,
    pub market_id: String,
}

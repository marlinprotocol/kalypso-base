use actix_web::{get, post};
use actix_web::{http::StatusCode, web, Responder};

#[get("/test")]
async fn test() -> impl Responder {
    common::response("IVS is running", StatusCode::OK, None)
}

#[post("/checkInputs")]
async fn check_input_handler(_payload: web::Json<common::InputPayload>) -> impl Responder {
    common::response(
        "Check Inputs API is not implemented",
        StatusCode::NOT_IMPLEMENTED,
        None,
    )
}

#[post("/checkEncryptedInputs")]
async fn check_enc_handler(_payload: web::Json<common::EncryptedInputPayload>) -> impl Responder {
    // use common::secret_inputs_helpers to read encryptions and decryptions
    common::response(
        "Check Encrypted Inputs API is not implemented",
        StatusCode::NOT_IMPLEMENTED,
        None,
    )
}

pub fn routes(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(test)
        .service(check_input_handler)
        .service(check_enc_handler);

    conf.service(scope);
}

use actix_web::{get, post};
use actix_web::{http::StatusCode, web, Responder};

#[get("/test")]
async fn test() -> impl Responder {
    common::response("IVS is running", StatusCode::OK, None)
}

#[post("/checkInputs")]
async fn check_input_handler(_payload: web::Json<common::InputPayload>) -> impl Responder {
    let check_input_response = common::CheckInputResponse { valid: false };

    let check_input_json = serde_json::to_value(check_input_response).unwrap();

    common::response(
        "Check Inputs API is not implemented",
        StatusCode::NOT_IMPLEMENTED,
        Some(check_input_json),
    )
}

#[post("/generateProofForInvalidInputs")]
async fn generate_proof_for_invalid_inputs(
    _payload: web::Json<common::InputPayload>,
) -> impl Responder {
    let proof = common::GenerateProofResponse {
        proof: "todo".into(),
    };
    let proof_json = serde_json::to_value(proof).unwrap();

    common::response(
        "Generate Proofs for Invalid Inputs is not implement",
        StatusCode::NOT_IMPLEMENTED,
        Some(proof_json),
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
        .service(generate_proof_for_invalid_inputs)
        .service(check_enc_handler);

    conf.service(scope);
}

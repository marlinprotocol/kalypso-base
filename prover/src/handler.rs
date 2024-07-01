use actix_web::{get, post};
use actix_web::{http::StatusCode, web, Responder};

#[get("/test")]
pub async fn test() -> impl Responder {
    common::response("Generator is running", StatusCode::OK, None)
}

#[get("/benchmark")]
pub async fn benchmark() -> impl Responder {
    common::response(
        "Benchmarking API is not implemented",
        StatusCode::NOT_IMPLEMENTED,
        None,
    )
}

#[post("/generateProof")]
pub async fn generate_proof(_jsonbody: web::Json<common::InputPayload>) -> impl Responder {
    let proof = common::GenerateProofResponse {
        proof: "todo".into(),
    };
    let proof_json = serde_json::to_value(proof).unwrap();

    common::response(
        "Proof Generation API is not implemented",
        StatusCode::NOT_IMPLEMENTED,
        Some(proof_json),
    )
}

pub fn routes(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(test)
        .service(benchmark)
        .service(generate_proof);

    conf.service(scope);
}

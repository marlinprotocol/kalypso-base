use actix_web::{get, post};
use actix_web::{http::StatusCode, web, Responder};

#[get("/test")]
async fn test() -> impl Responder {
    common::response("Generator is running", StatusCode::OK, None)
}

#[get("/benchmark")]
async fn benchmark() -> impl Responder {
    common::response(
        "Benchmarking API is not implemented",
        StatusCode::NOT_IMPLEMENTED,
        None,
    )
}

#[post("/generateProof")]
async fn generate_proof(_jsonbody: web::Json<common::GenerateProofInputs>) -> impl Responder {
    common::response(
        "Proof Generation API is not implemented",
        StatusCode::NOT_IMPLEMENTED,
        None,
    )
}

pub fn routes(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(test)
        .service(benchmark)
        .service(generate_proof);

    conf.service(scope);
}

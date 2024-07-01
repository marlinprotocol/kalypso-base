use actix_web::web;

pub fn routes(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(prover::handler::test)
        .service(prover::handler::benchmark)
        .service(prover::handler::generate_proof)
        .service(ivs::handler::check_input_handler)
        .service(ivs::handler::generate_proof_for_invalid_inputs)
        .service(ivs::handler::check_enc_handler);

    conf.service(scope);
}

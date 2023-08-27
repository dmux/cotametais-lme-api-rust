mod api;
mod models;
mod repository;
use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use api::lme_api::create_lme;
use repository::mongodb_repos::MongoRepo;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hi!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(info(description = "LME API", version = "1.0.0", title = "LME API"))]

    struct ApiDoc;

    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(hello)
            .service(create_lme)
            .service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

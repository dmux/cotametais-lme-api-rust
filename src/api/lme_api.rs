use crate::{models::lme_model::Lme, repository::mongodb_repos::MongoRepo};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use mongodb::bson::oid::ObjectId;

#[post("/lme")]
pub async fn create_lme(db: Data<MongoRepo>, new_lme: Json<Lme>) -> HttpResponse {
    let data = Lme {
        id: None,
        data: new_lme.data.to_owned(),
        cobre: new_lme.cobre.to_owned(),
        zinco: new_lme.zinco.to_owned(),
        aluminio: new_lme.aluminio.to_owned(),
        chumbo: new_lme.chumbo.to_owned(),
        estanho: new_lme.estanho.to_owned(),
        niquel: new_lme.niquel.to_owned(),
        dolar: new_lme.dolar.to_owned(),
    };
    let lme_detail = db.create_lme(data).await;
    match lme_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/lme/{id}")]
pub async fn get_lme(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let lme_detail = db.get_lme(&id).await;

    match lme_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

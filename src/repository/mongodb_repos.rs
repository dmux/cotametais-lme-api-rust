use std::env;
extern crate dotenv;

use dotenv::dotenv;

use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};

use crate::models::lme_model::Lme;

pub struct MongoRepo {
    col: Collection<Lme>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("cotametais-lme-api-rust");
        let col: Collection<Lme> = db.collection("User");
        MongoRepo { col }
    }

    pub async fn create_lme(&self, new_lme: Lme) -> Result<InsertOneResult, Error> {
        let new_doc = Lme {
            id: None,
            data: new_lme.data,
            cobre: new_lme.cobre,
            zinco: new_lme.zinco,
            aluminio: new_lme.aluminio,
            chumbo: new_lme.chumbo,
            estanho: new_lme.estanho,
            niquel: new_lme.niquel,
            dolar: new_lme.dolar,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating lme");
        Ok(user)
    }

    pub async fn get_lme(&self, id: &String) -> Result<Lme, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let lme_detail = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting lme's detail");

        Ok(lme_detail.unwrap())
    }
}

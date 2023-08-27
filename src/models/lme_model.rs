use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Debug, Serialize, Deserialize)]
pub struct Lme {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub data: String,
    pub cobre: String,
    pub zinco: String,
    pub aluminio: String,
    pub chumbo: String,
    pub estanho: String,
    pub niquel: String,
    pub dolar: String,
}

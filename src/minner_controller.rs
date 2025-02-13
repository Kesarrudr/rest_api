use actix_web::{
    get, post,
    web::{self, Json, Path},
    HttpResponse,
};
use uuid::Uuid;

use crate::{
    get_connection_to_pool,
    minner::{create_new_miner, fetch_all_miners, fetch_miner_by_id, Minner, NewMinnerRequest},
    utils::{NotFoundMessage, ResponseType},
    DBPool,
};

//GET all the MINNERS
#[get("/miners")]
pub async fn list_minners(pool: web::Data<DBPool>) -> HttpResponse {
    //TODO: get the minners from the database;
    let mut con = get_connection_to_pool(pool);

    let minner: Vec<Minner> = fetch_all_miners(&mut con);

    ResponseType::Ok(minner).get_response()
}

//GET the minner by a specific ID
#[get("/miner/{id}")]
pub async fn get_minner(path: Path<Uuid>, pool: web::Data<DBPool>) -> HttpResponse {
    //TODO: get the specific minner form the DAtabase having id:ID;

    let mut coon = get_connection_to_pool(pool);
    let minner: Option<Minner> = fetch_miner_by_id(path.into_inner(), &mut coon);

    match minner {
        Some(minner) => ResponseType::Ok(minner).get_response(),
        None => ResponseType::NotFound(NotFoundMessage::new(
            "Minner not found with ID:".to_string(),
        ))
        .get_response(),
    }
}

//Create a minner
#[post("/wallet/{id}/miner")]
pub async fn create_minner(
    path: Path<Uuid>,
    miner_request: Json<NewMinnerRequest>,
    pool: web::Data<DBPool>,
) -> HttpResponse {
    //TODO: create a new minner
    let mut conn = crate::get_connection_to_pool(pool);

    match create_new_miner(miner_request.0, path.into_inner(), &mut conn) {
        Ok(created_miner) => ResponseType::Created(created_miner).get_response(),
        Err(_) => ResponseType::NotFound(NotFoundMessage::new("Error creating miner.".to_string()))
            .get_response(),
    }
}

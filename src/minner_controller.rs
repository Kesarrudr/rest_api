use actix_web::{get, post, HttpResponse};

use crate::{
    minner::Minner,
    utils::{NotFoundMessage, ResponseType},
};

//GET all the MINNERS
#[get("/minners")]
pub async fn list_minners() -> HttpResponse {
    //TODO: get the minners from the database;

    let minner: Vec<Minner> = vec![];

    ResponseType::Ok(minner).get_response()
}

//GET the minner by a specific ID
#[get("/minner/{id}")]
pub async fn get_minner() -> HttpResponse {
    //TODO: get the specific minner form the DAtabase having id:ID;

    let minner: Option<Minner> = None;

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
pub async fn create_minner() -> HttpResponse {
    //TODO: create a new minner

    let miner: Vec<Minner> = vec![];

    ResponseType::Created(miner).get_response()
}

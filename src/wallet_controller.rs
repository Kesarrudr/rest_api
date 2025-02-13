use actix_web::{get, post, HttpResponse};

use crate::{
    utils::{NotFoundMessage, ResponseType},
    wallet::Wallet,
};

#[get("/wallet")]
pub async fn list_wallet() -> HttpResponse {
    //TODO: get all the wallets from DB

    let wallet: Vec<Wallet> = vec![];

    ResponseType::Ok(wallet).get_response()
}

#[get("/wallet/{id}")]
pub async fn get_wallet() -> HttpResponse {
    //TODO: get a specific wallet from the DB

    let wallet: Option<Wallet> = None;

    match wallet {
        Some(wallet) => ResponseType::Ok(wallet).get_response(),
        None => ResponseType::NotFound(NotFoundMessage::new("Wallet/Club not found".to_string()))
            .get_response(),
    }
}

#[post("/wallet")]
pub async fn create_wallet() -> HttpResponse {
    //TODO: create a new wallet and store it in DB

    let wallet: Vec<Wallet> = vec![];

    ResponseType::Created(wallet).get_response()
}

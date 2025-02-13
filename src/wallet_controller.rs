use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse,
};
use uuid::Uuid;

use crate::{
    get_connection_to_pool,
    utils::{NotFoundMessage, ResponseType},
    wallet::{create_new_wallet, fetch_all_wallets, fetch_wallet_by_id, NewWalletRequest, Wallet},
    DBPool,
};

#[get("/wallet")]
pub async fn list_wallet(pool: Data<DBPool>) -> HttpResponse {
    let mut conn = get_connection_to_pool(pool);

    let wallet: Vec<Wallet> = fetch_all_wallets(&mut conn);

    ResponseType::Ok(wallet).get_response()
}

#[get("/wallet/{id}")]
pub async fn get_wallet(path: web::Path<Uuid>, pool: Data<DBPool>) -> HttpResponse {
    let mut conn = get_connection_to_pool(pool);
    let wallet: Option<Wallet> = fetch_wallet_by_id(path.into_inner(), &mut conn);

    match wallet {
        Some(wallet) => ResponseType::Ok(wallet).get_response(),
        None => ResponseType::NotFound(NotFoundMessage::new("Wallet/Club not found".to_string()))
            .get_response(),
    }
}
// Create New Wallet
#[post("/wallet")]
pub async fn create_wallet(
    wallet_request: web::Json<NewWalletRequest>,
    pool: Data<DBPool>,
) -> HttpResponse {
    let mut conn = crate::get_connection_to_pool(pool);
    println!("{:?}", wallet_request.0);

    match create_new_wallet(wallet_request.0, &mut conn) {
        Ok(created_wallet) => ResponseType::Created(created_wallet).get_response(),
        Err(_) => {
            ResponseType::NotFound(NotFoundMessage::new("Error creating wallet.".to_string()))
                .get_response()
        }
    }
}

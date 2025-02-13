use std::vec;

use crate::{
    minner::{Minner, MinnerDAO},
    schema::wallet,
    DBPooledConnection,
};
use diesel::{
    prelude::{Insertable, Queryable},
    query_dsl::methods::FilterDsl,
    result::Error,
    ExpressionMethods, RunQueryDsl,
};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Wallet {
    pub address: String,
    pub club_name: String,
    pub total_hash_rate: i32,
    pub total_shares_mined: i32,
    pub total_workers_online: i32,
    pub workers_online: Vec<Minner>,
}

impl Wallet {
    pub fn to_wallet_dao(&self) -> WalletDAO {
        WalletDAO {
            address: Uuid::parse_str(self.address.as_str()).unwrap(),
            club_name: self.club_name.to_string(),
        }
    }
}

//New minner request
#[derive(Debug, Deserialize, Serialize)]
pub struct NewWalletRequest {
    club_name: String,
}

//Data base
#[derive(Queryable, Insertable)]
#[diesel(table_name = wallet)]
pub struct WalletDAO {
    pub address: Uuid,
    pub club_name: String,
}

impl WalletDAO {
    pub fn to_wallet(&self, workers_online: Vec<Minner>) -> Wallet {
        Wallet {
            address: self.address.to_string(),
            club_name: self.club_name.to_string(),
            total_hash_rate: workers_online.iter().map(|w| w.hash_rate).sum(),
            total_shares_mined: workers_online.iter().map(|w| w.shares_mined).sum(),
            total_workers_online: workers_online.len() as i32,
            workers_online,
        }
    }
}

pub fn get_workers_online(_wallet_dao: &WalletDAO, conn: &mut DBPooledConnection) -> Vec<Minner> {
    use crate::schema::miner::dsl::*;

    match miner
        .filter(address.eq(&_wallet_dao.address))
        .load::<MinnerDAO>(conn)
    {
        Ok(result) => result
            .into_iter()
            .map(|m| m.to_minner(_wallet_dao.club_name.clone()))
            .collect::<Vec<Minner>>(),
        Err(_) => vec![], // Fixed missing `Err` arm
    }
}

pub fn fetch_wallet_by_id(_address: Uuid, conn: &mut DBPooledConnection) -> Option<Wallet> {
    use crate::schema::wallet::dsl::*;

    match wallet.filter(address.eq(_address)).load::<WalletDAO>(conn) {
        Ok(result) => match result.first() {
            Some(matched_wallet) => {
                let workers_online = get_workers_online(matched_wallet, conn);
                Some(matched_wallet.to_wallet(workers_online))
            }
            _ => None,
        },
        Err(_) => None,
    }
}

pub fn fetch_all_wallets(conn: &mut DBPooledConnection) -> Vec<Wallet> {
    use crate::schema::wallet::dsl::*;

    match wallet.load::<WalletDAO>(conn) {
        Ok(result) => result
            .into_iter()
            .map(|w| {
                let workers_online = get_workers_online(&w, conn);
                w.to_wallet(workers_online)
            })
            .collect::<Vec<Wallet>>(),
        Err(_) => vec![],
    }
}

pub fn create_new_wallet(
    new_wallet: NewWalletRequest,
    conn: &mut DBPooledConnection,
) -> Result<Wallet, Error> {
    use crate::schema::wallet::dsl::*;

    let new_wallet = Wallet {
        address: Uuid::new_v4().to_string(),
        club_name: new_wallet.club_name.to_string(),
        total_hash_rate: 0,
        total_shares_mined: 0,
        total_workers_online: 0,
        workers_online: vec![],
    };

    let new_wallet_dao = new_wallet.to_wallet_dao();

    match diesel::insert_into(wallet)
        .values(&new_wallet_dao)
        .execute(conn)
    {
        Ok(_) => Ok(new_wallet_dao.to_wallet(vec![])),
        Err(e) => Err(e),
    }
}

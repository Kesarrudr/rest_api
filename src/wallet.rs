use std::vec;

use crate::{
    minner::{fetch_all_miners, Minner, MinnerDAO},
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

pub fn fetch_wallet_by_id(_address: Uuid, conn: &mut DBPooledConnection) -> Option<Wallet> {
    use crate::schema::miner::dsl::*;
    use crate::schema::wallet::dsl::*;

    match wallet
        .filter(crate::schema::wallet::address.eq(_address))
        .load::<WalletDAO>(conn)
    {
        Ok(result) => match result.first() {
            Some(match_wallet_dao) => {
                match miner
                    .filter(crate::schema::miner::address.eq(_address))
                    .load::<MinnerDAO>(conn)
                {
                    Ok(result) => Some(
                        match_wallet_dao.to_wallet(
                            result
                                .into_iter()
                                .map(|m| m.to_minner(match_wallet_dao.club_name.clone()))
                                .collect::<Vec<Minner>>(),
                        ),
                    ),
                    Err(_e) => Some(match_wallet_dao.to_wallet(vec![])),
                }
            }
            _ => None,
        },
        Err(_e) => None,
    }
}

pub fn fetch_all_wallets(conn: &mut DBPooledConnection) -> Vec<Wallet> {
    use crate::schema::wallet::dsl::*;
    //
    // let all_wallet = match wallet.load::<WalletDAO>(conn) {
    //     Ok(result) => result,
    //     Err(_) => vec![],
    // };

    let all_wallet = wallet.load::<WalletDAO>(conn).unwrap_or_default();

    let all_miners = fetch_all_miners(conn);

    all_wallet
        .into_iter()
        .map(|w| {
            let mut workers_online: Vec<Minner> = vec![];

            for m in &all_miners {
                if m.address.eq(&w.address.to_string()) {
                    workers_online.push(m.clone());
                }
            }
            w.to_wallet(workers_online)
        })
        .collect::<Vec<Wallet>>()
}

pub fn create_new_wallet(
    new_wallet: NewWalletRequest,
    conn: &mut DBPooledConnection,
) -> Result<Wallet, Error> {
    use crate::schema::wallet::dsl::*;

    let new_wallet_dao = WalletDAO {
        address: Uuid::new_v4(),
        club_name: new_wallet.club_name.to_string(),
    };

    match diesel::insert_into(wallet)
        .values(&new_wallet_dao)
        .execute(conn)
    {
        Ok(_) => Ok(new_wallet_dao.to_wallet(vec![])),
        Err(e) => Err(e),
    }
}

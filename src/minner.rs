use diesel::prelude::{Insertable, Queryable};
use diesel::result::Error;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rand::Rng;

use crate::{schema::miner, wallet::fetch_wallet_by_id, DBPooledConnection};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Minner {
    pub id: String,
    pub address: String,
    pub club_name: String,
    pub nickname: String,
    pub hash_rate: i32,
    pub shares_mined: i32,
}

impl Minner {
    pub fn to_miner_dao(&self) -> MinnerDAO {
        MinnerDAO {
            id: Uuid::parse_str(self.id.as_str()).unwrap(),
            address: Uuid::parse_str(self.address.as_str()).unwrap(),
            nickname: self.nickname.to_string(),
            hash_rate: self.hash_rate,
            shares_mined: self.shares_mined,
        }
    }
}

//New minner request
#[derive(Debug, Deserialize, Serialize)]
pub struct NewMinnerRequest {
    nickname: String,
}

//Data base
#[derive(Queryable, Insertable)]
#[diesel(table_name = miner)]
pub struct MinnerDAO {
    pub id: Uuid,
    pub address: Uuid,
    pub nickname: String,
    pub hash_rate: i32,
    pub shares_mined: i32,
}

impl MinnerDAO {
    pub fn to_minner(&self, club_name: String) -> Minner {
        Minner {
            id: self.id.to_string(),
            address: self.address.to_string(),
            club_name,
            nickname: self.nickname.to_string(),
            hash_rate: self.hash_rate,
            shares_mined: self.shares_mined,
        }
    }
}

pub fn get_club_name(_address: Uuid, conn: &mut DBPooledConnection) -> String {
    match fetch_wallet_by_id(_address, conn) {
        Some(matched_wallet) => matched_wallet.club_name,
        None => "Club name not found".to_string(),
    }
}

pub fn fetch_all_miners(conn: &mut DBPooledConnection) -> Vec<Minner> {
    use crate::schema::miner::dsl::*;

    match miner.load::<MinnerDAO>(conn) {
        Ok(result) => result
            .into_iter()
            .map(|x| {
                let club_name = get_club_name(x.address, conn);
                x.to_minner(club_name)
            })
            .collect::<Vec<Minner>>(),
        Err(_) => vec![],
    }
}

pub fn fetch_miner_by_id(_id: Uuid, conn: &mut DBPooledConnection) -> Option<Minner> {
    use crate::schema::miner::dsl::*;

    match miner.filter(id.eq(_id)).load::<MinnerDAO>(conn) {
        Ok(result) => match result.first() {
            Some(matched_miner) => {
                let club_name = get_club_name(matched_miner.address, conn);
                Some(matched_miner.to_minner(club_name))
            }
            _ => None,
        },
        Err(_) => None,
    }
}

pub fn create_new_miner(
    newminer: NewMinnerRequest,
    _address: Uuid,
    conn: &mut DBPooledConnection,
) -> Result<Minner, Error> {
    use crate::schema::miner::dsl::*;

    let club_name = get_club_name(_address, conn);

    let new_minner = Minner {
        id: Uuid::new_v4().to_string(),
        address: _address.to_string(),
        nickname: newminer.nickname.to_string(),
        hash_rate: rand::rng().random_range(20..100),
        shares_mined: rand::rng().random_range(1..40),
        club_name,
    };

    let new_miner_dao = new_minner.to_miner_dao();

    match diesel::insert_into(miner)
        .values(&new_miner_dao)
        .execute(conn)
    {
        Ok(_) => Ok(new_minner),
        Err(e) => Err(e),
    }
}

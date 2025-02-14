use diesel::prelude::{Insertable, Queryable};
use diesel::result::Error;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rand::Rng;

use crate::wallet::WalletDAO;
use crate::{schema::miner, DBPooledConnection};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
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

pub fn fetch_all_miners(conn: &mut DBPooledConnection) -> Vec<Minner> {
    use crate::schema::miner::dsl::*;
    use crate::schema::wallet::dsl::*;

    match wallet
        .inner_join(miner)
        .load::<(WalletDAO, MinnerDAO)>(conn)
    {
        Ok(result) => result
            .into_iter()
            .map(|(w, m)| m.to_minner(w.club_name))
            .collect::<Vec<Minner>>(),
        Err(_) => vec![],
    }
}

pub fn fetch_miner_by_id(_id: Uuid, conn: &mut DBPooledConnection) -> Option<Minner> {
    use crate::schema::miner::dsl::*;
    use crate::schema::wallet::dsl::*;

    match wallet
        .inner_join(miner)
        .filter(id.eq(_id))
        .load::<(WalletDAO, MinnerDAO)>(conn)
    {
        Ok(result) => match result.first() {
            Some((w, m)) => Some(m.to_minner(w.club_name.clone())),
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

    let new_miner_dao = MinnerDAO {
        id: Uuid::new_v4(),
        address: _address,
        nickname: newminer.nickname.to_string(),
        hash_rate: rand::rng().random_range(20..100),
        shares_mined: rand::rng().random_range(1..40),
    };

    match diesel::insert_into(miner)
        .values(&new_miner_dao)
        .execute(conn)
    {
        Ok(_) => match fetch_miner_by_id(new_miner_dao.id, conn) {
            Some(result) => Ok(result),
            None => Err(Error::NotFound),
        },
        Err(e) => Err(e),
    }
}

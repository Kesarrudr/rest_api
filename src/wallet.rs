use crate::minner::Minner;

#[derive(Debug, Deserialize, Serialize)]
pub struct Wallet {
    pub address: String,
    pub club_name: String,
    pub total_hash_rate: String,
    pub total_shares_mined: i32,
    pub total_workers_online: i32,
    pub workers_online: Vec<Minner>,
}

//New minner request
#[derive(Debug, Deserialize, Serialize)]
pub struct NewWalletRequest {
    nickname: String,
}

//Data base
#[derive(Debug, Deserialize, Serialize)]
pub struct WalletDAO {
    pub address: String,
    pub club_name: String,
}

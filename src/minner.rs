#[derive(Debug, Deserialize, Serialize)]
pub struct Minner {
    pub id: String,
    pub address: String,
    pub club_name: String,
    pub nickname: String,
    pub hash_rate: i32,
    pub share_mined: i32,
}

//New minner request
#[derive(Debug, Deserialize, Serialize)]
pub struct NewMinnerRequest {
    nickname: String,
}

//Data base
#[derive(Debug, Deserialize, Serialize)]
pub struct MinnerDAO {
    pub id: String,
    pub address: String,
    pub nickname: String,
    pub hash_rate: i32,
    pub share_mined: i32,
}

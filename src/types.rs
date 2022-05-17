use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetResponse {
    pub key: String,
    pub data: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostResponse {
    pub key: String,
    pub secret: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteResponse {
    pub deleted: bool,
    pub key: String
}
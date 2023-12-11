use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub age: u64,
}

pub struct Music {
    pub id: String,
    pub name: String,
    pub author: String,
    pub playlist_id: String,
}

pub struct Playlist {
    pub id: String,
    pub name: String,
    pub user_id: String,
}

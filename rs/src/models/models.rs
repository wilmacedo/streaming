use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub age: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Music {
    pub id: String,
    pub name: String,
    pub author: String,
    #[serde(rename = "playlistId")]
    pub playlist_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Data {
    pub users: Vec<User>,
    pub musics: Vec<Music>,
    pub playlists: Vec<Playlist>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
    pub code: u64,
}

use std::sync::Arc;

use crate::models::models::{Data, ErrorResponse};

pub async fn find(id: String, data: Arc<Data>) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    match data.playlists.iter().find(|playlist| playlist.id == id) {
        Some(playlist) => Ok(Box::new(warp::reply::json(&playlist))),
        None => Ok(Box::new(warp::reply::with_status(
            warp::reply::json(&ErrorResponse {
                message: String::from("playlist not found"),
                code: 404,
            }),
            warp::http::StatusCode::NOT_FOUND,
        ))),
    }
}

use std::sync::Arc;

use crate::models::models::{Data, ErrorResponse};

pub async fn find(id: String, data: Arc<Data>) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    match data.musics.iter().find(|music| music.id == id) {
        Some(music) => Ok(Box::new(warp::reply::json(&music))),
        None => Ok(Box::new(warp::reply::with_status(
            warp::reply::json(&ErrorResponse {
                message: String::from("music not found"),
                code: 404,
            }),
            warp::http::StatusCode::NOT_FOUND,
        ))),
    }
}

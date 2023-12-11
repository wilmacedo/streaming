use std::sync::Arc;

use crate::models::models::{Data, ErrorResponse};

pub async fn find(id: String, data: Arc<Data>) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    match data.users.iter().find(|user| user.id == id) {
        Some(user) => Ok(Box::new(warp::reply::json(&user))),
        None => Ok(Box::new(warp::reply::with_status(
            warp::reply::json(&ErrorResponse {
                message: String::from("user not found"),
                code: 404,
            }),
            warp::http::StatusCode::NOT_FOUND,
        ))),
    }
}

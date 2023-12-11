use std::sync::Arc;

use warp::Filter;

use crate::{handlers, models::models::Data};

use super::router::with_data;

pub fn users_routes(
    data: Arc<Data>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    find_user(data.clone()).or(list_users(data.clone()))
}

fn find_user(
    data: Arc<Data>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("users" / String)
        .and(warp::get())
        .and(with_data(data))
        .and_then(handlers::user::find)
}

fn list_users(
    data: Arc<Data>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("users")
        .and(warp::get())
        .and(with_data(data))
        .and_then(handlers::user::list)
}

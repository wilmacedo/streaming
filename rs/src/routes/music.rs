use std::sync::Arc;

use warp::Filter;

use crate::{handlers, models::models::Data};

use super::router::with_data;

pub fn musics_routes(
    data: Arc<Data>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    find_music(data.clone()).or(list_musics(data.clone()))
}

fn find_music(
    data: Arc<Data>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("musics" / String)
        .and(warp::get())
        .and(with_data(data))
        .and_then(handlers::music::find)
}

fn list_musics(
    data: Arc<Data>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("musics")
        .and(warp::get())
        .and(with_data(data))
        .and_then(handlers::music::list)
}

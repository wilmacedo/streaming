use std::sync::Arc;

use warp::Filter;

use crate::{handlers, models::models::Data};

use super::router::with_data;

pub fn playlists_routes(
    data: Arc<Data>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    find_playlist(data.clone()).or(list_playlists(data.clone()))
}

fn find_playlist(
    data: Arc<Data>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("playlists" / String)
        .and(warp::get())
        .and(with_data(data))
        .and_then(handlers::playlist::find)
}

fn list_playlists(
    data: Arc<Data>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("playlists")
        .and(warp::get())
        .and(with_data(data))
        .and_then(handlers::playlist::list)
}

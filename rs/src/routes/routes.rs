use std::sync::Arc;

use warp::Filter;

use crate::{handlers, models::models::Data};

pub fn routes(
    data: Arc<Data>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    find_user(data.clone())
        .or(find_music(data.clone()))
        .or(find_playlist(data.clone()))
}

fn with_data(
    data: Arc<Data>,
) -> impl Filter<Extract = (Arc<Data>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || data.clone())
}

fn find_user(
    data: Arc<Data>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("users" / String)
        .and(warp::get())
        .and(with_data(data))
        .and_then(handlers::user::find)
}

fn find_music(
    data: Arc<Data>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("musics" / String)
        .and(warp::get())
        .and(with_data(data))
        .and_then(handlers::music::find)
}

fn find_playlist(
    data: Arc<Data>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("playlists" / String)
        .and(warp::get())
        .and(with_data(data))
        .and_then(handlers::playlist::find)
}

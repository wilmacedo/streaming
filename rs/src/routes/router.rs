use std::sync::Arc;

use warp::Filter;

use crate::models::models::Data;

use super::{music::musics_routes, playlist::playlists_routes, user::users_routes};

pub fn routes(
    data: Arc<Data>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    users_routes(data.clone())
        .or(musics_routes(data.clone()))
        .or(playlists_routes(data.clone()))
}

pub fn with_data(
    data: Arc<Data>,
) -> impl Filter<Extract = (Arc<Data>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || data.clone())
}

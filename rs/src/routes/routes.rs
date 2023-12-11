use warp::Filter;

use crate::handlers;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    find_user()
}

fn find_user() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("users" / String)
        .and(warp::get())
        .and_then(handlers::user::find)
}

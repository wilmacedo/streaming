use crate::handlers;
use warp::Filter;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_users()
}

fn get_users() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("users" / String)
        .and(warp::get())
        .and_then(handlers::get_user)
}

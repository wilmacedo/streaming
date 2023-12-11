use crate::models::User;

pub async fn get_user(id: String) -> Result<impl warp::Reply, warp::Rejection> {
    let user = User {
        id,
        name: String::from("Hello, Warp!"),
        age: 10,
    };

    Ok(warp::reply::json(&user))
}

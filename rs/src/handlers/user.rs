use crate::models::models::User;

pub async fn find(id: String) -> Result<impl warp::Reply, warp::Rejection> {
    let user = User {
        id,
        name: String::from("Wilson"),
        age: 24,
    };

    Ok(warp::reply::json(&user))
}

use crate::routes::routes::routes;
use console::Style;

mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let target: String = "0.0.0.0:8000".parse().unwrap();
    let blue = Style::new().blue();
    let r = routes();

    println!("\nServer ready at {}", blue.apply_to(&target));

    warp::serve(r).run(([0, 0, 0, 0], 8000)).await;
}

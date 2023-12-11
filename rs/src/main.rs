use std::{error::Error, sync::Arc};

use crate::routes::router;
use console::Style;
use models::models::Data;

mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let target: String = "0.0.0.0:8000".parse().unwrap();
    let blue = Style::new().blue();

    let data = load_data().expect("Failed to load data");
    let data = Arc::new(data);
    let routes = router::routes(data);

    println!("\nServer ready at {}", blue.apply_to(&target));
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

fn load_data() -> Result<Data, Box<dyn Error>> {
    let data = std::fs::read_to_string("../database.json")?;
    let data: Data = serde_json::from_str(&data)?;

    Ok(data)
}

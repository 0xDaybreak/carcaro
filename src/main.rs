mod db;
mod types;

use std::collections::HashMap;
use warp::{Filter, http::Method, Rejection, Reply};

#[tokio::main]
async fn main() {

    let db = db::Connection::new("postgres://postgres:a@localhost:5432/carcaro").await;
    let db_filter = warp::any().map(move || db.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(
       &[Method::PUT, Method::DELETE, Method::GET, Method::POST]
        );


    let get_cars = warp::get()
        .and(warp::path("cars"))
        .and(warp::path::end())
        .and(warp::query())
        .and(db_filter.clone())
        .and_then(get_cars_with_images);

    let routes = get_cars.with(cors);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 7070))
        .await;

}

pub async fn get_cars_with_images(
    params: HashMap<String, String>,
    db: db::Connection
) -> Result<impl Reply, Rejection> {
    let res = match db.get_cars_with_images()
        .await {
        Ok( res) => res,
        Err(e) => {
            return Err(warp::reject::not_found())
        }
    };

    return Ok(warp::reply::json(&res));
}
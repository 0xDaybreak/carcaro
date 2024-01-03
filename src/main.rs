mod db;
mod types;
mod handle_errors;
mod functionality;

use std::collections::HashMap;
use reqwest::StatusCode;
use warp::{Filter, http::Method, Rejection, Reply};
use crate::functionality::{color_swap, container_generation};
use crate::types::carparams::{CarParams, extract_car_params};
use crate::types::image::{Image, NewImage};
use crate::types::image_request::ImageRequest;

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
        .and(db_filter.clone())
        .and_then(get_cars_with_images);

    let get_cars_to_visualize = warp::get()
        .and(warp::path("cars"))
        .and(warp::path("visualize"))
        .and(warp::path::end())
        .and(warp::query())
        .and(db_filter.clone())
        .and_then(get_car_to_visualize);

    let post_new_image = warp::post()
        .and(warp::path("cars"))
        .and(warp::path("newimage"))
        .and(db_filter.clone())
        .and(warp::body::json())
        .and_then(post_new_image);


    let routes = get_cars
        .or(get_cars_to_visualize)
        .or(post_new_image)
        .with(cors);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 7070))
        .await;

}

pub async fn get_cars_with_images(
    db: db::Connection
) -> Result<impl Reply, Rejection> {
    let res = match db.get_cars_with_images()
        .await {
        Ok( res) => res,
        Err(e) => {
            eprintln!("Error {}", e);
            return Err(warp::reject::not_found())
        }
    };

    return Ok(warp::reply::json(&res));
}

pub async fn post_chosen_color(
    db: db::Connection
) -> Result<impl Reply, Rejection> {
    /*
    if let Err(e) = db.post_chosen_color()
        .await {
            return Err(warp::reject::not_found())
    }

     */
    Ok(warp::reply::with_status(
"Color posted successfully",
        StatusCode::OK,
    ))
}

pub async fn get_car_to_visualize(
    params: HashMap<String, String>,
    db: db::Connection,
) -> Result<impl Reply, Rejection> {
    let mut car_params = CarParams::default();
    if !params.is_empty() {
        car_params = extract_car_params(params)?;
    }
    let res = match db.get_car_to_visualize(car_params.make, car_params.model, car_params.year)
        .await {
        Ok(res) => res,
        Err(e) => {
            return Err(warp::reject::not_found())
        }
    };
    Ok(warp::reply::json(&res))
}


pub async fn post_new_image(
    db: db::Connection,
    image: Image
) -> Result<impl Reply, Rejection> {
    let mask = match db.extract_mask(image.id.0)
        .await {
        Ok(mask) => mask,
        Err(e) => {
            return Err(warp::reject::not_found())
        }
    };

    let image_request = ImageRequest {
        image,
        mask,
    };
    color_swap::color_swap(image_request.image.url, image_request.image.colors, image_request.mask.url).await?;

    let new_image_urls = container_generation::generate_and_upload("test10".to_string()).await.unwrap();
    let new_image = NewImage {
        url:new_image_urls,
        colors: image_request.image.colors,
        maskid: 1,
    };

    let res = match db.add_new_image(new_image)
        .await {
        Ok(res) => res,
        Err(e) => {
            return Err(warp::reject::not_found())
        }
    };
    Ok(warp::reply::json(&res))
}
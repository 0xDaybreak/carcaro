use sqlx::{Error, query, Row};
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use warp::hyper::body::HttpBody;
use crate::types::car::{Car, CarId};
use crate::types::image::{Image, ImageId};

#[derive(Clone)]
pub struct Connection {
    pub connection: PgPool,
}

impl Connection {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url).await {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection")
        };

        Connection {
            connection: db_pool,
        }
    }

    pub async fn get_cars_with_images(&self) -> Result<Vec<(Car, Image)>, Error> {
        let query = query(
        r#"
        SELECT *
        FROM car
        INNER JOIN image ON car.imageid = image.imageid
        "#
    );

        let result = match query.fetch_all(&self.connection).await {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Error executing query: {:?}", e);
                return Err(Error::RowNotFound);
            }
        };
        let cars_with_images: Vec<(Car, Image)> = result
            .into_iter()
            .map(|row| {
                let car = Car {
                    id: CarId(row.get("carid")),
                    make: row.get("make"),
                    model: row.get("model"),
                    year: row.get("year"),
                    color_id: row.get("colorid"),
                    image_id: row.get("imageid"),
                };
                let image = Image {
                    id: ImageId(row.get("imageid")),
                    url: row.get("url"),
                };
                (car, image)
            })
            .collect();

        Ok(cars_with_images)
    }

}


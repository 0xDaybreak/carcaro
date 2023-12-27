use sqlx::{Error, query, Row};
use sqlx::postgres::{PgPool, PgPoolOptions};
use warp::hyper::body::HttpBody;
use crate::types::car::{Car, CarId};

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
            Err(e) => panic!("Couldn't establish DB connection {}", e)
        };

        Connection {
            connection: db_pool,
        }
    }

    pub async fn get_cars_with_images(&self) -> Result<Vec<Car>, Error> {
        let query = query(
            r#"
        SELECT *
        FROM car
        "#
        );

        let result = match query.fetch_all(&self.connection).await {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Error executing query: {:?}", e);
                return Err(Error::RowNotFound);
            }
        };
        let cars_with_images: Vec<Car> = result
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
                car
            })
            .collect();

        Ok(cars_with_images)
    }
}


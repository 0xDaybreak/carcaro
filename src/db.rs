use sqlx::{Error, query, Row};
use sqlx::postgres::{PgPool, PgPoolOptions};
use warp::hyper::body::HttpBody;
use crate::types::car::{Car, CarId};
use crate::types::image::{Image, ImageId, NewImage};

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

    pub async fn get_car_to_visualize(&self, make: String, model: String, year: i32) -> Result<Image, Error> {
        let query = query(
            r#"
            SELECT image.imageid, image.url, image.colors
            FROM image
            INNER JOIN car ON image.imageid = car.imageid
            WHERE car.make = $1 AND car.model = $2 AND car.year = $3
            "#
        )
            .bind(&make)
            .bind(&model)
            .bind(year);

        let res = match query.fetch_one(&self.connection).await {
            Ok(res) => res,
            Err(e) => {
                eprintln!("Error querying db {:?}", e);
                return Err(Error::RowNotFound);
            }
        };
        let images = Image {
            id: ImageId(res.get("imageid")),
            url: res.get("url"),
            colors: res.get("colors"),
        };

        Ok(images)
    }

    pub async fn add_new_image(
        &self,
        new_image: NewImage,
    ) -> Result<Image, Error> {
        let query = sqlx::query(
            r#"
            INSERT INTO image (url, colors)
            VALUES($1, $2)
            RETURNING imageid, url, colors
        "#)
            .bind(new_image.url)
            .bind(new_image.colors)
            .map(|row| Image {
                id: ImageId(row.get("imageid")),
                url: row.get("url"),
                colors: row.get("colors"),
            });

        match query.fetch_one(&self.connection).await {
            Ok(res) => Ok(res),
            Err(e) => {
                eprintln!("Database error {:?}", e);
                Err(Error::RowNotFound)
            }
        }
    }
}


use crate::types::car::{Car, CarId};
use crate::types::color::Color;
use crate::types::image::{Image, ImageId, NewImage};
use crate::types::image_request::ImageRequest;
use crate::types::user::{NewUser, User, UserCredentials, UserId};
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::{query, Error, Row};
use crate::types::favorite::Favorite;

#[derive(Clone)]
pub struct Connection {
    pub connection: PgPool,
}

impl Connection {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection {}", e),
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
        "#,
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

    pub async fn get_car_to_visualize(
        &self,
        make: String,
        model: String,
        year: i32,
    ) -> Result<Image, Error> {
        let query = query(
            r#"
            SELECT image.imageid, image.url, image.colors
            FROM image
            INNER JOIN car ON image.imageid = car.imageid
            WHERE car.make = $1 AND car.model = $2 AND car.year = $3
            "#,
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
            userid: None,
        };

        Ok(images)
    }

    pub async fn add_new_image(&self, new_image: NewImage) -> Result<Image, Error> {
        let query = sqlx::query(
            r#"
            INSERT INTO image (url, colors, userid)
            VALUES($1, $2, $3)
            RETURNING imageid, url, colors, userid
        "#,
        )
        .bind(new_image.url)
        .bind(new_image.colors)
        .bind(new_image.userid)
        .map(|row| Image {
            id: ImageId(row.get("imageid")),
            url: row.get("url"),
            colors: row.get("colors"),
            userid: row.get("userid"),
        });

        match query.fetch_one(&self.connection).await {
            Ok(res) => Ok(res),
            Err(e) => {
                eprintln!("Database error {:?}", e);
                Err(Error::RowNotFound)
            }
        }
    }

    pub async fn extract_image(&self, imageid: i32) -> Result<ImageRequest, Error> {
        let query = sqlx::query(
            r#"
                SELECT image.url
                FROM image
                WHERE image.imageid = $1
            "#,
        )
        .bind(imageid)
        .map(|row: PgRow| ImageRequest {
            url: row.get("url"),
            colors: [0, 0, 0],
        });

        match query.fetch_one(&self.connection).await {
            Ok(res) => Ok(res),
            Err(e) => {
                eprintln!("Error {}", e);
                Err(Error::RowNotFound)
            }
        }
    }

    pub async fn get_colors(&self) -> Result<Vec<Color>, Error> {
        let query = sqlx::query(
            r#"
            SELECT color.ral, color.name, color.hex
            FROM color
            "#,
        );

        let result = match query.fetch_all(&self.connection).await {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Error executing query: {:?}", e);
                return Err(Error::RowNotFound);
            }
        };
        let colors: Vec<Color> = result
            .into_iter()
            .map(|row| {
                let color = Color {
                    ral: row.get("ral"),
                    color_name: row.get("name"),
                    hex: row.get("hex"),
                };
                color
            })
            .collect();

        Ok(colors)
    }

    pub async fn get_user_favorites(&self, userid: UserId) -> Result<Vec<Favorite>, Error> {
        let query = sqlx::query(
            r#"
            SELECT image.colors, car.make, car.model
            FROM image
            INNER JOIN car ON image.carid = car.carid
            WHERE image.userid = $1
            "#,
        ).bind(userid.0);

        let result = match query.fetch_all(&self.connection).await {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Error executing query: {:?}", e);
                return Err(Error::RowNotFound);
            }
        };
        let favorites: Vec<Favorite> = result
            .into_iter()
            .map(|row| {
                let favorite = Favorite {
                    make: row.get("make"),
                    model: row.get("model"),
                    colors: row.get("colors"),
                };
                favorite
            })
            .collect();
        println!("{:?}",favorites);
        Ok(favorites)
    }

    pub async fn get_user_by_email(&self, email: &String) -> Result<UserCredentials, Error> {
        let query = sqlx::query(
            r#"
            SELECT "user".email, "user".password
            FROM "user"
            WHERE "user".email = $1
        "#,
        )
        .bind(email)
        .map(|row: PgRow| UserCredentials {
            email: row.get("email"),
            password: row.get("password"),
        });

        match query.fetch_one(&self.connection).await {
            Ok(res) => Ok(res),
            Err(e) => {
                eprintln!("Error {}", e);
                Err(Error::RowNotFound)
            }
        }
    }

    pub async fn create_user(&self, new_user: NewUser) -> Result<User, Error> {
        let query = sqlx::query(
            r#"
            INSERT INTO "user" (email, firstname, lastname, password, phonenumber)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING userid, email, firstname, lastname, password, phonenumber
            "#,
        )
        .bind(new_user.email)
        .bind(new_user.firstname)
        .bind(new_user.lastname)
        .bind(new_user.password_hash)
        .bind(new_user.phone_number)
        .map(|row: PgRow| User {
            id: UserId(row.get("userid")),
            email: row.get("email"),
            firstname: row.get("firstname"),
            lastname: row.get("lastname"),
            password_hash: row.get("password"),
            phone_number: row.get("phonenumber"),
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

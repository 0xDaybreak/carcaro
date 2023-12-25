use serde::{Deserialize, Serialize};
use crate::types::image::Image;

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct CarId(pub i32);

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Car {
    pub id: CarId,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub color_id: i32,
    pub image_id: i32,
}
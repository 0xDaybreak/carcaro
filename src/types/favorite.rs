use serde::{Deserialize, Serialize};
use crate::types::image::ImageId;
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Favorite {
    pub make: String,
    pub model: String,
    pub colors: [u8; 3],
}
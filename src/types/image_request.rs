use serde::{Deserialize, Serialize};
use crate::types::image::Image;

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageRequest {
    pub url: Vec<String>,
    pub colors: [u8;3],
}
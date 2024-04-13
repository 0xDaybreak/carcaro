use crate::types::image::Image;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageRequest {
    pub url: Vec<String>,
    pub colors: [u8; 3],
}

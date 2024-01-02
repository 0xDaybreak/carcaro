use serde::{Deserialize, Serialize};
use crate::types::car::Car;
use crate::types::image::Image;
use crate::types::mask::Mask;

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageRequest {
    pub image: Image,
    pub mask: Mask
}
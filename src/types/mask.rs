use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Mask {
    pub id: i32,
    pub url: Vec<String>
}


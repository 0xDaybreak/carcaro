use serde::{Deserialize, Serialize};

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct ImageId(pub i32);

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Image {
    pub id: ImageId,
    pub url: Vec<String>,
    pub colors: [u8; 3],
    pub userid: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewImage {
    pub url: Vec<String>,
    pub colors: [u8; 3],
    pub userid: Option<i32>,
}

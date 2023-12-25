use serde::{Deserialize, Serialize};

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct ImageId(pub i32);


#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Image {
    pub id: ImageId,
    pub url: Vec<String>,
}
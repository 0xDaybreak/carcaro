use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Color {
    pub ral: String,
    pub color_name: String,
    pub hex: String,
}

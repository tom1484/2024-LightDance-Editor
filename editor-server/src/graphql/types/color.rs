use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::db::types::ColorData;

#[derive(SimpleObject, Serialize, Deserialize, Default)]
pub struct Color {
    pub id: i32,
    pub color: String,
    pub color_code: Vec<i32>,
}

impl From<ColorData> for Color {
    fn from(data: ColorData) -> Self {
        Self {
            id: data.id.clone(),
            color: data.name.clone(),
            color_code: vec![data.r, data.g, data.b],
        }
    }
}

// impl Into<ColorData> for Color {
//     fn into(self) -> ColorData {
//         ColorData {
//             id: self.id,
//             name: self.color,
//             r: self.color_code[0],
//             g: self.color_code[1],
//             b: self.color_code[2],
//         }
//     }
// }

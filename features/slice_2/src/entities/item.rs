use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Slice2Item {
    pub id: String,
    pub title: String,
    pub value: i32,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Slice1Something {
    pub id: String,
    pub name: String,
    pub description: String,
}

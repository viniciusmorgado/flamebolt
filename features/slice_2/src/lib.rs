mod entities;

use axum::{routing::get, Json, Router};
use entities::item::Slice2Item;

#[axum::debug_handler]
pub async fn get_item() -> Json<Slice2Item> {
    Json(Slice2Item {
        id: "2".to_string(),
        title: "Slice 2 Item".to_string(),
        value: 42,
    })
}

pub fn router() -> Router {
    Router::new().route("/item", get(get_item))
}

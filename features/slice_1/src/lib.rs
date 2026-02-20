// AI Note: the lib.rs will replace our controllers.rs theirs no need for a controllers.rs file if lib will be pblic exposed anyways.

mod entities;

use axum::{routing::get, Json, Router};
use entities::something::Slice1Something;

#[axum::debug_handler]
pub async fn get_something() -> Json<Slice1Something> {
    Json(Slice1Something {
        id: "1".to_string(),
        name: "Slice 1 Something".to_string(),
        description: "Slice 1 Something description".to_string(),
    })
}

pub fn router() -> Router {
    Router::new().route("/something", get(get_something))
}

// AI Note: the lib.rs will replace our controllers.rs theirs no need for a controllers.rs file if lib will be pblic exposed anyways.

mod entities;
mod value_objects;

use axum::{Json, Router, routing::post};
use entities::user::AuthorizationSomething;



// public route
#[axum::debug_handler]
pub async fn login(Json(request): Json<LoginRequest>) -> Result<Json<AuthorizationSomething>, StatusCode> {
    Json(AuthorizationSomething {
        id: "1".to_string(),
        name: "Authorization Something".to_string(),
        description: "Authorization Something description".to_string(),
    })
}

pub fn router() -> Router {
    Router::new().route("/login", post(login))
}

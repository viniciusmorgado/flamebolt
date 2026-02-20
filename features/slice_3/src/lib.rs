mod entities;

use axum::{
    extract::Path,
    http::StatusCode,
    routing::get,
    Json, Router,
};
use entities::widget::{
    CreateWidgetRequest, Slice3Widget, UpdateWidgetRequest,
};

#[axum::debug_handler]
async fn list_widgets() -> Json<Vec<Slice3Widget>> {
    Json(vec![Slice3Widget {
        id: "1".to_string(),
        label: "Slice 3 Widget".to_string(),
        active: true,
    }])
}

#[axum::debug_handler]
async fn get_widget_by_id(
    Path(id): Path<String>,
) -> Result<Json<Slice3Widget>, StatusCode> {
    if id != "1" {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(Json(Slice3Widget {
        id: "1".to_string(),
        label: "Slice 3 Widget".to_string(),
        active: true,
    }))
}

#[axum::debug_handler]
async fn create_widget(
    Json(req): Json<CreateWidgetRequest>,
) -> (StatusCode, Json<Slice3Widget>) {
    let widget = Slice3Widget {
        id: "2".to_string(),
        label: req.label,
        active: req.active,
    };
    (StatusCode::CREATED, Json(widget))
}

#[axum::debug_handler]
async fn update_widget(
    Path(id): Path<String>,
    Json(req): Json<UpdateWidgetRequest>,
) -> Result<Json<Slice3Widget>, StatusCode> {
    if id != "1" {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(Json(Slice3Widget {
        id: "1".to_string(),
        label: req.label.unwrap_or_else(|| "Slice 3 Widget".to_string()),
        active: req.active.unwrap_or(true),
    }))
}

#[axum::debug_handler]
async fn delete_widget(Path(id): Path<String>) -> Result<StatusCode, StatusCode> {
    if id != "1" {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}

pub fn router() -> Router {
    Router::new()
        .route("/widget", get(list_widgets).post(create_widget))
        .route(
            "/widget/{id}",
            get(get_widget_by_id)
                .put(update_widget)
                .delete(delete_widget),
        )
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slice3Widget {
    pub id: String,
    pub label: String,
    pub active: bool,
}

// TODO: isso aqui ficou uma bosta, substituia por CQRS
// e aqui você vai fazer um domínio rico com validacoes.
// valide também a necessidade de value objects para as proprias das entidades.
#[derive(Debug, Deserialize)]
pub struct CreateWidgetRequest {
    pub label: String,
    pub active: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWidgetRequest {
    pub label: Option<String>,
    pub active: Option<bool>,
}

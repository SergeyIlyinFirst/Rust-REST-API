use axum::{response::IntoResponse, Json};

use crate::{
    infrastructure::data::repositories::todo_repository::TodoRepository,
    domain::models::todo::Todo,
    application::responses::result::Result
};

pub async fn get_all_todos_query() -> impl IntoResponse {
    let repository = TodoRepository::new();
    let mut todos: Vec<Todo> = Vec::new();

    if let Ok(result) = repository.get_all().await {
        todos = result;
    }

    let response = Result {
        result: Some(todos),
        error: None
    };

    Json(response)
}
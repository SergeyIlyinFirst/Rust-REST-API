use axum::{extract::Path, response::IntoResponse, Json};
use chrono::Local;

use crate::{
    application::responses::result::Result, 
    domain::models::todo::Todo, 
    infrastructure::data::repositories::todo_repository::TodoRepository
};

use super::requests::update_todo_request::UpdateTodoRequest;

pub async fn update_todo_command(
    Path(id): Path<String>,
    Json(body): Json<UpdateTodoRequest>
) -> impl IntoResponse {
    let repository = TodoRepository::new();

    match repository.get_by_id(&id).await {
        Ok(todo) => {
            let todo = Todo {
                id: todo.id,
                title: if !body.title.is_empty() {
                    body.title
                } else {
                    todo.title
                },
                content: if !body.content.is_empty() {
                    body.content
                } else {
                    todo.content
                },
                completed: body.completed,
                createdAt: todo.createdAt,
                updatedAt: Some(Local::now())
            };

            let todo_response = repository.update_todo(&id, todo).await.unwrap();

            let response = Result {
                result: Some(todo_response),
                error: None
            };

            Json(response)
        }
        Err(_) => {
            let response = Result {
                result: None,
                error: Some("Todo not found".to_string())
            };

            Json(response)
        }
    }
}
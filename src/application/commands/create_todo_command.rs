use axum::{Json, response::IntoResponse};
use chrono::Local;

use crate::{
    domain::models::todo::Todo, 
    infrastructure::data::repositories::todo_repository::TodoRepository,
    application::responses::result::Result
};

pub async fn create_todo_command(
    Json(mut body): Json<Todo>
) -> impl IntoResponse {
    let repository = TodoRepository::new();
    
    if let Ok(_todo) = repository.get_by_title(&body.title).await {
        let response = Result {
            result: None,
            error: Some("Todo exists".to_string())
        };

        return Json(response);
    }

    let datetime = Local::now();
    body.completed = Some(false);
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);

    let todo = repository.create_todo(body).await.unwrap()[0].to_owned();

    let response = Result {
        result: Some(todo),
        error: None
    };

    Json(response)
}
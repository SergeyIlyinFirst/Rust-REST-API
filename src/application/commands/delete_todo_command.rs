use axum::{extract::Path, response::IntoResponse, Json};

use crate::{
    application::responses::result::Result, 
    infrastructure::data::repositories::todo_repository::TodoRepository
};

pub async fn delete_todo_command(
    Path(id): Path<String>,
) -> impl IntoResponse {
    let repository = TodoRepository::new();
    let id = id.to_string();

    if let Ok(_) = repository.get_by_id(&id).await {
        let _ = repository.delete_todo(&id).await.unwrap();

        let response = Result {
            result: Some(id),
            error: None
        };

        return Json(response);
    }

    let response = Result {
        result: None,
        error: Some("Todo not found".to_string())
    };

    Json(response)
}
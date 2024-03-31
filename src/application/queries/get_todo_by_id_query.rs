use axum::{extract::Path, response::IntoResponse, Json};

use crate::{
    infrastructure::data::repositories::todo_repository::TodoRepository,
    application::responses::result::Result
};

pub async fn get_todo_by_id_query(
    Path(id): Path<String>,
) -> impl IntoResponse {
    let repository = TodoRepository::new();
    let id = id.to_string();

    if let Ok(_todo) = repository.get_by_id(&id).await {
        let response = Result {
            result: Some(_todo),
            error: None
        };

        return Json(response);
    }

    let response = Result {
        result: None,
        error: Some("Todo nof found".to_string())
    };

    Json(response)
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Result<T> {
    pub result: Option<T>,
    pub error: Option<String>
}
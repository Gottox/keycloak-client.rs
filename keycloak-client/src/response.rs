use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ResponseError {
    Error(String),
    ErrorMessage(String),
}
impl Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseError::Error(e) | ResponseError::ErrorMessage(e) => {
                write!(f, "{}", e)
            }
        }
    }
}

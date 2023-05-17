use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum DatabaseError {
    #[display(fmt = "Resource not found")]
    NotFound(String),
    // #[display(fmt = "Connection error")]
    // ConnectionError,
    // #[display(fmt = "Query error")]
    // QueryError,
    // #[display(fmt = "Insert error")]
    // InsertError,
    // #[display(fmt = "Update error")]
    // UpdateError,
    // #[display(fmt = "Delete error")]
    // DeleteError,
    // #[display(fmt = "Validation error")]
    // ValidationError,
    #[display(fmt = "Internal error")]
    InternalError,
}

impl ResponseError for DatabaseError {

    fn error_response(&self) -> HttpResponse {
        match self {
            DatabaseError::NotFound(id) => HttpResponse::NotFound().json(format!("Resource not found for id {}", id)),
            // DatabaseError::ConnectionError => HttpResponse::InternalServerError().finish(),
            // DatabaseError::QueryError => HttpResponse::InternalServerError().finish(),
            // DatabaseError::InsertError => HttpResponse::InternalServerError().finish(),
            // DatabaseError::UpdateError => HttpResponse::InternalServerError().finish(),
            // DatabaseError::DeleteError => HttpResponse::InternalServerError().finish(),
            // DatabaseError::ValidationError => HttpResponse::InternalServerError().finish(),
            DatabaseError::InternalError => HttpResponse::InternalServerError().json("Internal server error with database"),
        }
    }
}
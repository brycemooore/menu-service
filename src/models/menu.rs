use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Debug, Queryable)]
pub struct Menu {
    id: i32, 
    name: String, 
    is_active: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}
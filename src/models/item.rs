use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Debug, Queryable)]
pub struct Item {
    id: i32, 
    name: String, 
    description: String, 
    available: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}
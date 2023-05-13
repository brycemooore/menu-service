use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::item;

#[derive(Debug, Queryable)]
pub struct Item {
    id: i32, 
    name: String, 
    description: String, 
    available: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}

#[derive(Debug, Insertable)]
#[diesel(table_name = item)]
pub struct NewItem {
    name: String, 
    description: String, 
    available: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}
use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::menu;

#[derive(Debug, Queryable)]
pub struct Menu {
    id: i32, 
    name: String, 
    is_active: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}

#[derive(Debug, Insertable)]
#[diesel(table_name = menu)]
pub struct NewMenu {
    name: String, 
    is_active: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}
use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::models::menu::Menu;

#[derive(Debug, Queryable, Associations)]
#[diesel(belongs_to(Menu))]
pub struct MenuCategory {
    id: i32, 
    name: String, 
    menu_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}
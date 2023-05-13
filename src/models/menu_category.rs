use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::models::menu::Menu;
use crate::schema::menu_category;

#[derive(Debug, Queryable, Associations)]
#[diesel(belongs_to(Menu))]
#[diesel(table_name = menu_category)]
pub struct MenuCategory {
    id: i32, 
    name: String, 
    menu_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}

#[derive(Debug, Insertable)]
#[diesel(table_name = menu_category)]
pub struct NewMenuCategory {
    name: String, 
    menu_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}
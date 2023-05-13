use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::models::item::Item;
use crate::schema::item_ingredient;

#[derive(Debug, Queryable, Associations)]
#[diesel(belongs_to(Item))]
#[diesel(table_name = item_ingredient)]
pub struct ItemIngredient {
    id: i32, 
    name: String, 
    item_id: i32,
    available: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}

#[derive(Debug, Insertable)]
#[diesel(table_name = item_ingredient)]
pub struct NewItemIngredient {
    name: String, 
    item_id: i32,
    available: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}

use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::models::item::Item;

#[derive(Debug, Queryable, Associations)]
#[diesel(belongs_to(Item))]
pub struct ItemIngredient {
    id: i32, 
    name: String, 
    item_id: i32,
    available: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}

#[derive(Debug, Insertable)]
pub struct NewItemIngredient {
    name: String, 
    item_id: i32,
    available: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}

pub struct Ingredient {
    name: String, 
    item_id: i32,
}
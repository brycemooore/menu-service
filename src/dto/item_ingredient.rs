use chrono::NaiveDateTime;
use crate::{models::item_ingredient::ItemIngredient, vm::item_ingredient::ItemIngredientVM};

#[derive(Debug)]
pub struct ItemIngredientDTO {
    pub id: Option<i32>, 
    pub name: String, 
    pub item_id: Option<i32>,
    pub available: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

impl From<ItemIngredient> for ItemIngredientDTO {
    fn from(ingredient: ItemIngredient) -> Self {
        Self {
            id: Some(ingredient.id), 
            name: ingredient.name,
            available: ingredient.available,
            item_id: Some(ingredient.item_id),
            created_at: Some(ingredient.created_at),
            updated_at: Some(ingredient.updated_at),
        }
    }
}

impl From<ItemIngredientVM> for ItemIngredientDTO {
    fn from(ingredient: ItemIngredientVM) -> Self {
        Self {
            id: ingredient.id, 
            name: ingredient.name,
            available: ingredient.available,
            item_id: ingredient.item_id,
            created_at: ingredient.created_at,
            updated_at: ingredient.updated_at,
        }
    }
}

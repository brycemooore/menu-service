use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{dto::item_ingredient::ItemIngredientDTO, models::item_ingredient::ItemIngredient};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemIngredientVM {
    pub id: Option<i32>, 
    pub name: String, 
    pub item_id:Option<i32>,
    pub available: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

impl From<ItemIngredientDTO> for ItemIngredientVM {
    fn from(ingredient: ItemIngredientDTO) -> Self {
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


impl From<ItemIngredient> for ItemIngredientVM {
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

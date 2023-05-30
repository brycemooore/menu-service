use crate::{dto::item::ItemDTO, models::item::NewItem};
use super::item_ingredient::ItemIngredientVM;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemVM {
    pub id: Option<i32>, 
    pub name: String, 
    pub description: String, 
    pub available: bool,
    pub ingredients: Vec<ItemIngredientVM>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

impl From<ItemDTO> for ItemVM {
    fn from(item: ItemDTO) -> Self {
        Self {
            id: item.id, 
            name: item.name,
            description: item.description,
            available: item.available,
            created_at: item.created_at,
            updated_at: item.updated_at,
            ingredients: item.ingredients.into_iter().map(|i| i.into()).collect(),
        }
    }
}

impl From<NewItem> for ItemVM {
    fn from(item: NewItem) -> Self {
        Self {
            id: Some(item.id), 
            name: item.name,
            description: item.description,
            available: item.available,
            created_at: Some(item.created_at),
            updated_at: Some(item.updated_at),
            ingredients: Vec::new(),
        }
    }
}

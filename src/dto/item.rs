use chrono::NaiveDateTime;
use crate::{models::item::Item, vm::item::ItemVM};
use super::item_ingredient::ItemIngredientDTO;

#[derive(Debug)]
pub struct ItemDTO {
    pub id: Option<i32>, 
    pub name: String, 
    pub description: String, 
    pub available: bool,
    pub menu_category_id: i32,
    pub ingredients: Vec<ItemIngredientDTO>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}


impl From<Item> for ItemDTO {
    fn from(item: Item) -> Self {
        Self {
            id: Some(item.id), 
            name: item.name,
            description: item.description,
            available: item.available,
            menu_category_id: item.menu_category_id,
            created_at: Some(item.created_at),
            updated_at: Some(item.updated_at),
            ingredients: Vec::new(),
        }
    }
}

impl From<ItemVM> for ItemDTO {
    fn from(item: ItemVM) -> Self {
        Self {
            id: item.id, 
            name: item.name,
            description: item.description,
            available: item.available,
            menu_category_id: 0,
            created_at: item.created_at,
            updated_at: item.updated_at,
            ingredients: item.ingredients.into_iter().map(|i| i.into()).collect(),
        }
    }
}
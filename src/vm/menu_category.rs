use crate::models::menu_category::MenuCategory;
use super::item::ItemVM;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MenuCategoryVM {
    pub id: Option<i32>, 
    pub name: String, 
    pub menu_id: Option<i32>,
    pub items: Vec<ItemVM>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

impl From<MenuCategory> for MenuCategoryVM {
    fn from(cat: MenuCategory) -> Self {
        Self {
            id: Some(cat.id), 
            name: cat.name,
            menu_id: Some(cat.menu_id),
            created_at: Some(cat.created_at),
            updated_at: Some(cat.updated_at),
            items: Vec::new(),
        }
    }
}

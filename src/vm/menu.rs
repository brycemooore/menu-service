use crate::models::menu::Menu;
use super::menu_category::MenuCategoryVM;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuVM {
    pub id: Option<i32>, 
    pub name: String, 
    pub is_active: bool,
    pub categories: Vec<MenuCategoryVM>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

impl From<Menu> for MenuVM {
    fn from(menu: Menu) -> Self {
        Self {
            id: Some(menu.id), 
            name: menu.name,
            is_active: menu.is_active,
            created_at: Some(menu.created_at),
            updated_at: Some(menu.updated_at),
            categories: Vec::new(),
        }
    }
}

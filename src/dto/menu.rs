use chrono::NaiveDateTime;
use crate::vm::menu::MenuVM;

use super::menu_category::MenuCategoryDTO;

#[derive(Debug)]
pub struct MenuDTO {
    pub id: Option<i32>, 
    pub name: String, 
    pub is_active: bool,
    pub categories: Vec<MenuCategoryDTO>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

impl From<MenuVM> for MenuDTO {
    fn from(menu: MenuVM) -> Self {
        Self {
            id: menu.id, 
            name: menu.name,
            is_active: menu.is_active,
            created_at: menu.created_at,
            updated_at: menu.updated_at,
            categories: menu.categories.into_iter().map(|cat| cat.into()).collect(),
        }
    }
}
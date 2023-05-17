use chrono::NaiveDateTime;

use crate::vm::menu_category::MenuCategoryVM;

use super::item::ItemDTO;

#[derive(Debug)]
pub struct MenuCategoryDTO{
    pub id: Option<i32>, 
    pub name: String, 
    pub menu_id: Option<i32>,
    pub items: Vec<ItemDTO>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

impl From<MenuCategoryVM> for MenuCategoryDTO{
    fn from(cat: MenuCategoryVM) -> Self {
        Self {
            id: cat.id, 
            name: cat.name,
            menu_id: cat.menu_id,
            created_at: cat.created_at,
            updated_at: cat.updated_at,
            items: cat.items.into_iter().map(|item| item.into()).collect(),
        }
    }
}
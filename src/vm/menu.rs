use super::menu_category::MenuCategoryVM;
use chrono::NaiveDateTime;

pub struct MenuVM {
    id: i32, 
    name: String, 
    is_active: bool,
    categories: Vec<MenuCategoryVM>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}
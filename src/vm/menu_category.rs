use super::item::ItemVM;
use chrono::NaiveDateTime;

pub struct MenuCategoryVM {
    id: i32, 
    name: String, 
    menu_id: i32,
    items: Vec<ItemVM>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}
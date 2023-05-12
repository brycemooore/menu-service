use diesel::prelude::*;
use crate::models::{menu_category::MenuCategory, item::Item};

#[derive(Debug, Queryable, Associations)]
#[diesel(belongs_to(Item))]
#[diesel(belongs_to(MenuCategory))]
pub struct MenuCategoryItem {
    id: i32, 
    item_id: i32, 
    menu_category_id: i32
}
use diesel::prelude::*;
use crate::models::{menu_category::MenuCategory, item::Item};
use crate::schema::menu_category_item;

#[derive(Debug, Queryable, Associations)]
#[diesel(belongs_to(Item))]
#[diesel(belongs_to(MenuCategory))]
#[diesel(table_name = menu_category_item)]
pub struct MenuCategoryItem {
    id: i32, 
    item_id: i32, 
    menu_category_id: i32
}

#[derive(Debug, Insertable)]
#[diesel(table_name = menu_category_item)]
pub struct NewMenuCategoryItem {
    item_id: i32, 
    menu_category_id: i32
}
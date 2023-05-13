use super::item_ingredient::ItemIngredientVM;
use chrono::NaiveDateTime;

pub struct ItemVM {
    id: i32, 
    name: String, 
    description: String, 
    available: bool,
    ingredients: Vec<ItemIngredientVM>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}

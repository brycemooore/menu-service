use chrono::NaiveDateTime;

pub struct ItemIngredientVM {
    id: i32, 
    name: String, 
    item_id: i32,
    available: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}
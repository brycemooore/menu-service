use chrono::NaiveDateTime;


pub struct ItemIngredient {
    pub id: i32, 
    pub name: String, 
    pub item_id: i32,
    pub available: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}


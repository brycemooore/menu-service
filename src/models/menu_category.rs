use chrono::NaiveDateTime;

pub struct MenuCategory {
    pub id: i32, 
    pub name: String, 
    pub menu_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

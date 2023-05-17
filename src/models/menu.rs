use chrono::NaiveDateTime;

pub struct Menu {
    pub id: i32, 
    pub name: String, 
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}
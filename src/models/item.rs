use chrono::NaiveDateTime;


pub struct Item {
    pub id: i32, 
    pub name: String, 
    pub description: String, 
    pub available: bool,
    pub menu_category_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

pub struct NewItem {
    pub id: i32,
    pub name: String, 
    pub description: String, 
    pub available: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}


use chrono::{DateTime, NaiveDateTime, Utc};

pub struct Todo {
    pub id : Option<String>,
    pub title : String,
    pub content : String,
    pub completed: Option<bool>,
    pub created_at : Option<DateTime<Utc>>,
    pub updated_at : Option<DateTime<Utc>>,


}
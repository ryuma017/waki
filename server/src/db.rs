use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct PageRecord {
    pub id: i32,
    pub title: String,
    pub source: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

use chrono::Utc;

pub struct Entry {
    pub date: chrono::NaiveDate,
    pub content: String,
    pub mood: Option<String>,
    pub tags: Vec<String>,
    pub location: Option<String>,
    pub weather: Option<String>,
    pub last_edited: chrono::DateTime<chrono::Utc>,
}

impl Entry {
    pub fn new(content: String, tags: Vec<String>) -> Self {
        Entry { 
            date: Utc::now().date_naive(), 
            content, 
            mood: None, 
            tags,
            location: None, 
            weather: None, 
            last_edited: Utc::now(), 
        }
    }
}
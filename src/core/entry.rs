use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Entry {
    id: usize,
    pub date: chrono::NaiveDate,
    pub content: String,
    pub mood: Option<String>,
    pub tags: Vec<String>,
    pub location: Option<String>,
    pub weather: Option<String>,
    pub last_edited: chrono::DateTime<chrono::Utc>,
}

impl Entry {
    pub fn new(content: String, tags: Vec<String>, id: usize) -> Self {
        Entry {
            id,
            date: Utc::now().date_naive(),
            content,
            mood: None,
            tags,
            location: None,
            weather: None,
            last_edited: Utc::now(),
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(&self, input: &str) -> Result<(), serde_json::Error> {
        serde_json::from_str(input)
    }
}

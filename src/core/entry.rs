use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Entry {
    id: usize,
    #[serde(default)]
    pub date: chrono::NaiveDate,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mood: Option<String>,
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

    pub fn set_date(&mut self, date: chrono::NaiveDate) {
        self.date = date;
    }

    pub fn set_mood(&mut self, mood: &str) {
        self.mood = Some(mood.to_string());
    }

    pub fn set_location(&mut self, location: &str) {
        self.location = Some(location.to_string());
    }

    pub fn set_weather(&mut self, weather: &str) {
        self.weather = Some(weather.to_string());
    }
}

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub id: usize,
    pub date: NaiveDate,
    #[serde(default)]
    pub title: String,
    pub content: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mood: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weather: Option<String>,
    #[serde(default = "Utc::now")]
    pub last_edited: DateTime<Utc>,
}

impl Entry {
    pub fn builder() -> EntryBuilder {
        EntryBuilder::default()
    }

    pub fn update_last_edited(&mut self) {
        self.last_edited = Utc::now();
    }
}

#[derive(Default)]
pub struct EntryBuilder {
    id: Option<usize>,
    content: Option<String>,
    title: Option<String>,
    tags: Vec<String>,
    mood: Option<String>,
    location: Option<String>,
    weather: Option<String>,
}

impl EntryBuilder {
    pub fn id(mut self, id: usize) -> Self {
        self.id = Some(id);
        self
    }

    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags.extend(tags);
        self
    }

    pub fn mood(mut self, mood: Option<String>) -> Self {
        self.mood = mood;
        self
    }

    pub fn location(mut self, location: Option<String>) -> Self {
        self.location = location;
        self
    }

    pub fn weather(mut self, weather: Option<String>) -> Self {
        self.weather = weather;
        self
    }
    pub fn build(self) -> Result<Entry, &'static str> {
        let content = self.content.ok_or("Content is required")?;

        let written_date = Utc::now().date_naive();

        let title = self.title.unwrap_or_else(|| {
            let auto_title = String::from_iter(content.chars().take_while(|c| !c.is_ascii_punctuation()));
            // If the auto title is the same as the content, don't set it
            if auto_title == content { String::new() } else { auto_title }
        });

        Ok(Entry {
            id: 0, // This is set by the journal when adding the entry
            date: written_date,
            title,
            content,
            mood: self.mood,
            // TODO: add tag support
            tags: vec![],
            location: self.location,
            weather: self.weather,
            last_edited: Utc::now(),
        })
    }
}
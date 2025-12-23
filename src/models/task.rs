use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub completed: bool,
    pub archived: bool,
    pub description: String,
}

impl Task {
    pub fn new(title: impl Into<String>) -> Self {
        return Task {
            id: Uuid::new_v4(),
            title: title.into(),
            created_at: Utc::now(),
            completed: false,
            description: String::new(),
            archived: false,
        };
    }
}

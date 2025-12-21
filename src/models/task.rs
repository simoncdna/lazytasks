use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub completed: bool,
    pub description: String,
}

impl Task {
    pub fn new(id: usize, title: String, description: String) -> Self {
        return Task {
            id,
            title,
            created_at: Utc::now(),
            completed: false,
            description,
        };
    }
}

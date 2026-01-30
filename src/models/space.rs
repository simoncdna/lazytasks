use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub struct Space {
    pub id: Uuid,
    pub title: String,
    pub archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub archived_at: Option<DateTime<Utc>>,
}

impl Space {
    pub fn new(title: impl Into<String>) -> Self {
        Space {
            id: Uuid::new_v4(),
            title: title.into(),
            archived: false,
            created_at: Utc::now(),
            updated_at: None,
            archived_at: None,
        }
    }

    pub fn get_all() {}
}

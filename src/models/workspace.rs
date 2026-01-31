use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub struct Workspace {
    pub id: Uuid,
    pub title: String,
    pub archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub archived_at: Option<DateTime<Utc>>,
}

impl Workspace {
    pub fn new(title: impl Into<String>) -> Self {
        Workspace {
            id: Uuid::new_v4(),
            title: title.into(),
            archived: false,
            created_at: Utc::now(),
            updated_at: None,
            archived_at: None,
        }
    }
}

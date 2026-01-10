use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::Priority;

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub archived_at: Option<DateTime<Utc>>,
    pub completed: bool,
    pub archived: bool,
    pub description: String,
    pub priority: Option<Priority>,
}

impl Task {
    pub fn new(title: impl Into<String>) -> Self {
        return Task {
            id: Uuid::new_v4(),
            title: title.into(),
            created_at: Utc::now(),
            updated_at: None,
            archived_at: None,
            completed: false,
            archived: false,
            description: String::new(),
            priority: None,
        };
    }

    pub fn get_active_tasks(tasks: &[Task]) -> Vec<Task> {
        tasks.iter().filter(|task| !task.archived).cloned().collect()
    }

    pub fn get_archived_tasks(tasks: &[Task]) -> Vec<Task> {
        tasks.iter().filter(|task| task.archived).cloned().collect()
    }

    pub fn sort_by_priority(tasks: &mut Vec<Task>) {
        tasks.sort_by(|a, b| match (&a.priority, &b.priority) {
            (Some(pa), Some(pb)) => pb.cmp(pa),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.created_at.cmp(&b.created_at),
        });
    }

    pub fn sort_by_archived_date(tasks: &mut Vec<Task>) {
        tasks.sort_by(|a, b| b.archived_at.cmp(&a.archived_at));
    }
}

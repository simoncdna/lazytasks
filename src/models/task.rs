use chrono::{DateTime, Utc};
use rusqlite::Row;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::Priority;

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<Priority>,
    pub completed: bool,
    pub archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub archived_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(title: impl Into<String>) -> Self {
        return Task {
            id: Uuid::new_v4(),
            title: title.into(),
            description: None,
            priority: None,
            completed: false,
            archived: false,
            created_at: Utc::now(),
            updated_at: None,
            archived_at: None,
        };
    }

    pub fn get_active_tasks(tasks: &[Task]) -> Vec<Task> {
        tasks
            .iter()
            .filter(|task| !task.archived)
            .cloned()
            .collect()
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

    pub fn from_row(row: &Row) -> rusqlite::Result<Self> {
        let id: String = row.get("id")?;
        let priority: Option<String> = row.get("priority")?;
        let created_at: String = row.get("created_at")?;
        let updated_at: Option<String> = row.get("updated_at")?;
        let archived_at: Option<String> = row.get("archived_at")?;

        Ok(Task {
            id: Uuid::parse_str(&id).unwrap(),
            title: row.get("title")?,
            description: row.get("description")?,
            completed: row.get::<_, i32>("completed")? != 0,
            archived: row.get::<_, i32>("archived")? != 0,
            priority: priority.and_then(|p| Priority::from_str(&p)),
            created_at: DateTime::parse_from_rfc3339(&created_at)
                .unwrap()
                .with_timezone(&Utc),
            updated_at: updated_at.map(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .unwrap()
                    .with_timezone(&Utc)
            }),
            archived_at: archived_at.map(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .unwrap()
                    .with_timezone(&Utc)
            }),
        })
    }
}

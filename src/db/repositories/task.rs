use std::error::Error;

use chrono::{DateTime, Utc};
use rusqlite::{Connection, Row};
use uuid::Uuid;

use crate::models::{Priority, Task};

pub struct TaskRepository;

impl TaskRepository {
    pub fn create(connection: &Connection, task: &Task) -> Result<(), rusqlite::Error> {
        connection.execute(
            "INSERT INTO tasks (id, title, created_at) VALUES (?1, ?2, ?3) ",
            (
                task.id.to_string(),
                &task.title,
                task.created_at.to_rfc3339(),
            ),
        )?;

        Ok(())
    }

    pub fn update(connection: &Connection, task: &Task) -> Result<(), rusqlite::Error> {
        connection.execute(
            "UPDATE tasks SET 
                    title = ?2,
                    description = ?3,
                    completed = ?4,
                    archived = ?5,
                    priority = ?6,
                    updated_at = ?7,
                    archived_at = ?8
                WHERE id = ?1",
            (
                task.id.to_string(),
                &task.title,
                &task.description,
                task.completed as u32,
                task.archived as u32,
                task.priority.as_ref().map(|p| p.to_str()),
                task.updated_at.map(|d| d.to_rfc3339()),
                task.archived_at.map(|d| d.to_rfc3339()),
            ),
        )?;

        Ok(())
    }

    pub fn delete_many(connection: &Connection, ids: &[Uuid]) -> Result<(), rusqlite::Error> {
        let tx = connection.unchecked_transaction()?;

        for id in ids {
            tx.execute("DELETE FROM tasks WHERE id = ?1", [id.to_string()])?;
        }

        tx.commit()?;

        Ok(())
    }

    pub fn get_all(connection: &Connection) -> Result<Vec<Task>, Box<dyn Error>> {
        let mut stmt = connection.prepare("SELECT * FROM tasks")?;
        let mut rows = stmt.query([])?;

        let mut tasks: Vec<Task> = Vec::new();
        while let Some(row) = rows.next()? {
            tasks.push(Self::parse_row(row)?);
        }

        Ok(tasks)
    }

    fn parse_row(row: &Row) -> Result<Task, Box<dyn Error>> {
        let id: String = row.get("id")?;
        let priority: Option<String> = row.get("priority")?;
        let created_at: String = row.get("created_at")?;
        let updated_at: Option<String> = row.get("updated_at")?;
        let archived_at: Option<String> = row.get("archived_at")?;

        Ok(Task {
            id: Uuid::parse_str(&id)?,
            title: row.get("title")?,
            description: row.get("description")?,
            completed: row.get::<_, i32>("completed")? != 0,
            archived: row.get::<_, i32>("archived")? != 0,
            priority: priority.and_then(|p| Priority::from_str(&p)),
            created_at: DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
            updated_at: updated_at
                .map(|s| DateTime::parse_from_rfc3339(&s).map(|d| d.with_timezone(&Utc)))
                .transpose()?,
            archived_at: archived_at
                .map(|s| DateTime::parse_from_rfc3339(&s).map(|d| d.with_timezone(&Utc)))
                .transpose()?,
        })
    }
}

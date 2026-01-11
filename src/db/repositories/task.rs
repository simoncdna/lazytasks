use rusqlite::Connection;
use uuid::Uuid;

use crate::models::Task;

pub struct TaskRepository;

impl TaskRepository {
    pub fn create(connection: &Connection, task: &Task) {
        connection
            .execute(
                "INSERT INTO tasks (id, title, created_at) VALUES (?1, ?2, ?3) ",
                (
                    task.id.to_string(),
                    &task.title,
                    task.created_at.to_rfc3339(),
                ),
            )
            .expect("Failed to create task");
    }

    pub fn update(connection: &Connection, task: &Task) {
        connection
            .execute(
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
            )
            .expect("Failed to update task");
    }

    pub fn delete_many(connection: &Connection, ids: &[Uuid]) {
        let tx = connection.unchecked_transaction().unwrap();

        for id in ids {
            tx.execute("DELETE FROM tasks WHERE id = ?1", [id.to_string()])
                .expect("Failed to delete task");
        }

        tx.commit().unwrap();
    }

    pub fn get_all_tasks(connection: &Connection) -> Vec<Task> {
        let mut stmt = connection
            .prepare("SELECT * FROM tasks")
            .expect("Failed to prepare query");

        let tasks: Vec<Task> = stmt
            .query_map([], |row| Task::from_row(row))
            .expect("Failed to query tasks")
            .map(|r| r.expect("Failed to read task"))
            .collect();

        tasks
    }
}

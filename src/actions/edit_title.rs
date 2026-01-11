use chrono::Utc;
use uuid::Uuid;

use crate::{app::App, db::repositories::TaskRepository};

pub fn edit_title(app: &mut App, task_id: Uuid, title: String) {
    if !title.is_empty() {
        if let Some(task) = app.tasks.iter_mut().find(|task| task.id == task_id) {
            task.title = title;
            task.updated_at = Some(Utc::now());

            TaskRepository::update(&app.db.connection, task);
        }
    }
}

use chrono::Utc;
use uuid::Uuid;

use crate::{app::App, db::repositories::TaskRepository, models::Space};

pub fn move_task(app: &mut App, option_idx: Option<usize>, task_id: Uuid, spaces: &[Space]) {
    let idx = match option_idx {
        Some(i) => i,
        None => return,
    };

    let new_space_id = if idx >= spaces.len() {
        None
    } else {
        spaces.get(idx).map(|s| s.id)
    };

    let task = match app.tasks.iter_mut().find(|t| t.id == task_id) {
        Some(t) => t,
        None => return,
    };

    task.space_id = new_space_id;
    task.updated_at = Some(Utc::now());

    if let Err(e) = TaskRepository::update(&app.db.connection, task) {
        app.error = Some(e.to_string());
    }
}

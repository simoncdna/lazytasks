use chrono::Utc;
use uuid::Uuid;

use crate::{app::App, db::repositories::TaskRepository, models::Priority};

pub fn edit_priority(app: &mut App, option_idx: Option<usize>, task_ids: Vec<Uuid>) {
    let priority = match option_idx {
        Some(0) => Some(Priority::High),
        Some(1) => Some(Priority::Medium),
        Some(2) => Some(Priority::Low),
        Some(3) => None,
        _ => return,
    };

    for task in app.tasks.iter_mut() {
        if task_ids.contains(&task.id) {
            task.priority = priority.clone();
            task.updated_at = Some(Utc::now());

            if let Err(e) = TaskRepository::update(&app.db.connection, task) {
                app.error = Some(e.to_string());

                return;
            };
        }
    }

    app.selected_tasks.clear();
}

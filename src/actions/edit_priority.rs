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

    app.tasks.iter_mut().for_each(|task| {
        if task_ids.contains(&task.id) {
            task.priority = priority.clone();
            task.updated_at = Some(Utc::now());

            TaskRepository::update(&app.db.connection, task);
        }
    });

    app.selected_tasks.clear();
}

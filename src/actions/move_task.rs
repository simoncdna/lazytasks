use chrono::Utc;
use uuid::Uuid;

use crate::{app::App, db::repositories::TaskRepository, models::Workspace};

pub fn move_task(app: &mut App, option_idx: Option<usize>, task_id: Uuid, workspaces: &[Workspace]) {
    let idx = match option_idx {
        Some(i) => i,
        None => return,
    };

    let new_workspace_id = if idx >= workspaces.len() {
        None
    } else {
        workspaces.get(idx).map(|w| w.id)
    };

    let task = match app.tasks.iter_mut().find(|t| t.id == task_id) {
        Some(t) => t,
        None => return,
    };

    task.workspace_id = new_workspace_id;
    task.updated_at = Some(Utc::now());

    if let Err(e) = TaskRepository::update(&app.db.connection, task) {
        app.error = Some(e.to_string());
    }
}

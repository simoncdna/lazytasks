use chrono::Utc;
use uuid::Uuid;

use crate::{app::App, db::repositories::TaskRepository, state::PanelState};

pub fn toggle_task_completion(app: &mut App) {
    if !app.selected_tasks.is_empty() {
        for task in app.tasks.iter_mut() {
            if app.selected_tasks.contains(&task.id) {
                task.completed = !task.completed;
                task.updated_at = Some(Utc::now());

                if let Err(e) = TaskRepository::update(&app.db.connection, task) {
                    app.error = Some(e.to_string());
                    return;
                };
            }
        }
        app.selected_tasks.clear();
        return;
    }

    if app.state.active_panel == PanelState::ActiveTasks {
        let selected = app.state.workspaces_tree_state.selected();
        if selected.is_empty() {
            return;
        }

        let selected_id = selected.last().unwrap();
        if let Ok(uuid) = Uuid::parse_str(selected_id) {
            if let Some(task) = app.tasks.iter_mut().find(|t| t.id == uuid) {
                task.completed = !task.completed;
                task.updated_at = Some(Utc::now());

                if let Err(e) = TaskRepository::update(&app.db.connection, task) {
                    app.error = Some(e.to_string());
                };
            }
        }
    }
}

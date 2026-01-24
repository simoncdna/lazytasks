use chrono::Utc;

use crate::{app::App, db::repositories::TaskRepository};

pub fn toggle_task_completion(app: &mut App) {
    if app.selected_tasks.is_empty() {
        if let Some(task_index) = app
            .state
            .get_selected_panel_state()
            .and_then(|s| s.selected())
        {
            let task = app.get_current_tasks()[task_index].clone();
            if let Some(task) = app.tasks.iter_mut().find(|t| t.id == task.id) {
                task.completed = !task.completed;
                task.updated_at = Some(Utc::now());

                if let Err(e) = TaskRepository::update(&app.db.connection, task) {
                    app.error = Some(e.to_string());
                    return;
                };
            }
        }
    } else {
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
    }
}

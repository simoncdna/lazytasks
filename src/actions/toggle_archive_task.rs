use chrono::Utc;
use uuid::Uuid;

use crate::{app::App, db::repositories::TaskRepository};

pub fn toggle_archive_task(app: &mut App, option_idx: Option<usize>, task_ids: Vec<Uuid>) {
    if option_idx == Some(0) {
        for task in app.tasks.iter_mut() {
            if task_ids.contains(&task.id) {
                task.archived = !task.archived;
                task.archived_at = if task.archived {
                    Some(Utc::now())
                } else {
                    None
                };

                if let Err(e) = TaskRepository::update(&app.db.connection, task) {
                    app.error = Some(e.to_string());

                    return;
                };
            }
        }

        app.selected_tasks.clear();

        let count = app.get_current_tasks().len();
        if let Some(idx) = app
            .state
            .get_selected_panel_state()
            .and_then(|s| s.selected())
            && idx >= count
            && let Some(panel_state) = app.state.get_selected_panel_state()
        {
            panel_state.select(count.checked_sub(1));
        }
    }
}

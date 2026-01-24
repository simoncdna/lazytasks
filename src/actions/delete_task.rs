use uuid::Uuid;

use crate::{app::App, db::repositories::TaskRepository};

pub fn delete_task(app: &mut App, option_idx: Option<usize>, task_ids: Vec<Uuid>) {
    if option_idx == Some(0) {
        app.tasks.retain(|t| !task_ids.contains(&t.id));
        if let Err(e) = TaskRepository::delete_many(&app.db.connection, &task_ids) {
            app.error = Some(e.to_string());

            return;
        };

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

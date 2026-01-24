use chrono::Utc;
use ratatui::DefaultTerminal;

use crate::{app::App, db::repositories::TaskRepository, editor, state::PanelState};

pub fn edit_task(app: &mut App, terminal: &mut DefaultTerminal) {
    if app.state.active_panel == PanelState::ActiveTasks {
        let task_id = app
            .state
            .get_selected_panel_state()
            .and_then(|s| s.selected())
            .and_then(|idx| app.get_current_tasks().get(idx).map(|t| t.id));

        if let Some(task_id) = task_id
            && let Some(task_ref) = app.tasks.iter().find(|t| t.id == task_id)
        {
            let update = editor::open_in_editor(task_ref, terminal);

            // Only apply changes if title is not empty
            if !update.title.is_empty()
                && let Some(task) = app.tasks.iter_mut().find(|t| t.id == task_id)
            {
                task.title = update.title;
                task.description = Some(update.description);
                task.updated_at = Some(Utc::now());

                if let Err(e) = TaskRepository::update(&app.db.connection, task) {
                    app.error = Some(e.to_string());

                    return;
                };
            }
        }
    }
}

use chrono::Utc;
use ratatui::DefaultTerminal;
use uuid::Uuid;

use crate::{app::App, db::repositories::TaskRepository, editor, state::PanelState};

pub fn edit_task(app: &mut App, terminal: &mut DefaultTerminal) {
    if app.state.active_panel != PanelState::ActiveTasks {
        return;
    }

    let selected = app.state.workspaces_tree_state.selected();
    if selected.is_empty() {
        return;
    }

    let selected_id = selected.last().unwrap();
    let task_id = match Uuid::parse_str(selected_id) {
        Ok(uuid) => uuid,
        Err(_) => return,
    };

    let task_ref = match app.tasks.iter().find(|t| t.id == task_id) {
        Some(t) => t,
        None => return,
    };

    let update = editor::open_in_editor(task_ref, terminal);

    if !update.title.is_empty() {
        if let Some(task) = app.tasks.iter_mut().find(|t| t.id == task_id) {
            task.title = update.title;
            task.description = Some(update.description);
            task.updated_at = Some(Utc::now());

            if let Err(e) = TaskRepository::update(&app.db.connection, task) {
                app.error = Some(e.to_string());
            };
        }
    }
}

use uuid::Uuid;

use crate::{app::App, state::PanelState};

pub fn open_delete_modal(app: &mut App) {
    if !app.selected_tasks.is_empty() {
        app.state.open_delete_task(app.selected_tasks.clone());
        return;
    }

    if app.state.active_panel == PanelState::ActiveTasks {
        let selected = app.state.workspaces_tree_state.selected();
        if selected.is_empty() {
            return;
        }

        let selected_id = selected.last().unwrap();
        if let Ok(uuid) = Uuid::parse_str(selected_id) {
            if app.tasks.iter().any(|t| t.id == uuid) {
                app.state.open_delete_task(vec![uuid]);
            }
        }
    } else if let Some(task_index) = app.state.get_selected_panel_state().and_then(|s| s.selected())
    {
        let tasks = app.get_current_tasks();
        if task_index < tasks.len() {
            app.state.open_delete_task(vec![tasks[task_index].id]);
        }
    }
}

use uuid::Uuid;

use crate::{app::App, state::PanelState};

pub fn toggle_task_selection(app: &mut App) {
    if app.state.active_panel != PanelState::ActiveTasks {
        return;
    }

    let selected = app.state.workspaces_tree_state.selected();
    if selected.is_empty() {
        return;
    }

    let selected_id = selected.last().unwrap();
    if let Ok(uuid) = Uuid::parse_str(selected_id) {
        if app.tasks.iter().any(|t| t.id == uuid) {
            if app.selected_tasks.contains(&uuid) {
                app.selected_tasks.retain(|id| *id != uuid);
            } else {
                app.selected_tasks.push(uuid);
            }
        }
    }
}

use uuid::Uuid;

use crate::{app::App, state::PanelState};

pub fn open_priority_modal(app: &mut App) {
    if app.state.active_panel != PanelState::ActiveTasks {
        return;
    }

    if !app.selected_tasks.is_empty() {
        app.state.open_priority_task(app.selected_tasks.clone());
        return;
    }

    let selected = app.state.workspaces_tree_state.selected();
    if selected.is_empty() {
        return;
    }

    let selected_id = selected.last().unwrap();
    if let Ok(uuid) = Uuid::parse_str(selected_id) {
        if app.tasks.iter().any(|t| t.id == uuid) {
            app.state.open_priority_task(vec![uuid]);
        }
    }
}

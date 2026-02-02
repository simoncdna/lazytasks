use uuid::Uuid;

use crate::{app::App, state::PanelState};

pub fn open_move_task_modal(app: &mut App) {
    if app.state.active_panel != PanelState::ActiveTasks {
        return;
    }

    let selected = app.state.workspaces_tree_state.selected();
    if selected.is_empty() {
        return;
    }

    let selected_id = selected.last().unwrap();

    if let Ok(uuid) = Uuid::parse_str(selected_id) {
        if app.tasks.iter().any(|t| t.id == uuid && !t.archived) {
            app.state.open_move_task(uuid);
        }
    }
}

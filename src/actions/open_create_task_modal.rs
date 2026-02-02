use uuid::Uuid;

use crate::{app::App, state::PanelState};

pub fn open_create_task_modal(app: &mut App) {
    if app.state.active_panel != PanelState::ActiveTasks {
        return;
    }

    let selected = app.state.workspaces_tree_state.selected();

    let workspace_id = if selected.is_empty() {
        None
    } else {
        Uuid::parse_str(&selected[0]).ok().filter(|uuid| {
            app.workspaces.iter().any(|w| w.id == *uuid)
        })
    };

    app.state.open_create_task(workspace_id)
}

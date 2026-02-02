use uuid::Uuid;

use crate::{app::App, state::PanelState};

pub fn open_archive_workspace_modal(app: &mut App) {
    if app.state.active_panel != PanelState::ActiveTasks {
        return;
    }

    let selected = app.state.workspaces_tree_state.selected();
    if selected.len() != 1 {
        return;
    }

    let selected_id = &selected[0];

    if let Ok(uuid) = Uuid::parse_str(selected_id) {
        if let Some(workspace) = app.workspaces.iter().find(|s| s.id == uuid) {
            app.state.open_archive_workspace(uuid, workspace.archived);
        }
    }
}

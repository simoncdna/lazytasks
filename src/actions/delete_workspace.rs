use uuid::Uuid;

use crate::{app::App, db::repositories::WorkspaceRepository};

pub fn delete_workspace(app: &mut App, option_idx: Option<usize>, workspace_id: Uuid) {
    if option_idx != Some(0) {
        return;
    }

    if let Err(e) = WorkspaceRepository::delete(&app.db.connection, &workspace_id) {
        app.error = Some(e.to_string());
        return;
    }

    app.tasks.retain(|t| t.workspace_id != Some(workspace_id));
    app.workspaces.retain(|s| s.id != workspace_id);
    app.selected_tasks.clear();
    app.state.workspaces_tree_state.select_first();
}

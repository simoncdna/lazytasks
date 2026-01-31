use chrono::Utc;
use uuid::Uuid;

use crate::{
    app::App,
    db::repositories::{WorkspaceRepository, TaskRepository},
};

pub fn archive_workspace(app: &mut App, option_idx: Option<usize>, workspace_id: Uuid) {
    if option_idx != Some(0) {
        return;
    }

    let workspace = match app.workspaces.iter_mut().find(|s| s.id == workspace_id) {
        Some(s) => s,
        None => return,
    };

    workspace.archived = !workspace.archived;
    workspace.archived_at = if workspace.archived {
        Some(Utc::now())
    } else {
        None
    };
    workspace.updated_at = Some(Utc::now());

    if let Err(e) = WorkspaceRepository::update(&app.db.connection, workspace) {
        app.error = Some(e.to_string());
        return;
    }

    let is_archived = workspace.archived;

    for task in app.tasks.iter_mut().filter(|t| t.workspace_id == Some(workspace_id)) {
        task.archived = is_archived;
        task.archived_at = if is_archived { Some(Utc::now()) } else { None };
        task.updated_at = Some(Utc::now());

        if let Err(e) = TaskRepository::update(&app.db.connection, task) {
            app.error = Some(e.to_string());
            return;
        }
    }

    app.selected_tasks.clear();
    app.state.workspaces_tree_state.select_first();
}

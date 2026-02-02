use crate::{app::App, db::repositories::WorkspaceRepository, models};

pub fn create_workspace(app: &mut App, title: String) {
    let new_workspace = models::Workspace::new(title);
    let workspace_id = new_workspace.id.to_string();

    if let Err(e) = WorkspaceRepository::create(&app.db.connection, &new_workspace) {
        app.error = Some(e.to_string());
        return;
    };

    let is_first_workspace = app.workspaces.is_empty();
    app.workspaces.push(new_workspace);

    if is_first_workspace {
        app.state.workspaces_tree_state.select(vec![workspace_id]);
    }
}

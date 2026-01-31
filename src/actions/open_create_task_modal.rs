use crate::{app::App, state::PanelState};

pub fn open_create_task_modal(app: &mut App) {
    if app.state.active_panel == PanelState::ActiveTasks {
        let workspace_id = app.state.workspaces_tree_state.selected()[0].clone();
        app.state.open_create_task(workspace_id)
    }
}

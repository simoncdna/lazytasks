use uuid::Uuid;

use crate::{app::App, state::PanelState};

pub fn open_edit_title_modal(app: &mut App) {
    if app.state.active_panel == PanelState::ActiveTasks {
        let selected = app.state.workspaces_tree_state.selected();
        if selected.is_empty() {
            return;
        }

        let selected_id = selected.last().unwrap();
        if let Ok(uuid) = Uuid::parse_str(selected_id) {
            if let Some(task) = app.tasks.iter().find(|t| t.id == uuid) {
                app.state.open_edit_task(task.id, task.title.clone());
            }
        }
    }
}

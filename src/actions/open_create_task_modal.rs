use crate::{app::App, state::PanelState};

pub fn open_create_task_modal(app: &mut App) {
    if app.state.active_panel == PanelState::ActiveTasks {
        app.state.open_create_task()
    }
}

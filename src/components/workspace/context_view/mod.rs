pub mod about;
pub mod task_view;
pub mod workspace_view;

use ratatui::{Frame, layout::Rect};
use uuid::Uuid;

use crate::{app::App, components::workspace::context_view, state::PanelState};

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    match app.state.active_panel {
        PanelState::ActiveTasks => {
            let selected = app.state.workspaces_tree_state.selected();

            if selected.is_empty() {
                context_view::task_view::render_empty(frame, area);
                return;
            }

            let selected_id = selected.last().unwrap();
            if let Ok(uuid) = Uuid::parse_str(selected_id) {
                if let Some(workspace) = app.workspaces.iter().find(|w| w.id == uuid) {
                    context_view::workspace_view::render(frame, area, workspace, &app.tasks);
                    return;
                }

                if let Some(task) = app.tasks.iter().find(|t| t.id == uuid) {
                    context_view::task_view::render_task(frame, area, task);
                    return;
                }
            }

            context_view::task_view::render_empty(frame, area);
        }
        PanelState::ArchivedTasks => {
            let title = String::from(" Task details ");
            let tasks = app.archived_tasks();
            let current_list = &app.state.archived_tasks_state;
            context_view::task_view::render(frame, area, title, current_list, tasks);
        }
        PanelState::About => {
            let title = String::from(" About ");
            context_view::about::render(frame, area, title);
        }
    };
}

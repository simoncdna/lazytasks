use uuid::Uuid;

use crate::{app::App, state::PanelState};

pub fn toggle_task_selection(app: &mut App) {
    if app.state.active_panel != PanelState::ActiveTasks {
        return;
    }

    let selected = app.state.workspaces_tree_state.selected();
    if selected.is_empty() {
        return;
    }

    let selected_id = selected.last().unwrap();
    if let Ok(uuid) = Uuid::parse_str(selected_id) {
        if app.tasks.iter().any(|t| t.id == uuid) {
            if app.selected_tasks.contains(&uuid) {
                app.selected_tasks.retain(|id| *id != uuid);
            } else {
                app.selected_tasks.push(uuid);
            }
            return;
        }

        if app.workspaces.iter().any(|w| w.id == uuid) {
            let workspace_task_ids: Vec<Uuid> = app
                .tasks
                .iter()
                .filter(|t| t.workspace_id == Some(uuid) && !t.archived)
                .map(|t| t.id)
                .collect();

            let all_selected = workspace_task_ids
                .iter()
                .all(|id| app.selected_tasks.contains(id));

            if all_selected {
                app.selected_tasks
                    .retain(|id| !workspace_task_ids.contains(id));
            } else {
                for task_id in workspace_task_ids {
                    if !app.selected_tasks.contains(&task_id) {
                        app.selected_tasks.push(task_id);
                    }
                }
            }
        }
    }
}

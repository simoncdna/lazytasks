use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders},
};
use tui_tree_widget::{Tree, TreeItem};

use crate::{app::App, components::shared, state::PanelState};

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let is_active =
        app.state.active_modal.is_none() && app.state.active_panel == PanelState::ActiveTasks;
    let mut items: Vec<TreeItem<String>> = Vec::new();

    let border_color = if is_active {
        Color::Green
    } else {
        Color::White
    };

    for workspace in app.workspaces.iter().filter(|w| !w.archived) {
        let workspace_tasks: Vec<TreeItem<String>> = app
            .tasks
            .iter()
            .filter(|t| t.workspace_id == Some(workspace.id) && !t.archived)
            .map(|task| TreeItem::new_leaf(task.id.to_string(), task.title.clone()))
            .collect();

        let workspace_item = TreeItem::new(
            workspace.id.to_string(),
            format!("{} ({})", workspace.title.clone(), workspace_tasks.len()),
            workspace_tasks,
        )
        .unwrap();

        items.push(workspace_item);
    }

    for task in &app.tasks {
        if task.workspace_id.is_none() && !task.archived {
            items.push(TreeItem::new_leaf(task.id.to_string(), task.title.clone()));
        }
    }

    let tree = Tree::new(&items)
        .expect("identifiers are unique")
        .block(
            Block::new()
                .title(" Workspaces ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_color),
        )
        .highlight_style(if is_active {
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        })
        .node_closed_symbol("▶ ")
        .node_open_symbol("▼ ")
        .node_no_children_symbol("  ");

    frame.render_stateful_widget(tree, area, &mut app.state.workspaces_tree_state);
    shared::scrollbar::render(
        frame,
        area,
        app.workspaces.len(),
        app.state.active_tasks_state.offset(),
    );
}

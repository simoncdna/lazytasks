use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders},
};
use tui_tree_widget::{Tree, TreeItem};

use crate::{app::App, components::shared, models::Task, state::PanelState};

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
        let mut tasks_in_workspace: Vec<Task> = app
            .tasks
            .iter()
            .filter(|t| t.workspace_id == Some(workspace.id) && !t.archived)
            .cloned()
            .collect();
        Task::sort_by_priority(&mut tasks_in_workspace);

        let workspace_tasks: Vec<TreeItem<String>> = tasks_in_workspace
            .iter()
            .map(|task| {
                let line = match &task.priority {
                    Some(p) => Line::from(vec![
                        Span::styled(format!("{} ", p.label()), Style::default().fg(p.color())),
                        Span::raw(task.title.clone()),
                    ]),
                    None => Line::from(task.title.clone()),
                };
                TreeItem::new_leaf(task.id.to_string(), line)
            })
            .collect();

        let workspace_item = TreeItem::new(
            workspace.id.to_string(),
            format!("{} ({})", workspace.title.clone(), workspace_tasks.len()),
            workspace_tasks,
        )
        .unwrap();

        items.push(workspace_item);
    }

    let mut orphan_tasks: Vec<Task> = app
        .tasks
        .iter()
        .filter(|t| t.workspace_id.is_none() && !t.archived)
        .cloned()
        .collect();
    Task::sort_by_priority(&mut orphan_tasks);

    for task in &orphan_tasks {
        let line = match &task.priority {
            Some(p) => Line::from(vec![
                Span::styled(format!("{} ", p.label()), Style::default().fg(p.color())),
                Span::raw(task.title.clone()),
            ]),
            None => Line::from(task.title.clone()),
        };
        items.push(TreeItem::new_leaf(task.id.to_string(), line));
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

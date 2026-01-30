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

    for space in app.spaces.iter().filter(|s| !s.archived) {
        let space_tasks: Vec<TreeItem<String>> = app
            .tasks
            .iter()
            .filter(|t| t.space_id == Some(space.id) && !t.archived)
            .map(|task| TreeItem::new_leaf(task.id.to_string(), task.title.clone()))
            .collect();

        let space_item = TreeItem::new(
            space.id.to_string(),
            format!("{} ({})", space.title.clone(), space_tasks.len()),
            space_tasks,
        )
        .unwrap();

        items.push(space_item);
    }

    for task in &app.tasks {
        if task.space_id.is_none() && !task.archived {
            items.push(TreeItem::new_leaf(task.id.to_string(), task.title.clone()));
        }
    }

    let tree = Tree::new(&items)
        .expect("identifiers are unique")
        .block(
            Block::new()
                .title(" Spaces ")
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

    frame.render_stateful_widget(tree, area, &mut app.state.spaces_tree_state);
    shared::scrollbar::render(
        frame,
        area,
        app.spaces.len(),
        app.state.active_tasks_state.offset(),
    );
}

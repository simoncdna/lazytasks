use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::app::App;

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let task_title = " Tasks ";
    let list_items = app
        .tasks
        .iter()
        .map(|task| ListItem::new(task.title.clone()));
    let highlighted_style = if app.state.show_popup {
        Style::default()
    } else {
        Style::default()
            .bg(Color::Blue)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    };
    let border_color = if app.state.show_popup {
        Color::White
    } else {
        Color::Green
    };

    let tasks_view = List::new(list_items)
        .block(
            Block::new()
                .title(task_title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_color),
        )
        .highlight_style(highlighted_style);

    frame.render_stateful_widget(tasks_view, area, &mut app.state.tasks_list_state);
}

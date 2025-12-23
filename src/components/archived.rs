use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::{app::App, components::scrollbar, models::task::Task, state::PanelState};

pub fn render(frame: &mut Frame, area: Rect, app: &mut App, tasks: &[Task]) {
    let is_active = app.state.active_panel == PanelState::ArchivedTasks;
    let task_title = " Archived ";
    let list_items = tasks.iter().map(|task| {
        let span = if task.completed {
            Span::styled(
                task.title.clone(),
                Style::default().add_modifier(Modifier::CROSSED_OUT),
            )
        } else {
            Span::raw(task.title.clone())
        };
        ListItem::new(span)
    });
    let highlighted_style = if app.state.active_modal.is_some() || !is_active {
        Style::default()
    } else {
        Style::default()
            .bg(Color::Blue)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    };
    let border_color = if app.state.active_modal.is_some() || !is_active {
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

    frame.render_stateful_widget(tasks_view, area, &mut app.state.archived_tasks_list_state);
    scrollbar::render(
        frame,
        area,
        tasks.len(),
        app.state.archived_tasks_list_state.offset(),
    );
}

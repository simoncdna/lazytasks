use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::{app::App, components::shared, models::task::Task, state::PanelState};

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let tasks: Vec<&Task> = app.tasks.iter().filter(|t| t.archived).collect();
    let is_active = app.state.active_panel == PanelState::ArchivedTasks;
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

    let current_task_count = if tasks.len() > 0 {
        app.state.archived_tasks_state.selected().unwrap_or(0) + 1
    } else {
        0
    };
    let tasks_view = List::new(list_items)
        .block(
            Block::new()
                .title(" Archived ")
                .title_bottom(
                    Line::from(format!(" {} of {} ", current_task_count, tasks.len()))
                        .right_aligned(),
                )
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_color),
        )
        .highlight_style(highlighted_style);

    frame.render_stateful_widget(tasks_view, area, &mut app.state.archived_tasks_state);
    shared::scrollbar::render(
        frame,
        area,
        tasks.len(),
        app.state.archived_tasks_state.offset(),
    );
}

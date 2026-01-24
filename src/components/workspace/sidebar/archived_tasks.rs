use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::{app::App, components::shared, models::Task, state::PanelState};

pub fn render(frame: &mut Frame, area: Rect, app: &mut App, tasks: Vec<Task>) {
    let is_active =
        app.state.active_modal.is_none() && app.state.active_panel == PanelState::ArchivedTasks;
    let selected_index = app.state.archived_tasks_state.selected();

    let list_items: Vec<ListItem> = tasks
        .iter()
        .enumerate()
        .map(|(index, task)| {
            let is_cursor = selected_index == Some(index) && is_active;
            let is_selected = app.selected_tasks.contains(&task.id);

            let mut text_style = Style::default();

            if task.completed {
                text_style = text_style.add_modifier(Modifier::CROSSED_OUT);
            }

            if is_selected {
                text_style = text_style.fg(Color::LightGreen);
            } else if is_cursor {
                text_style = text_style.fg(Color::White).add_modifier(Modifier::BOLD);
            }

            let item_style = if is_cursor {
                Style::default()
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            ListItem::new(Line::from(vec![
                shared::priority_span::render(task.priority.as_ref()),
                Span::raw(" "),
                Span::styled(&task.title, text_style),
            ]))
            .style(item_style)
        })
        .collect();

    let border_color = if is_active {
        Color::Green
    } else {
        Color::White
    };

    let current_task_count = if !tasks.is_empty() {
        app.state.archived_tasks_state.selected().unwrap_or(0) + 1
    } else {
        0
    };
    let tasks_view = List::new(list_items).block(
        Block::new()
            .title(" Archived ")
            .title_bottom(
                Line::from(format!(" {} of {} ", current_task_count, tasks.len())).right_aligned(),
            )
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_color),
    );

    frame.render_stateful_widget(tasks_view, area, &mut app.state.archived_tasks_state);
    shared::scrollbar::render(
        frame,
        area,
        tasks.len(),
        app.state.archived_tasks_state.offset(),
    );
}

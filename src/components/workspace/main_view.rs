use chrono::Local;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, ListState, Paragraph, Wrap},
};

use crate::{app::App, models::task::Task, state::PanelState};

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let main_title = " Main View ";
    let (current_task, current_list): (Vec<&Task>, &ListState) = match app.state.active_panel {
        PanelState::ActiveTasks => (
            app.tasks.iter().filter(|t| !t.archived).collect(),
            &app.state.active_tasks_state,
        ),
        PanelState::ArchivedTasks => (
            app.tasks.iter().filter(|t| t.archived).collect(),
            &app.state.archived_tasks_state,
        ),
    };

    let dim_style = Style::default().fg(Color::DarkGray);

    let text = if let Some(selected_idx) = current_list.selected() {
        if let Some(task) = current_task.get(selected_idx) {
            let updated_at = task
                .updated_at
                .map(|d| d.with_timezone(&Local).format("%d/%m/%Y %H:%M").to_string())
                .unwrap_or_else(|| "-".to_string());

            let mut lines = vec![
                Line::from(Span::styled("---", dim_style)),
                Line::from(Span::styled(
                    format!("ID          : {}", task.id),
                    dim_style,
                )),
                Line::from(Span::styled(
                    format!(
                        "Created_at  : {}",
                        task.created_at.with_timezone(&Local).format("%d/%m/%Y %H:%M")
                    ),
                    dim_style,
                )),
                Line::from(Span::styled(
                    format!("Updated_at  : {}", updated_at),
                    dim_style,
                )),
                Line::from(Span::styled(
                    format!("Completed   : {}", task.completed),
                    dim_style,
                )),
                Line::from(Span::styled("---", dim_style)),
                Line::from(""),
                Line::from(Span::styled(
                    format!("# {}", task.title),
                    Style::default().add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
            ];

            for desc_line in task.description.lines() {
                lines.push(Line::from(format!("  {}", desc_line)));
            }

            lines
        } else {
            vec![Line::from("Task not found")]
        }
    } else {
        vec![Line::from(Span::styled("No task selected", dim_style))]
    };

    let main_view = Paragraph::new(text)
        .block(
            Block::new()
                .title(main_title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });
    frame.render_widget(main_view, area);
}

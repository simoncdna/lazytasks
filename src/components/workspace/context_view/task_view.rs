use chrono::Local;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, ListState, Paragraph, Wrap},
};

use crate::models::{Priority, Task};

pub fn render(
    frame: &mut Frame,
    area: Rect,
    title: String,
    current_list: &ListState,
    tasks: Vec<Task>,
) {
    let dim_style = Style::default().fg(Color::DarkGray);

    let text = if let Some(selected_idx) = current_list.selected() {
        if let Some(task) = tasks.get(selected_idx) {
            build_task_lines(task)
        } else {
            vec![Line::from("Task not found")]
        }
    } else {
        vec![Line::from(Span::styled("No task selected", dim_style))]
    };

    let main_view = Paragraph::new(text)
        .block(
            Block::new()
                .title(title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });
    frame.render_widget(main_view, area);
}

pub fn render_task(frame: &mut Frame, area: Rect, task: &Task) {
    let text = build_task_lines(task);

    let main_view = Paragraph::new(text)
        .block(
            Block::new()
                .title(" Task details ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });
    frame.render_widget(main_view, area);
}

pub fn render_empty(frame: &mut Frame, area: Rect) {
    let dim_style = Style::default().fg(Color::DarkGray);
    let text = vec![Line::from(Span::styled("No task selected", dim_style))];

    let main_view = Paragraph::new(text)
        .block(
            Block::new()
                .title(" Task details ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });
    frame.render_widget(main_view, area);
}

fn build_task_lines(task: &Task) -> Vec<Line<'static>> {
    let dim_style = Style::default().fg(Color::DarkGray);

    let updated_at = task
        .updated_at
        .map(|d| d.with_timezone(&Local).format("%d/%m/%Y %H:%M").to_string())
        .unwrap_or_else(|| "-".to_string());
    let archived_at = task
        .archived_at
        .map(|d| d.with_timezone(&Local).format("%d/%m/%Y %H:%M").to_string())
        .unwrap_or_else(|| "-".to_string());

    let mut lines = vec![
        Line::from(Span::styled("---", dim_style)),
        Line::from(Span::styled(format!("ID          : {}", task.id), dim_style)),
        Line::from(Span::styled(
            format!(
                "Created_at  : {}",
                task.created_at
                    .with_timezone(&Local)
                    .format("%d/%m/%Y %H:%M")
            ),
            dim_style,
        )),
        Line::from(Span::styled(format!("Updated_at  : {}", updated_at), dim_style)),
        Line::from(Span::styled(format!("Archived_at : {}", archived_at), dim_style)),
        Line::from(Span::styled(format!("Completed   : {}", task.completed), dim_style)),
        Line::from(Span::styled(
            format!(
                "Priority    : {}",
                match &task.priority {
                    Some(p) => Priority::label(p),
                    None => "x",
                }
            ),
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

    if let Some(description) = &task.description {
        for desc_line in description.lines() {
            lines.push(Line::from(format!("  {}", desc_line)));
        }
    }

    lines
}

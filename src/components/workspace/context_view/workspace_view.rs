use chrono::Local;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Cell, Row, Table},
};

use crate::models::{Task, Workspace};

pub fn render(frame: &mut Frame, area: Rect, workspace: &Workspace, tasks: &[Task]) {
    let mut workspace_tasks: Vec<Task> = tasks
        .iter()
        .filter(|t| t.workspace_id == Some(workspace.id) && !t.archived)
        .cloned()
        .collect();
    Task::sort_by_priority(&mut workspace_tasks);

    let header = Row::new(vec![
        Cell::from(Line::from("Priority").alignment(Alignment::Center)),
        Cell::from("Title"),
        Cell::from(Line::from("Created").alignment(Alignment::Center)),
        Cell::from(Line::from("Completed").alignment(Alignment::Center)),
    ])
    .style(Style::default().add_modifier(Modifier::BOLD))
    .height(1);

    let rows: Vec<Row> = workspace_tasks
        .iter()
        .map(|task| {
            let priority_cell = match &task.priority {
                Some(p) => Cell::from(Line::from(p.label()).alignment(Alignment::Center))
                    .style(Style::default().fg(p.color())),
                None => Cell::from(Line::from("-").alignment(Alignment::Center)),
            };

            let created_at = task
                .created_at
                .with_timezone(&Local)
                .format("%d/%m/%Y")
                .to_string();

            let completed_cell = if task.completed {
                Cell::from(Line::from("✓").alignment(Alignment::Center))
                    .style(Style::default().fg(Color::Green))
            } else {
                Cell::from(Line::from("✗").alignment(Alignment::Center))
                    .style(Style::default().fg(Color::DarkGray))
            };

            Row::new(vec![
                priority_cell,
                Cell::from(task.title.clone()),
                Cell::from(Line::from(created_at).alignment(Alignment::Center)),
                completed_cell,
            ])
        })
        .collect();

    let table = Table::new(rows, [
        Constraint::Length(8),
        Constraint::Fill(1),
        Constraint::Length(12),
        Constraint::Length(10),
    ])
        .header(header)
        .block(
            Block::new()
                .title(format!(" {} ({} tasks) ", workspace.title, workspace_tasks.len()))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );

    frame.render_widget(table, area);
}

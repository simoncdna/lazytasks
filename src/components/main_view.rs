use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

use crate::app::App;

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let main_title = " Main View ";

    let text = if let Some(selected_idx) = app.state.tasks_list_state.selected() {
        if let Some(task) = app.tasks.get(selected_idx) {
            vec![
                Line::from(format!("ID: {}", task.id)),
                Line::from(format!("TITLE: {}", task.title)),
                Line::from(format!("DESCRIPTION: {}", task.description)),
                Line::from(format!("COMPLETED: {}", task.completed)),
                Line::from(format!("CREATED_AT: {}", task.created_at)),
            ]
        } else {
            vec![Line::from("Task not found")]
        }
    } else {
        vec![Line::from("No task selected")]
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

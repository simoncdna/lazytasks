use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, BorderType, Borders},
};

pub fn render(frame: &mut Frame, area: Rect) {
    let task_title = " Tasks ";
    let tasks_view = Block::new()
        .title(task_title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    frame.render_widget(tasks_view, area);
}

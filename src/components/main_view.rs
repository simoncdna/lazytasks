use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, BorderType, Borders},
};

pub fn render(frame: &mut Frame, area: Rect) {
    let main_title = " Main View ";
    let main_view = Block::new()
        .title(main_title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    frame.render_widget(main_view, area);
}

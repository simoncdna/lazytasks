use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::Paragraph,
};

pub fn render(frame: &mut Frame, area: Rect, err: &str) {
    let bottom_bar =
        Paragraph::new(err).style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));

    frame.render_widget(bottom_bar, area);
}

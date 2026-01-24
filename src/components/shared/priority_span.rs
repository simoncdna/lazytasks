use ratatui::{style::Style, text::Span};

use crate::models::Priority;

pub fn render(priority: Option<&Priority>) -> Span<'static> {
    match priority {
        Some(p) => {
            let label = Priority::label(p);
            let color = Priority::color(p);
            Span::styled(label, Style::default().fg(color))
        }
        None => Span::from("  "),
    }
}

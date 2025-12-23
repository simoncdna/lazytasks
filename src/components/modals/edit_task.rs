use ratatui::{Frame, widgets::Paragraph};
use tui_input::Input;

use crate::components::shared;

pub fn render(frame: &mut Frame, input_state: &Input) {
    let area = shared::modal::Modal::new("Edit task title")
        .height(3)
        .render(frame);
    let width = area.width.saturating_sub(2) as usize;
    let scroll = input_state.visual_scroll(width);

    let input = Paragraph::new(input_state.value()).scroll((0, scroll as u16));

    frame.render_widget(input, area);

    frame.set_cursor_position((
        area.x + (input_state.visual_cursor().saturating_sub(scroll)) as u16,
        area.y,
    ))
}

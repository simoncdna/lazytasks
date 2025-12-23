use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState},
};

pub fn render(frame: &mut Frame, area: Rect, items_length: usize, position: usize) {
    let visible_height = area.height.saturating_sub(2) as usize;
    if items_length > visible_height {
        let max_scroll = items_length.saturating_sub(visible_height) + 1;

        let mut scrollbar_state = ScrollbarState::new(max_scroll)
            .viewport_content_length(10)
            .position(position);

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .track_symbol(None)
            .end_symbol(None)
            .thumb_symbol("▌");
        let scrollbar_area = area.inner(ratatui::layout::Margin {
            vertical: 1,
            horizontal: 0,
        });
        frame.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }
}

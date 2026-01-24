use ratatui::{
    Frame,
    style::{Color, Modifier, Style},
    widgets::{List, ListItem, ListState},
};

use crate::components::shared;

pub fn render(frame: &mut Frame, selected_option: &mut ListState) {
    let area = shared::modal::Modal::new("Edit Priority".to_string())
        .height(7)
        .render(frame);
    let list_items: Vec<ListItem> = vec![
        ListItem::from("P0 (High)"),
        ListItem::from("P1 (Medium)"),
        ListItem::from("P2 (Low)"),
        ListItem::from("x  (No priority)"),
        ListItem::from("Cancel"),
    ];
    let priority_options = List::new(list_items).highlight_style(
        Style::default()
            .bg(Color::Blue)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    );

    frame.render_stateful_widget(priority_options, area, selected_option);
}

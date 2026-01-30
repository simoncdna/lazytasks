use ratatui::{
    Frame,
    style::{Color, Modifier, Style},
    widgets::{List, ListItem, ListState},
};

use crate::{components::shared, models::Space};

pub fn render(frame: &mut Frame, selected_option: &mut ListState, spaces: &[Space]) {
    let height = (spaces.len() + 3).max(3).min(10) as u16;
    let area = shared::modal::Modal::new("Move to space")
        .height(height)
        .render(frame);

    let mut list_items: Vec<ListItem> = spaces
        .iter()
        .map(|s| ListItem::from(s.title.clone()))
        .collect();
    list_items.push(ListItem::from("No space"));

    let options = List::new(list_items).highlight_style(
        Style::default()
            .bg(Color::Blue)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    );

    frame.render_stateful_widget(options, area, selected_option);
}

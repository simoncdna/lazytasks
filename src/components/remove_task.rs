use ratatui::{
    Frame,
    style::{Color, Modifier, Style},
    widgets::{List, ListItem, ListState},
};

use crate::components::modal::Modal;

pub fn render(frame: &mut Frame, selected_option: &mut ListState) {
    let area = Modal::new("Delete confirmation").height(4).render(frame);
    let list_items: Vec<ListItem> = vec![ListItem::from("Delete task"), ListItem::from("Cancel")];
    let delete_options = List::new(list_items).highlight_style(
        Style::default()
            .bg(Color::Blue)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    );

    frame.render_stateful_widget(delete_options, area, selected_option);
}

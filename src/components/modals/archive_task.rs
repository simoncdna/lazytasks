use ratatui::{
    Frame,
    style::{Color, Modifier, Style},
    widgets::{List, ListItem, ListState},
};

use crate::components::shared;

pub fn render(frame: &mut Frame, selected_option: &mut ListState, is_archived: bool) {
    let action_name = if is_archived { "Unarchive" } else { "Archive" };
    let area = shared::modal::Modal::new(format!("{} confirmation", action_name))
        .height(4)
        .render(frame);
    let list_items: Vec<ListItem> = vec![ListItem::from(action_name), ListItem::from("Cancel")];
    let delete_options = List::new(list_items).highlight_style(
        Style::default()
            .bg(Color::Blue)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    );

    frame.render_stateful_widget(delete_options, area, selected_option);
}

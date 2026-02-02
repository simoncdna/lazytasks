use ratatui::{
    Frame,
    style::{Color, Modifier, Style},
    widgets::{List, ListItem, ListState},
};

use crate::{components::shared, models::Workspace};

pub fn render(frame: &mut Frame, selected_option: &mut ListState, workspaces: &[Workspace]) {
    let height = (workspaces.len() + 3).max(3).min(10) as u16;
    let area = shared::modal::Modal::new("Move to workspace")
        .height(height)
        .render(frame);

    let mut list_items: Vec<ListItem> = workspaces
        .iter()
        .map(|w| ListItem::from(w.title.clone()))
        .collect();
    list_items.push(ListItem::from("No workspace"));

    let options = List::new(list_items).highlight_style(
        Style::default()
            .bg(Color::Blue)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    );

    frame.render_stateful_widget(options, area, selected_option);
}

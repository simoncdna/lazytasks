pub mod app_info;
pub mod error_hints;
pub mod key_hints;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::{app::App, components::bottom_bar};

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let bottom_bar_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(area);

    match &app.error {
        Some(err) => bottom_bar::error_hints::render(frame, bottom_bar_layout[0], &err),
        None => bottom_bar::key_hints::render(frame, bottom_bar_layout[0], app),
    }

    bottom_bar::app_info::render(frame, bottom_bar_layout[1]);
}

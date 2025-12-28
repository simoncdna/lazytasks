pub mod main_view;
pub mod sidebar;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::{app::App, components::workspace};

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let workspace = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    workspace::sidebar::render(frame, workspace[0], app);
    workspace::main_view::render(frame, workspace[1], app);
}

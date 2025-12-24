pub mod active_tasks;
pub mod archived_tasks;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::{app::App, components::sidebar, models::task::Task};

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let sidebar = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

    let active_tasks: Vec<Task> = app
        .tasks
        .iter()
        .filter(|task| !task.archived)
        .cloned()
        .collect();

    let archived_tasks: Vec<Task> = app
        .tasks
        .iter()
        .filter(|task| task.archived)
        .cloned()
        .collect();

    sidebar::active_tasks::render(frame, sidebar[0], app, &active_tasks);
    sidebar::archived_tasks::render(frame, sidebar[1], app, &archived_tasks);
}

use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::{Block, BorderType, Borders, ListState, Paragraph, Wrap},
};

use crate::{app::App, models::task::Task, state::PanelState};

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let main_title = " Main View ";
    let (current_task, current_list): (Vec<&Task>, &ListState) = match app.state.active_panel {
        PanelState::ActiveTasks => (
            app.tasks.iter().filter(|t| !t.archived).collect(),
            &app.state.active_tasks_state,
        ),
        PanelState::ArchivedTasks => (
            app.tasks.iter().filter(|t| t.archived).collect(),
            &app.state.archived_tasks_state,
        ),
    };

    let text = if let Some(selected_idx) = current_list.selected() {
        if let Some(task) = current_task.get(selected_idx) {
            vec![
                Line::from(format!("ID: {}", task.id)),
                Line::from(format!("Title: {}", task.title)),
                Line::from(format!("Description: {}", task.description)),
                Line::from(format!("Completed: {}", task.completed)),
                Line::from(format!("Archived: {}", task.archived)),
                Line::from(format!(
                    "Created_at: {}",
                    task.created_at.format("%Y-%m-%d %H:%M:%S")
                )),
                Line::from(format!(
                    "Updated_at: {}",
                    match task.updated_at {
                        Some(value) => value.format("%Y-%m-%d %H:%M:%S").to_string(),
                        None => "Not updated".to_string(),
                    }
                )),
            ]
        } else {
            vec![Line::from("Task not found")]
        }
    } else {
        vec![Line::from("No task selected")]
    };

    let main_view = Paragraph::new(text)
        .block(
            Block::new()
                .title(main_title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });
    frame.render_widget(main_view, area);
}

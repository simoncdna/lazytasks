use ratatui::crossterm::{self, event::Event};
use tui_input::backend::crossterm::EventHandler;

use crate::{app::App, models, state::ModalState};

pub fn handle_key_event(app: &mut App, event: &Event) {
    if let crossterm::event::Event::Key(key) = event {
        match &mut app.state.active_modal {
            Some(ModalState::CreateTask { input }) => match key.code {
                crossterm::event::KeyCode::Esc => {
                    app.state.close_modal();
                }
                crossterm::event::KeyCode::Enter => {
                    let new_task = models::task::Task::new(
                        app.tasks.len(),
                        input.value().to_string(),
                        "".to_string(),
                    );
                    app.tasks.push(new_task);
                    app.state.close_modal();
                    app.state.tasks_list_state.select(Some(0));
                }
                _ => {
                    input.handle_event(&event);
                }
            },
            Some(ModalState::DeleteTask {
                index,
                selected_option,
            }) => match key.code {
                crossterm::event::KeyCode::Esc => {
                    app.state.close_modal();
                }
                crossterm::event::KeyCode::Enter => {
                    let current_option_index = selected_option.selected();

                    if current_option_index == Some(0) {
                        app.tasks.remove(*index);
                        app.state.close_modal();
                    } else {
                        app.state.close_modal();
                    }
                }
                crossterm::event::KeyCode::Char('j') => {
                    selected_option.select_next();
                }
                crossterm::event::KeyCode::Char('k') => {
                    selected_option.select_previous();
                }
                _ => {}
            },
            None => match key.code {
                crossterm::event::KeyCode::Char('c') => app.state.open_create_task(),
                crossterm::event::KeyCode::Char('q') => app.exit = true,
                crossterm::event::KeyCode::Char('d') => {
                    app.state.open_delete_task();
                }
                crossterm::event::KeyCode::Char('j') => {
                    app.state.select_next_task(app.tasks.len());
                }
                crossterm::event::KeyCode::Char('k') => {
                    app.state.select_previous_task();
                }
                _ => {}
            },
        }
    }
}

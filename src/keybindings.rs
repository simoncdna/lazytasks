use ratatui::crossterm::{self, event::Event};

use ratatui::DefaultTerminal;
use tui_input::backend::crossterm::EventHandler;

use crate::actions;
use crate::{app::App, state::ModalState};

pub fn handle_key_event(app: &mut App, event: &Event, terminal: &mut DefaultTerminal) {
    if let crossterm::event::Event::Key(key) = event {
        match &mut app.state.active_modal {
            Some(ModalState::CreateTask { input }) => match key.code {
                crossterm::event::KeyCode::Esc => actions::close_modal(app),
                crossterm::event::KeyCode::Enter => {
                    let title = input.value().trim().to_owned();

                    if !title.is_empty() {
                        actions::create_task(app, title);
                    }
                    actions::close_modal(app);
                }
                _ => {
                    input.handle_event(event);
                }
            },
            Some(ModalState::EditTask { task_id, input }) => match key.code {
                crossterm::event::KeyCode::Esc => actions::close_modal(app),
                crossterm::event::KeyCode::Enter => {
                    let title = input.value().trim().to_owned();
                    let task_id = *task_id;

                    actions::edit_title(app, task_id, title);
                    actions::close_modal(app);
                }
                _ => {
                    input.handle_event(event);
                }
            },
            Some(ModalState::ArchivedTask {
                task_ids,
                selected_option,
                is_archived: _,
            }) => match key.code {
                crossterm::event::KeyCode::Esc => actions::close_modal(app),
                crossterm::event::KeyCode::Enter => {
                    let option_idx = selected_option.selected();
                    let task_ids = task_ids.clone();

                    actions::toggle_archive_task(app, option_idx, task_ids);
                    actions::close_modal(app);
                }
                crossterm::event::KeyCode::Char('j') => {
                    selected_option.select_next();
                }
                crossterm::event::KeyCode::Char('k') => {
                    selected_option.select_previous();
                }
                _ => {}
            },
            Some(ModalState::DeleteTask {
                task_ids,
                selected_option,
            }) => match key.code {
                crossterm::event::KeyCode::Esc => actions::close_modal(app),
                crossterm::event::KeyCode::Enter => {
                    let option_idx = selected_option.selected();
                    let task_ids = task_ids.clone();

                    actions::delete_task(app, option_idx, task_ids);
                    actions::close_modal(app);
                }
                crossterm::event::KeyCode::Char('j') => {
                    selected_option.select_next();
                }
                crossterm::event::KeyCode::Char('k') => {
                    selected_option.select_previous();
                }
                _ => {}
            },
            Some(ModalState::PriorityTask {
                task_ids,
                selected_option,
            }) => match key.code {
                crossterm::event::KeyCode::Esc => actions::close_modal(app),
                crossterm::event::KeyCode::Enter => {
                    let option_idx = selected_option.selected();
                    let task_ids = task_ids.clone();

                    actions::edit_priority(app, option_idx, task_ids);
                    actions::close_modal(app);
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
                crossterm::event::KeyCode::Char('a') => actions::open_archive_modal(app),
                crossterm::event::KeyCode::Char('c') => actions::open_create_modal(app),
                crossterm::event::KeyCode::Char('e') => actions::open_edit_title_modal(app),
                crossterm::event::KeyCode::Char('p') => actions::open_priority_modal(app),
                crossterm::event::KeyCode::Char('E') => actions::edit_task(app, terminal),
                crossterm::event::KeyCode::Char('y') => actions::toggle_task_completion(app),
                crossterm::event::KeyCode::Char('q') => actions::quit(app),
                crossterm::event::KeyCode::Char('d') => actions::open_delete_modal(app),
                crossterm::event::KeyCode::Char('j') => actions::select_next_task(app),
                crossterm::event::KeyCode::Char('k') => actions::select_previous_task(app),
                crossterm::event::KeyCode::Char(' ') => actions::toggle_task_selection(app),
                crossterm::event::KeyCode::Tab => actions::switch_panel(app),
                crossterm::event::KeyCode::Enter => actions::clean_err_msg(app),
                _ => {}
            },
        }
    }
}

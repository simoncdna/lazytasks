use ratatui::crossterm::{self, event::Event};
use tui_input::backend::crossterm::EventHandler;

use crate::{
    app::App,
    models,
    state::{ModalState, PanelState},
};

pub fn handle_key_event(app: &mut App, event: &Event) {
    if let crossterm::event::Event::Key(key) = event {
        match &mut app.state.active_modal {
            Some(ModalState::CreateTask { input }) => match key.code {
                crossterm::event::KeyCode::Esc => {
                    app.state.close_modal();
                }
                crossterm::event::KeyCode::Enter => {
                    let title = input.value().trim();
                    if !title.is_empty() {
                        let new_task = models::task::Task::new(input.value());
                        app.tasks.push(new_task);
                        app.storage.save(&app.tasks);
                    }

                    let new_index = app.active_tasks().len() - 1;

                    app.state.active_tasks_state.select(Some(new_index));
                    app.state.close_modal();
                }
                _ => {
                    input.handle_event(&event);
                }
            },
            Some(ModalState::EditTask { task_id, input }) => match key.code {
                crossterm::event::KeyCode::Esc => {
                    app.state.close_modal();
                }
                crossterm::event::KeyCode::Enter => {
                    let new_title = input.value().trim();
                    if !new_title.is_empty() {
                        if let Some(task) = app.tasks.iter_mut().find(|task| task.id == *task_id) {
                            task.title = new_title.to_string();
                        }
                        app.storage.save(&app.tasks);
                    }
                    app.state.close_modal();
                }
                _ => {
                    input.handle_event(&event);
                }
            },
            Some(ModalState::ArchivedTask {
                task_id,
                selected_option,
            }) => match key.code {
                crossterm::event::KeyCode::Esc => {
                    app.state.close_modal();
                }
                crossterm::event::KeyCode::Enter => {
                    let current_option_index = selected_option.selected();

                    if current_option_index == Some(0) {
                        if let Some(task) = app.tasks.iter_mut().find(|task| task.id == *task_id) {
                            task.archived = true;
                        }
                        app.storage.save(&app.tasks);
                        app.state.archived_tasks_state.select(Some(0));
                    }

                    app.state.close_modal();
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
                task_id,
                selected_option,
            }) => match key.code {
                crossterm::event::KeyCode::Esc => {
                    app.state.close_modal();
                }
                crossterm::event::KeyCode::Enter => {
                    let current_option_index = selected_option.selected();

                    if current_option_index == Some(0) {
                        app.tasks.retain(|t| t.id != *task_id);
                        app.storage.save(&app.tasks);
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
                crossterm::event::KeyCode::Char('a') => {
                    if let Some(task_index) = app.state.get_selected_panel_state().selected() {
                        let task = &app.selected_tasks()[task_index];
                        app.state.open_archived_task(task.id)
                    }
                }
                crossterm::event::KeyCode::Char('c') => app.state.open_create_task(),
                crossterm::event::KeyCode::Char('e') => {
                    if let Some(task_index) = app.state.get_selected_panel_state().selected() {
                        let task = &app.selected_tasks()[task_index];
                        app.state.open_edit_task(task.id, task.title.clone());
                    }
                }
                crossterm::event::KeyCode::Char('y') => {
                    if let Some(task_index) = app.state.get_selected_panel_state().selected() {
                        let task = app.selected_tasks()[task_index].clone();
                        if let Some(task) = app.tasks.iter_mut().find(|t| t.id == task.id) {
                            task.completed = !task.completed;
                        }
                    }
                    app.storage.save(&app.tasks);
                }
                crossterm::event::KeyCode::Char('q') => app.exit = true,
                crossterm::event::KeyCode::Char('d') => {
                    if let Some(task_index) = app.state.get_selected_panel_state().selected() {
                        let task = &app.selected_tasks()[task_index];
                        app.state.open_delete_task(task.id);
                    }
                }
                crossterm::event::KeyCode::Char('j') => match app.state.active_panel {
                    PanelState::ActiveTasks => app.state.select_next_task(app.active_tasks().len()),
                    PanelState::ArchivedTasks => {
                        app.state.select_next_task(app.archived_tasks().len())
                    }
                },
                crossterm::event::KeyCode::Char('k') => match app.state.active_panel {
                    PanelState::ActiveTasks => {
                        app.state.select_previous_task(app.active_tasks().len())
                    }
                    PanelState::ArchivedTasks => {
                        app.state.select_previous_task(app.archived_tasks().len())
                    }
                },
                crossterm::event::KeyCode::Tab => {
                    app.state.toggle_active_panel();
                }
                _ => {}
            },
        }
    }
}

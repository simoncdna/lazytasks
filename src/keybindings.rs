use chrono::Utc;
use ratatui::crossterm::{self, event::Event};

use ratatui::DefaultTerminal;
use tui_input::backend::crossterm::EventHandler;

use crate::{
    app::App,
    editor, models,
    state::{ModalState, PanelState},
};

pub fn handle_key_event(app: &mut App, event: &Event, terminal: &mut DefaultTerminal) {
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
                            task.updated_at = Some(Utc::now());
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
                task_ids,
                selected_option,
                is_archived: _,
            }) => match key.code {
                crossterm::event::KeyCode::Esc => {
                    app.state.close_modal();
                }
                crossterm::event::KeyCode::Enter => {
                    let current_option_index = selected_option.selected();

                    if current_option_index == Some(0) {
                        app.tasks.iter_mut().for_each(|task| {
                            if task_ids.contains(&task.id) {
                                task.archived = !task.archived;
                                task.updated_at = Some(Utc::now());
                            }
                        });
                        app.storage.save(&app.tasks);
                        app.selected_tasks.clear();

                        let count = app.get_current_tasks().len();
                        if let Some(idx) = app.state.get_selected_panel_state().selected() {
                            if idx >= count {
                                app.state
                                    .get_selected_panel_state()
                                    .select(count.checked_sub(1));
                            }
                        }
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
                task_ids,
                selected_option,
            }) => match key.code {
                crossterm::event::KeyCode::Esc => {
                    app.state.close_modal();
                }
                crossterm::event::KeyCode::Enter => {
                    let current_option_index = selected_option.selected();

                    if current_option_index == Some(0) {
                        app.tasks.retain(|t| !task_ids.contains(&t.id));
                        app.selected_tasks.clear();
                        app.storage.save(&app.tasks);

                        let count = app.get_current_tasks().len();
                        if let Some(idx) = app.state.get_selected_panel_state().selected() {
                            if idx >= count {
                                app.state
                                    .get_selected_panel_state()
                                    .select(count.checked_sub(1));
                            }
                        }
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
            None => match key.code {
                crossterm::event::KeyCode::Char('a') => {
                    let is_archived = app.state.active_panel == PanelState::ArchivedTasks;

                    if app.selected_tasks.is_empty() {
                        if let Some(task_index) = app.state.get_selected_panel_state().selected() {
                            let task_id = app.get_current_tasks()[task_index].id;
                            app.state.open_archived_task(vec![task_id], is_archived)
                        }
                    } else {
                        app.state
                            .open_archived_task(app.selected_tasks.clone(), is_archived);
                    }
                }
                crossterm::event::KeyCode::Char('c') => {
                    if app.state.active_panel == PanelState::ActiveTasks {
                        app.state.open_create_task()
                    }
                }
                crossterm::event::KeyCode::Char('e') => {
                    if let Some(task_index) = app.state.get_selected_panel_state().selected()
                        && app.state.active_panel == PanelState::ActiveTasks
                    {
                        let task = &app.get_current_tasks()[task_index];
                        app.state.open_edit_task(task.id, task.title.clone());
                    }
                }
                crossterm::event::KeyCode::Char('E') => {
                    if app.state.active_panel == PanelState::ActiveTasks {
                        let task_id = app
                            .state
                            .get_selected_panel_state()
                            .selected()
                            .and_then(|idx| app.get_current_tasks().get(idx).map(|t| t.id));

                        if let Some(task_id) = task_id {
                            if let Some(task_ref) = app.tasks.iter().find(|t| t.id == task_id) {
                                let update = editor::open_in_editor(task_ref, terminal);

                                // Only apply changes if title is not empty
                                if !update.title.is_empty() {
                                    if let Some(task) =
                                        app.tasks.iter_mut().find(|t| t.id == task_id)
                                    {
                                        task.title = update.title;
                                        task.description = update.description;
                                        task.updated_at = Some(Utc::now());
                                    }
                                    app.storage.save(&app.tasks);
                                }
                            }
                        }
                    }
                }
                crossterm::event::KeyCode::Char('y') => {
                    if app.selected_tasks.is_empty() {
                        if let Some(task_index) = app.state.get_selected_panel_state().selected() {
                            let task = app.get_current_tasks()[task_index].clone();
                            if let Some(task) = app.tasks.iter_mut().find(|t| t.id == task.id) {
                                task.completed = !task.completed;
                                task.updated_at = Some(Utc::now());
                            }
                        }
                    } else {
                        app.tasks.iter_mut().for_each(|t| {
                            if app.selected_tasks.contains(&t.id) {
                                t.completed = !t.completed;
                                t.updated_at = Some(Utc::now());
                            }
                        });
                        app.selected_tasks.clear();
                    }
                    app.storage.save(&app.tasks);
                }
                crossterm::event::KeyCode::Char('q') => app.exit = true,
                crossterm::event::KeyCode::Char('d') => {
                    if app.selected_tasks.is_empty() {
                        if let Some(task_index) = app.state.get_selected_panel_state().selected() {
                            let task_id = app.get_current_tasks()[task_index].id;
                            app.state.open_delete_task(vec![task_id]);
                        }
                    } else {
                        app.state.open_delete_task(app.selected_tasks.clone());
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
                crossterm::event::KeyCode::Char(' ') => {
                    if let Some(task_index) = app.state.get_selected_panel_state().selected() {
                        let task_id = app.get_current_tasks()[task_index].id;
                        if app.selected_tasks.contains(&task_id) {
                            app.selected_tasks.retain(|id| *id != task_id);
                        } else {
                            app.selected_tasks.push(task_id);
                        }
                    }
                }
                crossterm::event::KeyCode::Tab => {
                    app.selected_tasks.clear();
                    app.state.toggle_active_panel();
                }
                _ => {}
            },
        }
    }
}

use std::io::Result;

use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
};
use uuid::Uuid;

use crate::{
    components,
    db::{Db, repositories::TaskRepository},
    keybindings::handle_key_event,
    models::Task,
    state,
};
use crate::{models, state::ModalState};

pub struct App {
    pub exit: bool,
    pub tasks: Vec<models::Task>,
    pub selected_tasks: Vec<Uuid>,
    pub state: state::AppState,
    pub db: Db,
    pub error: Option<String>,
}

impl App {
    pub fn new() -> Self {
        let state = state::AppState::new();
        let db = Db::new();

        let (tasks, error) = match TaskRepository::get_all(&db.connection) {
            Ok(tasks) => (tasks, None),
            Err(err) => (vec![], Some(err.to_string())),
        };

        App {
            exit: false,
            selected_tasks: Vec::new(),
            state,
            db,
            tasks,
            error,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
            let event = ratatui::crossterm::event::read()?;
            handle_key_event(self, &event, terminal);
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(1), Constraint::Length(1)])
            .split(frame.area());

        components::workspace::render(frame, layout[0], self);
        components::bottom_bar::render(frame, layout[1], self);

        match &mut self.state.active_modal {
            Some(ModalState::CreateTask { input }) => {
                components::modals::create_task::render(frame, input);
            }
            Some(ModalState::EditTask { task_id: _, input }) => {
                components::modals::edit_task::render(frame, input);
            }
            Some(ModalState::ArchivedTask {
                task_ids: _,
                selected_option,
                is_archived,
            }) => {
                components::modals::archive_task::render(frame, selected_option, *is_archived);
            }
            Some(ModalState::DeleteTask {
                task_ids: _,
                selected_option,
            }) => {
                components::modals::delete_task::render(frame, selected_option);
            }
            Some(ModalState::PriorityTask {
                task_ids: _,
                selected_option,
            }) => {
                components::modals::priority_task::render(frame, selected_option);
            }
            None => {}
        }
    }

    pub fn active_tasks(&self) -> Vec<Task> {
        let mut tasks = Task::get_active_tasks(&self.tasks);
        Task::sort_by_priority(&mut tasks);

        tasks
    }

    pub fn archived_tasks(&self) -> Vec<Task> {
        let mut tasks = Task::get_archived_tasks(&self.tasks);
        Task::sort_by_archived_date(&mut tasks);
        tasks
    }

    pub fn get_current_tasks(&self) -> Vec<Task> {
        match self.state.active_panel {
            state::PanelState::ActiveTasks => self.active_tasks(),
            state::PanelState::ArchivedTasks => self.archived_tasks(),
            state::PanelState::About => Vec::new(),
        }
    }
}

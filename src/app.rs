use std::io::Result;

use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::{
    components,
    keybindings::handle_key_event,
    state,
    storage::storage::{self, Storage},
};
use crate::{models, state::ModalState};

pub struct App {
    pub exit: bool,
    pub tasks: Vec<models::task::Task>,
    pub state: state::AppState,
    pub storage: Storage,
}

impl App {
    pub fn new() -> Self {
        let state = state::AppState::new();
        let storage = storage::Storage::new();

        return App {
            exit: false,
            tasks: storage.load(),
            storage,
            state,
        };
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
            let event = ratatui::crossterm::event::read()?;
            handle_key_event(self, &event);
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(frame.area());

        components::tasks::render(frame, layout[0], self);
        components::main_view::render(frame, layout[1], self);

        match &mut self.state.active_modal {
            Some(ModalState::CreateTask { input }) => {
                components::create_task::render(frame, input);
            }
            Some(ModalState::EditTask { task_id: _, input }) => {
                components::edit_task::render(frame, input);
            }
            Some(ModalState::DeleteTask {
                task_id: _,
                selected_option,
            }) => {
                components::remove_task::render(frame, selected_option);
            }
            None => {}
        }
    }
}

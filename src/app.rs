use std::io::Result;

use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::{components, keybindings::handle_key_event, state};
use crate::{models, state::ModalState};

pub struct App {
    pub exit: bool,
    pub tasks: Vec<models::task::Task>,
    pub state: state::AppState,
}

impl App {
    pub fn new() -> Self {
        let mut tasks: Vec<models::task::Task> = Vec::new();
        let task_one = models::task::Task::new(
            0,
            "Sed ut perspiciatis unde omnis".to_string(),
            "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo.".to_string(),
        );
        let task_two = models::task::Task::new(
            1,
            "Lorem Ipsum standard".to_string(),
            "But I must explain to you how all this mistaken idea of denouncing pleasure and praising pain was born and I will give you a complete account of the system".to_string(),
        );
        let task_three = models::task::Task::new(
            2,
            "De Finibus Bonorum et Malorum".to_string(),
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec eu congue augue. Integer felis risus, sagittis sit amet pretium eu.".to_string(),
        );
        tasks.push(task_one);
        tasks.push(task_two);
        tasks.push(task_three);

        let state = state::AppState::new();

        return App {
            exit: false,
            tasks,
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
            Some(ModalState::DeleteTask {
                index: _,
                selected_option,
            }) => {
                components::remove_task::render(frame, selected_option);
            }
            None => {}
        }
    }
}

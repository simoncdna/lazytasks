use std::io::Result;

use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::models;
use crate::{components, state};

pub struct App {
    pub exit: bool,
    pub tasks: Vec<models::task::Task>,
    pub state: state::AppState,
}

impl App {
    pub fn new() -> Self {
        let mut tasks: Vec<models::task::Task> = Vec::new();
        let task_one = models::task::Task::new(
            "0".to_string(),
            "task-01".to_string(),
            "Lorem ipsum dolor".to_string(),
        );
        let task_two = models::task::Task::new(
            "1".to_string(),
            "task-02".to_string(),
            "Lorem ipsum dolor".to_string(),
        );
        let task_three = models::task::Task::new(
            "2".to_string(),
            "task-03".to_string(),
            "Lorem ipsum dolor".to_string(),
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
            self.handle_key_event();
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
    }

    fn handle_key_event(&mut self) {
        if let Ok(crossterm::event::Event::Key(key)) = crossterm::event::read() {
            match key.code {
                crossterm::event::KeyCode::Char('q') => self.exit = true,
                crossterm::event::KeyCode::Char('j') => {
                    self.state.select_next_task(self.tasks.len());
                }
                crossterm::event::KeyCode::Char('k') => {
                    self.state.select_previous_task();
                }
                _ => {}
            }
        }
    }
}

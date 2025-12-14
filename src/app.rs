use std::io::Result;

use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::components;

pub struct App {}

impl App {
    pub fn new() -> Self {
        return App {};
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;
            if matches!(event::read()?, Event::Key(_)) {
                break Ok(());
            }
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(frame.area());

        components::tasks::render(frame, layout[0]);
        components::main_view::render(frame, layout[1]);
    }
}

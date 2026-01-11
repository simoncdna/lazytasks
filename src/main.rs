use std::io::Result;

mod actions;
mod app;
mod components;
mod constants;
mod db;
mod editor;
mod keybindings;
mod models;
mod state;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = app::App::new();
    let result = app.run(&mut terminal);

    ratatui::restore();

    result
}

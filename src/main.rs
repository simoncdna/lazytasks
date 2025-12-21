use std::io::Result;

mod app;
mod components;
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

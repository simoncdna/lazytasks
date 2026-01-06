use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::Paragraph,
};

use crate::{app::App, state::PanelState};

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let help_text = match app.state.active_panel {
        PanelState::ActiveTasks => {
            "Create: c | Edit (Title): e | Edit: E | Delete: d | Complete: y | Archive: a | Select: <Space> | Keybindings: ?"
        }
        PanelState::ArchivedTasks => {
            "Unarchive: a | Complete: y | Delete: d | Select: <Space> | Keybindings: ?"
        }
    };
    let bottom_bar = Paragraph::new(help_text).style(Style::default().fg(Color::Blue));

    frame.render_widget(bottom_bar, area);
}

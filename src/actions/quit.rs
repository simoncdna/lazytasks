use crate::app::App;

pub fn quit(app: &mut App) {
    app.exit = true;
}

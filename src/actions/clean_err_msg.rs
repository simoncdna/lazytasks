use crate::app::App;

pub fn clean_err_msg(app: &mut App) {
    app.error = None;
}

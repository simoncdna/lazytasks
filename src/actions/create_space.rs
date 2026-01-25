use crate::{app::App, db::repositories::SpaceRepository, models};

pub fn create_space(app: &mut App, title: String) {
    let new_space = models::Space::new(title);

    if let Err(e) = SpaceRepository::create(&app.db.connection, &new_space) {
        app.error = Some(e.to_string());

        return;
    };

    app.spaces.push(new_space);

    let new_index = app.active_tasks().len() - 1;
    app.state.active_tasks_state.select(Some(new_index));
}

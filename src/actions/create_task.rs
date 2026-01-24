use crate::{app::App, db::repositories::TaskRepository, models};

pub fn create_task(app: &mut App, title: String) {
    let new_task = models::Task::new(title);

    if let Err(e) = TaskRepository::create(&app.db.connection, &new_task) {
        app.error = Some(e.to_string());

        return;
    };
    app.tasks.push(new_task);

    let new_index = app.active_tasks().len() - 1;
    app.state.active_tasks_state.select(Some(new_index));
}

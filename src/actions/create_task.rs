use crate::{app::App, db::repositories::TaskRepository, models};

pub fn create_task(app: &mut App, title: String) {
    let new_task = models::Task::new(title);

    TaskRepository::create(&app.db.connection, &new_task);
    app.tasks.push(new_task);

    let new_index = app.active_tasks().len() - 1;
    app.state.active_tasks_state.select(Some(new_index));
}

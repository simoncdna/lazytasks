use ratatui::widgets::ListState;
use tui_input::Input;

/// The application global state
pub struct AppState {
    /// State of the task list (selected, scroll)
    pub tasks_list_state: ListState,

    pub active_modal: Option<ModalState>,
}

pub enum ModalState {
    CreateTask {
        input: Input,
    },
    EditTask {
        input: Input,
    },
    DeleteTask {
        index: usize,
        selected_option: ListState,
    },
}

impl AppState {
    pub fn new() -> Self {
        let mut tasks_list_state = ListState::default();
        tasks_list_state.select(Some(0));

        let mut delete_task_list_state = ListState::default();
        delete_task_list_state.select(Some(0));

        AppState {
            tasks_list_state: tasks_list_state,
            active_modal: None,
        }
    }

    pub fn select_next_task(&mut self, tasks_count: usize) {
        let current_task = self.tasks_list_state.selected();
        if current_task < Some(tasks_count - 1) {
            self.tasks_list_state.select_next();
        } else {
            self.tasks_list_state.select_first();
        }
    }

    pub fn select_previous_task(&mut self) {
        let current_task = self.tasks_list_state.selected();
        if current_task > Some(0) {
            self.tasks_list_state.select_previous();
        } else {
            self.tasks_list_state.select_last();
        }
    }

    pub fn open_create_task(&mut self) {
        self.active_modal = Some(ModalState::CreateTask {
            input: Input::default(),
        })
    }

    pub fn open_edit_task(&mut self, current_value: String) {
        self.active_modal = Some(ModalState::EditTask {
            input: Input::from(current_value),
        })
    }

    pub fn open_delete_task(&mut self) {
        let mut option_list_state = ListState::default();
        option_list_state.select(Some(0));
        self.active_modal = Some(ModalState::DeleteTask {
            index: self.tasks_list_state.selected().unwrap(),
            selected_option: option_list_state,
        })
    }

    pub fn close_modal(&mut self) {
        self.active_modal = None
    }
}

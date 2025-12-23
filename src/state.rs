use ratatui::widgets::ListState;
use tui_input::Input;
use uuid::Uuid;

/// The application global state
pub struct AppState {
    /// State of the task list (selected, scroll)
    pub tasks_list_state: ListState,

    /// State of the archived task list (selected, scroll)
    pub archived_tasks_list_state: ListState,

    /// State of the focus pane
    pub active_panel: PanelState,

    /// State of the current active modal (CreateTask, EditTask, ArchivedTask, DeleteTask)
    pub active_modal: Option<ModalState>,
}

#[derive(PartialEq, Eq)]
pub enum PanelState {
    ActiveTasks,
    ArchivedTasks,
}

pub enum ModalState {
    CreateTask {
        input: Input,
    },
    EditTask {
        task_id: Uuid,
        input: Input,
    },
    ArchivedTask {
        task_id: Uuid,
        selected_option: ListState,
    },
    DeleteTask {
        task_id: Uuid,
        selected_option: ListState,
    },
}

impl AppState {
    pub fn new() -> Self {
        let mut tasks_list_state = ListState::default();
        tasks_list_state.select(Some(0));

        let mut delete_task_list_state = ListState::default();
        delete_task_list_state.select(Some(0));

        let mut archived_tasks_list_state = ListState::default();
        archived_tasks_list_state.select(Some(0));

        AppState {
            tasks_list_state,
            archived_tasks_list_state,
            active_panel: PanelState::ActiveTasks,
            active_modal: None,
        }
    }

    pub fn toggle_active_panel(&mut self) {
        if self.active_panel == PanelState::ActiveTasks {
            self.active_panel = PanelState::ArchivedTasks
        } else {
            self.active_panel = PanelState::ActiveTasks
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

    pub fn select_next_archived_task(&mut self, tasks_count: usize) {
        let current_task = self.archived_tasks_list_state.selected();
        if current_task < Some(tasks_count - 1) {
            self.archived_tasks_list_state.select_next();
        } else {
            self.archived_tasks_list_state.select_first();
        }
    }

    pub fn select_previous_archived_task(&mut self) {
        let current_task = self.archived_tasks_list_state.selected();
        if current_task > Some(0) {
            self.archived_tasks_list_state.select_previous();
        } else {
            self.archived_tasks_list_state.select_last();
        }
    }

    pub fn open_create_task(&mut self) {
        self.active_modal = Some(ModalState::CreateTask {
            input: Input::default(),
        })
    }

    pub fn open_edit_task(&mut self, task_id: Uuid, current_value: String) {
        self.active_modal = Some(ModalState::EditTask {
            task_id,
            input: Input::from(current_value),
        })
    }

    pub fn open_archived_task(&mut self, task_id: Uuid) {
        let mut option_list_state = ListState::default();
        option_list_state.select(Some(0));
        self.active_modal = Some(ModalState::ArchivedTask {
            task_id,
            selected_option: option_list_state,
        })
    }

    pub fn open_delete_task(&mut self, task_id: Uuid) {
        let mut option_list_state = ListState::default();
        option_list_state.select(Some(0));
        self.active_modal = Some(ModalState::DeleteTask {
            task_id,
            selected_option: option_list_state,
        })
    }

    pub fn get_selected_list(&self) -> &ListState {
        match self.active_panel {
            PanelState::ActiveTasks => &self.tasks_list_state,
            PanelState::ArchivedTasks => &self.archived_tasks_list_state,
        }
    }

    pub fn close_modal(&mut self) {
        self.active_modal = None
    }
}

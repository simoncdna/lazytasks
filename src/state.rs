use ratatui::widgets::ListState;
use tui_input::Input;
use tui_tree_widget::TreeState;
use uuid::Uuid;

/// The application global state
pub struct AppState {
    /// State of the task list (selected, scroll)
    pub active_tasks_state: ListState,

    /// State of the archived task list (selected, scroll)
    pub archived_tasks_state: ListState,

    /// State of the focus pane
    pub active_panel: PanelState,

    /// State of the workspaces tree (identifier = UUID as String)
    pub workspaces_tree_state: TreeState<String>,

    /// State of the current active modal (CreateTask, EditTask, ArchivedTask, DeleteTask)
    pub active_modal: Option<ModalState>,
}

#[derive(PartialEq, Eq)]
pub enum PanelState {
    ActiveTasks,
    ArchivedTasks,
    About,
}

pub enum ModalState {
    CreateTask {
        input: Input,
        workspace_id: String,
    },
    EditTask {
        task_id: Uuid,
        input: Input,
    },
    ArchivedTask {
        task_ids: Vec<Uuid>,
        selected_option: ListState,
        is_archived: bool,
    },
    DeleteTask {
        task_ids: Vec<Uuid>,
        selected_option: ListState,
    },
    PriorityTask {
        task_ids: Vec<Uuid>,
        selected_option: ListState,
    },
    CreateWorkspace {
        input: Input,
    },
    DeleteWorkspace {
        workspace_id: Uuid,
        selected_option: ListState,
    },
    ArchiveWorkspace {
        workspace_id: Uuid,
        selected_option: ListState,
        is_archived: bool,
    },
    MoveTask {
        task_id: Uuid,
        selected_option: ListState,
    },
}

impl AppState {
    pub fn new() -> Self {
        let mut active_tasks_state = ListState::default();
        active_tasks_state.select(Some(0));

        let mut archived_tasks_state = ListState::default();
        archived_tasks_state.select(Some(0));

        let mut workspaces_tree_state = TreeState::default();
        workspaces_tree_state.select_first();

        AppState {
            active_tasks_state,
            archived_tasks_state,
            active_panel: PanelState::ActiveTasks,
            active_modal: None,
            workspaces_tree_state,
        }
    }

    pub fn toggle_active_panel(&mut self) {
        if self.active_panel == PanelState::ActiveTasks {
            self.active_panel = PanelState::ArchivedTasks
        } else if self.active_panel == PanelState::ArchivedTasks {
            self.active_panel = PanelState::About
        } else if self.active_panel == PanelState::About {
            self.active_panel = PanelState::ActiveTasks
        }
    }

    pub fn select_next_task(&mut self, tasks_count: usize) {
        if let Some(current_pannel_state) = self.get_selected_panel_state() {
            let current_task = current_pannel_state.selected();

            if current_task < Some(tasks_count - 1) {
                current_pannel_state.select_next();
            } else {
                current_pannel_state.select_first();
            }
        }
    }

    pub fn select_previous_task(&mut self, tasks_count: usize) {
        if let Some(current_pannel_state) = self.get_selected_panel_state() {
            let current_task = current_pannel_state.selected();

            if current_task > Some(0) {
                current_pannel_state.select_previous();
            } else {
                current_pannel_state.select(Some(tasks_count - 1));
            }
        }
    }

    pub fn open_create_task(&mut self, workspace_id: String) {
        self.active_modal = Some(ModalState::CreateTask {
            input: Input::default(),
            workspace_id,
        })
    }

    pub fn open_edit_task(&mut self, task_id: Uuid, current_value: String) {
        self.active_modal = Some(ModalState::EditTask {
            task_id,
            input: Input::from(current_value),
        })
    }

    pub fn open_archived_task(&mut self, task_ids: Vec<Uuid>, is_archived: bool) {
        let mut option_list_state = ListState::default();
        option_list_state.select(Some(0));
        self.active_modal = Some(ModalState::ArchivedTask {
            task_ids,
            selected_option: option_list_state,
            is_archived,
        })
    }

    pub fn open_delete_task(&mut self, task_ids: Vec<Uuid>) {
        let mut option_list_state = ListState::default();
        option_list_state.select(Some(0));
        self.active_modal = Some(ModalState::DeleteTask {
            task_ids,
            selected_option: option_list_state,
        })
    }

    pub fn open_priority_task(&mut self, task_ids: Vec<Uuid>) {
        let mut option_list_state = ListState::default();
        option_list_state.select(Some(0));
        self.active_modal = Some(ModalState::PriorityTask {
            task_ids,
            selected_option: option_list_state,
        })
    }

    pub fn get_selected_panel_state(&mut self) -> Option<&mut ListState> {
        match self.active_panel {
            PanelState::ActiveTasks => Some(&mut self.active_tasks_state),
            PanelState::ArchivedTasks => Some(&mut self.archived_tasks_state),
            PanelState::About => None,
        }
    }

    pub fn open_create_workspace(&mut self) {
        self.active_modal = Some(ModalState::CreateWorkspace {
            input: Input::default(),
        })
    }

    pub fn close_modal(&mut self) {
        self.active_modal = None
    }

    pub fn open_delete_workspace(&mut self, workspace_id: Uuid) {
        let mut option_list_state = ListState::default();
        option_list_state.select(Some(0));
        self.active_modal = Some(ModalState::DeleteWorkspace {
            workspace_id,
            selected_option: option_list_state,
        })
    }

    pub fn open_archive_workspace(&mut self, workspace_id: Uuid, is_archived: bool) {
        let mut option_list_state = ListState::default();
        option_list_state.select(Some(0));
        self.active_modal = Some(ModalState::ArchiveWorkspace {
            workspace_id,
            selected_option: option_list_state,
            is_archived,
        })
    }

    pub fn open_move_task(&mut self, task_id: Uuid) {
        let mut option_list_state = ListState::default();
        option_list_state.select(Some(0));
        self.active_modal = Some(ModalState::MoveTask {
            task_id,
            selected_option: option_list_state,
        })
    }
}

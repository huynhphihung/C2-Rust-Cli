use ratatui::widgets::{ListState, TableState};
use uuid::Uuid;

use crate::message::{SessionInfo, ShellState};

pub struct SessionState {
    pub info: SessionInfo,
    pub shell_state: ShellState,
}

pub struct AppState {
    pub current_window: View,

    pub sessions: Vec<SessionState>,
    pub session_list_state: TableState,
    pub menu_list_state: ListState,

    pub current_session: Option<Uuid>,

    pub should_quit: bool,
}

impl AppState {
    pub fn new() -> Self {
        let mut session_list_state = TableState::default();
        session_list_state.select(Some(0));

        let mut menu_list_state = ListState::default();
        menu_list_state.select(Some(0));
        Self {
            current_window: View::Menu,
            sessions: Vec::new(),
            session_list_state,
            menu_list_state,
            should_quit: false,
            current_session: None,
        }
    }
}

pub enum View {
    Menu,
    Sessions,
    Shell,
}

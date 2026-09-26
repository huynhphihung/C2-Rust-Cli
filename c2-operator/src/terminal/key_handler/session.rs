use crossterm::event::{KeyCode, KeyEvent};

use crate::terminal::state::{AppState, View};

pub fn handle_key(state: &mut AppState, key: KeyEvent) {
    match key {
        KeyEvent {
            code: KeyCode::Esc, ..
        } => state.current_window = View::Menu,
        KeyEvent {
            code: KeyCode::Char('j'),
            ..
        } => state.session_list_state.select_next(),
        KeyEvent {
            code: KeyCode::Char('k'),
            ..
        } => {
            state.session_list_state.select_previous();
        }
        KeyEvent {
            code: KeyCode::Enter,
            ..
        } => {
            if let Some(index) = state.session_list_state.selected() {
                if let Some(session) = state.sessions.get(index) {
                    state.current_session = Some(session.info.session_id);
                    state.current_window = View::Shell
                }
            }
        }
        _ => {}
    }
}

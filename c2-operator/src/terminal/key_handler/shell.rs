use color_eyre::eyre::{Ok, Result};
use crossterm::event::{KeyCode, KeyEvent};

use crate::terminal::{
    app::OperatorApp,
    key_handler::session,
    state::{AppState, SessionState, View},
};

pub fn current_session_mut(state: &mut AppState) -> Option<&mut SessionState> {
    let session_id = state.current_session?;

    state
        .sessions
        .iter_mut()
        .find(|session| session.info.session_id == session_id)
}

pub async fn handle_key(app: &mut OperatorApp, key: KeyEvent) -> Result<()> {
    match key {
        KeyEvent {
            code: KeyCode::Esc, ..
        } => app.app_state.current_window = View::Sessions,
        KeyEvent {
            code: KeyCode::Char(c),
            ..
        } => {
            if let Some(session) = current_session_mut(&mut app.app_state) {
                session.shell_state.input.push(c);
            }
        }
        KeyEvent {
            code: KeyCode::Backspace,
            ..
        } => {
            if let Some(session) = current_session_mut(&mut app.app_state) {
                session.shell_state.input.pop();
            }
        }
        KeyEvent {
            code: KeyCode::Enter,
            ..
        } => {
            app.create_task().await?;
        }
        KeyEvent {
            code: KeyCode::Up, ..
        } => {
            if let Some(session) = current_session_mut(&mut app.app_state) {
                session.shell_state.scroll = session.shell_state.scroll.saturating_sub(1)
            }
        }
        KeyEvent {
            code: KeyCode::Down,
            ..
        } => {
            if let Some(session) = current_session_mut(&mut app.app_state) {
                session.shell_state.scroll = session.shell_state.scroll.saturating_add(1)
            }
        }
        _ => {}
    }
    Ok(())
}

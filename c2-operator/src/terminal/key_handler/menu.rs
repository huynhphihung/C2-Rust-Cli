use color_eyre::eyre::{Ok, Result};
use crossterm::event::{KeyCode, KeyEvent};

use crate::terminal::{app::OperatorApp, state::View};

pub async fn handle_key(app: &mut OperatorApp, key: KeyEvent) -> Result<()> {
    match key {
        KeyEvent {
            code: KeyCode::Esc, ..
        } => app.app_state.should_quit = true,
        KeyEvent {
            code: KeyCode::Char('j'),
            ..
        } => app.app_state.menu_list_state.select_next(),
        KeyEvent {
            code: KeyCode::Char('k'),
            ..
        } => app.app_state.menu_list_state.select_previous(),
        KeyEvent {
            code: KeyCode::Enter,
            ..
        } => match app.app_state.menu_list_state.selected() {
            Some(0) => {
                app.app_state.current_window = View::Sessions;
                app.requeset_sessions().await?;
            }
            Some(1) => app.app_state.should_quit = true,
            _ => {}
        },
        _ => {}
    }
    Ok(())
}

use ratatui::Frame;

use crate::terminal::{
    app::OperatorApp,
    state::View,
    ui::{menu, session, shell},
};

pub fn ui(frame: &mut Frame, app: &mut OperatorApp) {
    match app.app_state.current_window {
        View::Menu => menu::render(frame, frame.area(), &mut app.app_state),
        View::Sessions => session::render(frame, frame.area(), app),
        View::Shell => shell::render(frame, frame.area(), app),
    }
}

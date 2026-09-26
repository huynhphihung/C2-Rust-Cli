use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::terminal::app::OperatorApp;

pub fn render(frame: &mut Frame, area: Rect, app: &mut OperatorApp) {
    let block = Block::default().borders(Borders::ALL);
    let Some(session) = app
        .app_state
        .sessions
        .iter_mut()
        .find(|session| Some(session.info.session_id) == app.app_state.current_session)
    else {
        frame.render_widget(Paragraph::new("No session selected").block(block), area);
        return;
    };

    let mut lines = session.shell_state.output.clone();
    let input = format!("$ {}_", session.shell_state.input);
    lines.push(input);

    let text = lines.join("\n");

    let para = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((session.shell_state.scroll, 0));

    frame.render_widget(para, area);
}

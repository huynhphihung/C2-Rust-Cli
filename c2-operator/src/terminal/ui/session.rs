use std::vec;

use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};

use crate::terminal::app::OperatorApp;

pub fn render(frame: &mut Frame, area: Rect, app: &mut OperatorApp) {
    let block = Block::default().borders(Borders::ALL);
    let header_row = Row::new(vec![
        Cell::from("Session ID").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Agent ID").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Host name").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Username").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("IP Adress").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("OS").style(Style::default().add_modifier(Modifier::BOLD)),
    ])
    .height(1)
    .bottom_margin(0);

    let rows: Vec<Row> = app
        .app_state
        .sessions
        .iter()
        .map(|session| {
            Row::new(vec![
                Cell::from(session.info.session_id.to_string()).style(Style::new()),
                Cell::from(session.info.info.agent_id.to_string()),
                Cell::from(session.info.info.hostname.clone()),
                Cell::from(session.info.info.username.clone()),
                Cell::from(session.info.peer_addr.to_string()),
                Cell::from(session.info.info.os.to_string()),
            ])
            .height(1)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Min(40),
            Constraint::Min(40),
            Constraint::Min(20),
            Constraint::Min(20),
            Constraint::Min(20),
            Constraint::Min(20),
        ],
    )
    .header(header_row)
    .block(block)
    .highlight_symbol(" ")
    .row_highlight_style(Style::new().yellow());

    frame.render_stateful_widget(table, area, &mut app.app_state.session_list_state);
}

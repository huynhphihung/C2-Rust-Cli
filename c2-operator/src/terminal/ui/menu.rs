use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier},
    widgets::{Block, Borders, List},
};

use crate::terminal::state::AppState;

pub fn render(frame: &mut Frame, area: Rect, state: &mut AppState) {
    let block = Block::default().borders(Borders::ALL);

    let menu_items = ["Sessions", "Exit"];
    let menu_list = List::new(menu_items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ")
        .block(block);

    frame.render_stateful_widget(menu_list, area, &mut state.menu_list_state);
}

use ratatui::{
    Frame, layout::{
        Constraint,
        Direction,
        Layout
    }, style::{Modifier, Style}, widgets::{Block, Borders, List, ListState}
};

use crate::{
    app::{ActiveTab, App},
    tabs
};

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Min(1)])
        .split(frame.area());

    let menu_items = vec!["F1 Console", "F2 Stats", "F3 Shop", "F4 Settings"];
    let menu = List::new(menu_items)
        .block(Block::default().borders(Borders::ALL).title("Menu"))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    let mut list_state = ListState::default();
    let tab_index = app.active_tab.index();

    list_state.select(Some(tab_index));

    frame.render_stateful_widget(menu, chunks[0], &mut list_state);

    match app.active_tab {
        ActiveTab::Console => tabs::console::draw(frame, chunks[1], &app.console),
        ActiveTab::Stats => tabs::stats::draw(frame, chunks[1]),
        ActiveTab::Shop =>tabs::shop::draw(frame, chunks[1]),
        ActiveTab::Settings => tabs::settings::draw(frame, chunks[1])
    }
}
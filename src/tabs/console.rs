use ratatui::{Frame, layout::{Constraint, Direction, Layout, Rect}, widgets::{Block, Borders, Paragraph, Wrap}};

use crate::app::ConsoleState;

pub fn draw(frame: &mut Frame, area: Rect, console: &ConsoleState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(area);

    let history_text = console.get_history().join("\n");
    let history = Paragraph::new(history_text)
        .block(Block::default().borders(Borders::ALL).title("Console"))
        .wrap(Wrap { trim: false });

    frame.render_widget(history, chunks[0]);

    let input = Paragraph::new(format!("> {}", console.get_input()))
        .block(Block::default().borders(Borders::ALL).title("Input"));

    frame.render_widget(input, chunks[1]);

    let cursor_x = chunks[1].x + 1 + 2 + console.get_input().len() as u16;
    let cursor_y = chunks[1].y + 1;

    frame.set_cursor_position((cursor_x, cursor_y));
}
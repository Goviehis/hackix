use ratatui::{Frame, layout::Rect, widgets::{Block, Borders, Paragraph}};

pub fn draw(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Settings");
    
    let text = Paragraph::new("There are settings").block(block);
    frame.render_widget(text, area);
}
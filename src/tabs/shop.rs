use ratatui::{Frame, layout::Rect, widgets::{Block, Borders, Paragraph}};

pub fn draw(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Shop");
    
    let text = Paragraph::new("There are shop").block(block);
    frame.render_widget(text, area);
}
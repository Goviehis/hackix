use ratatui::{Frame, layout::Rect, widgets::{Block, Borders, Paragraph}};

pub fn draw(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Stats");
    
    let text = Paragraph::new("There are stats").block(block);
    frame.render_widget(text, area);
}
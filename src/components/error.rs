use super::Component;
use color_eyre::Result;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

/// Represents an error component in the application.
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: String) -> Self {
        Error { message }
    }
}

impl Component for Error {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        frame.render_widget(
            Paragraph::new(self.message.clone())
                .style(Style::default().fg(Color::Red))
                .block(ratatui::widgets::Block::bordered().title("Error")),
            area,
        );
        Ok(())
    }
}

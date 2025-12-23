use ratatui::{
    Frame,
    layout::Rect,
    style::Color,
    widgets::{Block, BorderType, Borders, Clear},
};

pub struct Modal {
    title: String,
    width: u16,
    height: u16,
}

impl Modal {
    pub fn new(title: impl Into<String>) -> Self {
        Modal {
            title: title.into(),
            width: 100,
            height: 10,
        }
    }

    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    pub fn render(&self, frame: &mut Frame) -> Rect {
        let area = self.centered_rect(frame.area());

        frame.render_widget(Clear, area);

        let block = Block::default()
            .title(format!(" {} ", self.title))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Color::Green);

        let inner = block.inner(area);
        frame.render_widget(block, area);

        inner
    }

    fn centered_rect(&self, area: Rect) -> Rect {
        let x = area.x + (area.width.saturating_sub(self.width)) / 2;
        let y = area.y + (area.height.saturating_sub(self.height)) / 2;

        Rect::new(
            x,
            y,
            self.width.min(area.width),
            self.height.min(area.height),
        )
    }
}

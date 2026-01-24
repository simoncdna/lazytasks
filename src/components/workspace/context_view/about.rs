use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::constants::LOGO;

pub fn render(frame: &mut Frame, area: Rect, title: String) {
    let mut lines: Vec<Line> = LOGO
        .lines()
        .map(|line| Line::styled(line, Style::default()))
        .collect();

    lines.push(Line::raw(""));
    lines.push(Line::styled(
        "Copyright 2026 Simon Cardona".to_string(),
        Style::default().fg(Color::White),
    ));
    lines.push(Line::raw(""));
    lines.push(Line::from(vec![
        Span::styled("Raise an Issue: ", Style::default().fg(Color::White)),
        Span::styled(
            "https://github.com/simoncdna/lazytasks/issues",
            Style::default().underlined(),
        ),
    ]));
    lines.push(Line::raw(""));
    lines.push(Line::from(vec![
        Span::styled("Release Notes: ", Style::default().fg(Color::White)),
        Span::styled(
            "https://github.com/simoncdna/lazytasks/releases",
            Style::default().underlined(),
        ),
    ]));
    lines.push(Line::raw(""));
    lines.push(
        Line::from(vec![
            Span::styled("Become a sponsor: ", Style::default()),
            Span::styled(
                "https://github.com/sponsors/simoncdna",
                Style::default().underlined(),
            ),
        ])
        .style(Color::Magenta),
    );

    let about_view = Paragraph::new(Text::from(lines)).block(
        Block::new()
            .title(title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );

    frame.render_widget(about_view, area);
}

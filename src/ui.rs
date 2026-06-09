use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    let [left, right] = Layout::horizontal([Constraint::Percentage(50); 2]).areas(frame.area());

    frame.render_widget(
        Paragraph::new(format!(
            "
        Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
        Press `j` and `k` to increment and decrement the counter respectively.\n\
        Counter: {}
      ",
            app.counter
        ))
        .block(
            Block::default()
                .title("Counter App")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::LightYellow))
        .alignment(Alignment::Center),
        left,
    );

    frame.render_widget(
        Paragraph::new("Test").block(Block::default()
            .title("Right one")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center),
        right,
    );
}

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    // Vertical: header + content
    let vertical = Layout::vertical([Constraint::Length(3), Constraint::Min(5)]);
    let [header_area, content_area] = vertical.areas(frame.area());

    // Horizontal: chat + sidebar
    let [chat_area, sidebar_area] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Length(30)]).areas(content_area);

    // Header
    frame.render_widget(
        Paragraph::new("rusty — AI Rust Agent")
            .block(Block::default().title("Status").borders(Borders::ALL))
            .style(Style::default().fg(Color::LightYellow))
            .alignment(Alignment::Center),
        header_area,
    );

    // Chat / prompt area
    frame.render_widget(
        Paragraph::new("Chat / prompt area")
            .block(Block::default().title("Chat").borders(Borders::ALL))
            .style(Style::default().fg(Color::LightYellow))
            .alignment(Alignment::Left),
        chat_area,
    );

    // Sidebar (tokens, etc.)
    frame.render_widget(
        Paragraph::new("Tokens: 0\nStatus: idle")
            .block(Block::default().title("Info").borders(Borders::ALL))
            .style(Style::default().fg(Color::LightYellow))
            .alignment(Alignment::Left),
        sidebar_area,
    );
}

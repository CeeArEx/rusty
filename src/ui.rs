use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Position},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::{App, InputMode};

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

    // Chat area split into messages + input
    let chat_layout = Layout::vertical([Constraint::Min(1), Constraint::Length(3)]);
    let [messages_area, input_area] = chat_layout.areas(chat_area);

    // Messages
    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .map(|m| {
            let content = Line::from(Span::raw(m.clone()));
            ListItem::new(content)
        })
        .collect();
    let messages_widget = List::new(messages)
        .block(Block::default().title("Chat").borders(Borders::ALL));
    frame.render_widget(messages_widget, messages_area);

    // Input box
    let input = Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Editing => Style::default().fg(Color::Yellow),
            InputMode::Normal => Style::default(),
        })
        .block(Block::default().title("Input").borders(Borders::ALL));
    frame.render_widget(input, input_area);

    // Cursor
    if app.input_mode == InputMode::Editing {
        frame.set_cursor_position(Position::new(
            input_area.x + app.cursor as u16 + 1,
            input_area.y + 1,
        ));
    }

    // Sidebar (tokens, etc.)
    frame.render_widget(
        Paragraph::new("Tokens: 0\nStatus: idle")
            .block(Block::default().title("Info").borders(Borders::ALL))
            .style(Style::default().fg(Color::LightYellow))
            .alignment(Alignment::Left),
        sidebar_area,
    );
}

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, InputMode};

pub fn update(app: &mut App, key_event: KeyEvent) {
    match app.input_mode {
        InputMode::Editing => match key_event.code {
            KeyCode::Enter => app.submit_message(),
            KeyCode::Char(c) => app.enter_char(c),
            KeyCode::Backspace => app.delete_char(),
            KeyCode::Left => app.move_cursor_left(),
            KeyCode::Right => app.move_cursor_right(),
            KeyCode::Esc => app.input_mode = InputMode::Normal,
            _ => {}
        },
        InputMode::Normal => match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => app.quit(),
            KeyCode::Char('c') | KeyCode::Char('C')
                if key_event.modifiers == KeyModifiers::CONTROL =>
            {
                app.quit()
            }
            KeyCode::Right | KeyCode::Char('j') => app.increment_counter(),
            KeyCode::Left | KeyCode::Char('k') => app.decrement_counter(),
            KeyCode::Char('e') => app.input_mode = InputMode::Editing,
            _ => {}
        },
    }
}

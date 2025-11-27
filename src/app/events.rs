use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

use super::state::{App, AppMode};


pub fn handle_event(app: &mut App, event: Event) {
    if let Event::Key(key) = event {
        if key.kind == KeyEventKind::Press {
            handle_key_press(app, key);
        }
    }
}


fn handle_key_press(app: &mut App, key: KeyEvent) {
    match app.mode {
        AppMode::Normal => match key.code {
            KeyCode::Char('q') => app.quit(),
            KeyCode::Char('j') | KeyCode::Down => app.select_next(),
            KeyCode::Char('k') | KeyCode::Up => app.select_previous(),
            KeyCode::Char('/') => app.enter_editing_mode(),
            KeyCode::Enter => app.confirm_selection(),
            _ => {}
        },
        AppMode::Editing => match key.code {
            KeyCode::Esc => app.enter_normal_mode(),
            KeyCode::Enter => app.confirm_selection(),
            KeyCode::Up => app.select_previous(),
            KeyCode::Down => app.select_next(),
            KeyCode::Char(c) => app.on_char_input(c),
            KeyCode::Backspace => app.on_backspace(),
            _ => {}
        },
    }
}
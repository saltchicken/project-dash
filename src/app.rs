// ‼️ Declare submodules. Rust looks for them in src/app/*.rs
pub mod fs;
pub mod tui;
pub mod ui;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use futures::StreamExt;
use ratatui::widgets::ListState;
use std::path::PathBuf;

// ‼️ Import from the submodules declared above

#[derive(Debug, PartialEq)]
pub enum AppMode {
    Normal,
    Editing,
}

pub struct App {
    pub running: bool,
    pub folders: Vec<String>,
    pub filtered_folders: Vec<String>,
    pub list_state: ListState,
    pub desktop_path: PathBuf,
    pub result: Option<PathBuf>,
    pub input_text: String,
    pub mode: AppMode,
}

impl App {
    pub fn new() -> Result<Self> {
        let desktop_path = fs::get_desktop_path()?;
        let folders = fs::get_folders(&desktop_path)?;
        let filtered_folders = folders.clone();

        let mut list_state = ListState::default();
        if !filtered_folders.is_empty() {
            list_state.select(Some(0));
        }

        Ok(Self {
            running: true,
            folders,
            filtered_folders,
            list_state,
            desktop_path,
            result: None,
            input_text: String::new(),
            mode: AppMode::Normal,
        })
    }

    pub async fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        let mut event_stream = event::EventStream::new();

        while self.running {
            terminal.draw(|frame| ui::render(self, frame))?;

            if let Some(Ok(event)) = event_stream.next().await {
                self.handle_event(event);
            }
        }
        Ok(())
    }

    fn handle_event(&mut self, event: Event) {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                self.handle_key_press(key);
            }
        }
    }

    fn handle_key_press(&mut self, key: KeyEvent) {
        match self.mode {
            AppMode::Normal => match key.code {
                KeyCode::Char('q') => self.quit(),
                KeyCode::Char('j') | KeyCode::Down => self.select_next(),
                KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
                KeyCode::Char('/') => self.enter_editing_mode(),
                KeyCode::Enter => self.confirm_selection(),
                _ => {}
            },
            AppMode::Editing => match key.code {
                KeyCode::Esc => self.enter_normal_mode(),
                KeyCode::Enter => self.confirm_selection(),
                KeyCode::Up => self.select_previous(),
                KeyCode::Down => self.select_next(),
                KeyCode::Char(c) => self.on_char_input(c),
                KeyCode::Backspace => self.on_backspace(),
                _ => {}
            },
        }
    }

    fn enter_editing_mode(&mut self) {
        self.mode = AppMode::Editing;
    }

    fn enter_normal_mode(&mut self) {
        self.mode = AppMode::Normal;
        self.input_text.clear();
        self.apply_filter();
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn on_char_input(&mut self, c: char) {
        self.input_text.push(c);
        self.apply_filter();
    }

    fn on_backspace(&mut self) {
        self.input_text.pop();
        self.apply_filter();
    }

    fn apply_filter(&mut self) {
        let filter_text = self.input_text.to_lowercase();

        self.filtered_folders = self
            .folders
            .iter()
            .filter(|f| f.to_lowercase().contains(&filter_text))
            .cloned()
            .collect();

        self.adjust_selection_after_filter();
    }

    fn adjust_selection_after_filter(&mut self) {
        if self.filtered_folders.is_empty() {
            self.list_state.select(None);
        } else {
            match self.list_state.selected() {
                Some(selected) if selected >= self.filtered_folders.len() => {
                    self.list_state
                        .select(Some(self.filtered_folders.len() - 1));
                }
                None => {
                    self.list_state.select(Some(0));
                }
                _ => {}
            }
        }
    }

    fn select_next(&mut self) {
        if self.filtered_folders.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => (i + 1) % self.filtered_folders.len(),
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn select_previous(&mut self) {
        if self.filtered_folders.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.filtered_folders.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn confirm_selection(&mut self) {
        if let Some(selected_index) = self.list_state.selected() {
            if let Some(selected_folder) = self.filtered_folders.get(selected_index) {
                let full_path = self.desktop_path.join(selected_folder);
                self.result = Some(full_path);
            }
        }
        self.running = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_logic() {
        let folders = vec!["Alpha".to_string(), "Beta".to_string(), "Gamma".to_string()];
        let input = "a";
        let filtered: Vec<String> = folders
            .iter()
            .filter(|f| f.to_lowercase().contains(input))
            .cloned()
            .collect();
        assert_eq!(filtered.len(), 3);

        let input_z = "z";
        let filtered_z: Vec<String> = folders
            .iter()
            .filter(|f| f.to_lowercase().contains(input_z))
            .cloned()
            .collect();
        assert_eq!(filtered_z.len(), 0);
    }
}

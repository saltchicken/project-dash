use super::fs;
use color_eyre::Result;
use ratatui::widgets::ListState;
use std::path::PathBuf;

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
    pub projects_path: PathBuf,
    pub result: Option<PathBuf>,
    pub input_text: String,
    pub mode: AppMode,
}

impl App {
    pub fn new() -> Result<Self> {
        let projects_path = fs::get_projects_path()?;
        let folders = fs::get_folders(&projects_path)?;
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
            projects_path,
            result: None,
            input_text: String::new(),
            mode: AppMode::Normal,
        })
    }

    pub fn enter_editing_mode(&mut self) {
        self.mode = AppMode::Editing;
    }

    pub fn enter_normal_mode(&mut self) {
        self.mode = AppMode::Normal;
        self.input_text.clear();
        self.apply_filter();
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn on_char_input(&mut self, c: char) {
        self.input_text.push(c);
        self.apply_filter();
    }

    pub fn on_backspace(&mut self) {
        self.input_text.pop();
        self.apply_filter();
    }

    pub fn apply_filter(&mut self) {
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

    pub fn select_next(&mut self) {
        if self.filtered_folders.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => (i + 1) % self.filtered_folders.len(),
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn select_previous(&mut self) {
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

    pub fn confirm_selection(&mut self) {
        if let Some(selected_index) = self.list_state.selected() {
            if let Some(selected_folder) = self.filtered_folders.get(selected_index) {
                let full_path = self.projects_path.join(selected_folder);
                self.result = Some(full_path);
            }
        }
        self.running = false;
    }
}


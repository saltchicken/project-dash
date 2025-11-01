use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    prelude::*,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};
use std::env;
use std::io::stderr;
use std::path::PathBuf;

struct App {
    running: bool,
    folders: Vec<String>,
    filtered_folders: Vec<String>,
    list_state: ListState,
    desktop_path: PathBuf,
    result: Option<PathBuf>,
    input_text: String,
}

impl App {
    fn new() -> Result<Self> {
        let home_dir =
            env::var("HOME").map_err(|_| color_eyre::eyre::eyre!("Could not find HOME env var"))?;
        let desktop_path = PathBuf::from(home_dir).join("Desktop");
        if !desktop_path.is_dir() {
            return Err(color_eyre::eyre::eyre!(
                "~/Desktop directory not found at: {}",
                desktop_path.display()
            ));
        }
        let folders: Vec<String> = std::fs::read_dir(&desktop_path)?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_dir())
            .map(|entry| entry.file_name().into_string().unwrap_or_default())
            .filter(|s| !s.is_empty() && !s.starts_with('.'))
            .collect();
        if folders.is_empty() {
            return Err(color_eyre::eyre::eyre!("No folders found in ~/Desktop"));
        }
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        let filtered_folders = folders.clone();
        Ok(Self {
            running: true,
            folders,
            filtered_folders,
            list_state,
            desktop_path,
            result: None,
            input_text: String::new(),
        })
    }

    // ‼️ CHANGED: No longer async
    fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<std::io::Stderr>>) -> Result<()> {
        while self.running {
            terminal.draw(|frame| self.render(frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => self.quit(),
                        KeyCode::Down => self.select_next(),
                        KeyCode::Up => self.select_previous(),
                        KeyCode::Enter => self.confirm_selection(),
                        KeyCode::Char(c) => {
                            self.input_text.push(c);
                            self.apply_filter();
                        }
                        KeyCode::Backspace => {
                            self.input_text.pop();
                            self.apply_filter();
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let items: Vec<ListItem> = self
            .filtered_folders // Use the filtered list
            .iter()
            .map(|f| ListItem::new(f.as_str()))
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Select a Folder (Filter: {})", self.input_text)),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Gray)
                    .fg(Color::Black),
            )
            .highlight_symbol(">> ");

        let area = centered_rect(60, 50, frame.area());
        frame.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn apply_filter(&mut self) {
        let filter_text = self.input_text.to_lowercase();
        self.filtered_folders = self
            .folders // Filter from the original, complete list
            .iter()
            .filter(|f| f.to_lowercase().contains(&filter_text))
            .cloned()
            .collect();

        if self.filtered_folders.is_empty() {
            self.list_state.select(None);
        } else {
            // If selection is now out of bounds, select the last item
            if let Some(selected) = self.list_state.selected() {
                if selected >= self.filtered_folders.len() {
                    self.list_state
                        .select(Some(self.filtered_folders.len() - 1));
                }
            } else {
                // If nothing was selected, select the first
                self.list_state.select(Some(0));
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

fn main() -> Result<()> {
    color_eyre::install()?;
    //  --- MANUAL TUI SETUP on STDERR ---
    enable_raw_mode()?;
    execute!(stderr(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stderr());
    let mut terminal = Terminal::new(backend)?;
    //  --- END MANUAL SETUP ---

    let mut app = App::new()?;
    let run_result = app.run(&mut terminal);

    //  --- MANUAL TUI RESTORE from STDERR ---
    disable_raw_mode()?;
    execute!(stderr(), LeaveAlternateScreen)?;
    //  --- END MANUAL RESTORE ---

    if let Err(e) = run_result {
        eprintln!("Application error: {}", e);
    } else if let Some(folder_path) = app.result {
        // Print the final result to STDOUT
        println!("{}", folder_path.display());
    }

    Ok(())
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}

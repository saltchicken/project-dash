use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use futures::StreamExt;
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
    list_state: ListState,
    desktop_path: PathBuf,
    result: Option<PathBuf>,
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

        // Read the directory and collect folder names
        let folders: Vec<String> = std::fs::read_dir(&desktop_path)?
            .filter_map(Result::ok) // Ignore entries we can't read
            .filter(|entry| entry.path().is_dir()) // Only include directories
            .map(|entry| entry.file_name().into_string().unwrap_or_default())
            .filter(|s| !s.is_empty() && !s.starts_with('.')) // Filter out empty or hidden
            .collect();

        if folders.is_empty() {
            return Err(color_eyre::eyre::eyre!("No folders found in ~/Desktop"));
        }

        // State for tracking the selected item
        let mut list_state = ListState::default();
        list_state.select(Some(0)); // Select the first item

        Ok(Self {
            running: true,
            folders,
            list_state,
            desktop_path,
            result: None,
        })
    }

    async fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stderr>>,
    ) -> Result<()> {
        let mut event_stream = event::EventStream::new();
        while self.running {
            terminal.draw(|frame| self.render(frame))?;

            if let Some(Ok(event)) = event_stream.next().await {
                if let Event::Key(key) = event {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Esc => self.quit(),
                            KeyCode::Down => self.select_next(),
                            KeyCode::Up => self.select_previous(),
                            KeyCode::Enter => self.confirm_selection(),
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let items: Vec<ListItem> = self
            .folders
            .iter()
            .map(|f| ListItem::new(f.as_str()))
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Select a Folder"),
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

    fn select_next(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            let next = (selected + 1) % self.folders.len();
            self.list_state.select(Some(next));
        }
    }

    fn select_previous(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            let prev = if selected == 0 {
                self.folders.len() - 1
            } else {
                selected - 1
            };
            self.list_state.select(Some(prev));
        }
    }

    fn confirm_selection(&mut self) {
        if let Some(selected_index) = self.list_state.selected() {
            let selected_folder = &self.folders[selected_index];
            let full_path = self.desktop_path.join(selected_folder);
            self.result = Some(full_path);
        }
        self.running = false;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    //  --- MANUAL TUI SETUP on STDERR ---
    enable_raw_mode()?;
    execute!(stderr(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stderr());
    let mut terminal = Terminal::new(backend)?;
    //  --- END MANUAL SETUP ---

    let mut app = App::new()?;
    let run_result = app.run(&mut terminal).await;

    //  --- MANUAL TUI RESTORE from STDERR ---
    disable_raw_mode()?;
    execute!(stderr(), LeaveAlternateScreen)?;
    //  --- END MANUAL RESTORE ---

    if let Err(e) = run_result {
        eprintln!("Application error: {}", e);
    } else if let Some(folder_path) = app.result {
        println!("{}", folder_path.display());
    }

    // If run_result was Ok(()) and app.result is None (user pressed 'Esc'), we print nothing
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

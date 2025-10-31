use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
// ‼️ Need to import these for manual setup
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use futures::StreamExt;
use ratatui::{
    Frame,
    Terminal, // ‼️ Use the full Terminal struct
    // ‼️ Import the backend and Terminal
    backend::CrosstermBackend,
    prelude::*,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};
use std::env;
// ‼️ We need stderr
use std::io::stderr;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // ‼️ --- MANUAL TUI SETUP on STDERR ---
    enable_raw_mode()?;
    // ‼️ Use stderr() here
    execute!(stderr(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stderr());
    let mut terminal = Terminal::new(backend)?;
    // ‼️ --- END MANUAL SETUP ---

    // ‼️ Run result is now handled to get the selected path
    // ‼️ We pass our manually created terminal
    let run_result = run(&mut terminal).await;

    // ‼️ --- MANUAL TUI RESTORE from STDERR ---
    disable_raw_mode()?;
    // ‼️ Use stderr() here
    execute!(stderr(), LeaveAlternateScreen)?;
    // ‼️ --- END MANUAL RESTORE ---

    // ‼️ This part is PERFECT. It prints the path to STDOUT,
    // ‼️ which the fish script will capture.
    if let Ok(Some(folder_path)) = run_result {
        println!("{}", folder_path.display());
    } else if let Err(e) = run_result {
        // ‼️ Errors go to STDERR, so the user sees them
        eprintln!("Application error: {}", e);
    }
    // ‼️ If run_result was Ok(None) (user pressed 'q'), we print nothing

    Ok(())
}

// ‼️ Changed signature to accept the explicit Terminal type
async fn run(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stderr>>,
) -> Result<Option<PathBuf>> {
    // ‼️ Get the path to ~/Desktop
    let home_dir =
        env::var("HOME").map_err(|_| color_eyre::eyre::eyre!("Could not find HOME env var"))?;
    let desktop_path = PathBuf::from(home_dir).join("Desktop");

    if !desktop_path.is_dir() {
        return Err(color_eyre::eyre::eyre!(
            "~/Desktop directory not found at: {}",
            desktop_path.display()
        ));
    }

    // ‼️ Read the directory and collect folder names
    let folders: Vec<String> = std::fs::read_dir(&desktop_path)?
        .filter_map(Result::ok) // Ignore entries we can't read
        .filter(|entry| entry.path().is_dir()) // ‼️ Only include directories
        .map(|entry| entry.file_name().into_string().unwrap_or_default())
        .filter(|s| !s.is_empty() && !s.starts_with('.')) // ‼️ Filter out empty or hidden
        .collect();

    if folders.is_empty() {
        return Err(color_eyre::eyre::eyre!("No folders found in ~/Desktop"));
    }

    // ‼️ State for tracking the selected item
    let mut list_state = ListState::default();
    list_state.select(Some(0)); // Select the first item

    let mut event_stream = event::EventStream::new();

    loop {
        // ‼️ Pass the folder list and selection state to render
        terminal.draw(|frame| render(frame, &folders, &mut list_state))?;

        // ‼️ Simplified the loop to only handle input events
        if let Some(Ok(event)) = event_stream.next().await {
            if let Event::Key(key) = event {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            break Ok(None);
                        }
                        KeyCode::Char('j') | KeyCode::Down => {
                            if let Some(selected) = list_state.selected() {
                                let next = (selected + 1) % folders.len();
                                list_state.select(Some(next));
                            }
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            if let Some(selected) = list_state.selected() {
                                let prev = if selected == 0 {
                                    folders.len() - 1
                                } else {
                                    selected - 1
                                };
                                list_state.select(Some(prev));
                            }
                        }
                        KeyCode::Enter => {
                            if let Some(selected_index) = list_state.selected() {
                                let selected_folder = &folders[selected_index];
                                let full_path = desktop_path.join(selected_folder);
                                break Ok(Some(full_path));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

// ‼️ This function remains identical
fn render(frame: &mut Frame, folders: &[String], list_state: &mut ListState) {
    let items: Vec<ListItem> = folders.iter().map(|f| ListItem::new(f.as_str())).collect();
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
    frame.render_stateful_widget(list, frame.area(), list_state);
}

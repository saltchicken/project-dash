use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use futures::StreamExt;
use ratatui::{
    DefaultTerminal,
    Frame,
    prelude::*,
    // ‼️ Added for selection highlighting
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState}, // ‼️ Added ListState
};

// ‼️ Added for path and env var handling
use std::env;
use std::path::PathBuf;

// ‼️ No longer need tokio::process or tokio::io
// use std::process::Stdio;
// use tokio::io::{AsyncBufReadExt, BufReader};
// use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    // ‼️ Run result is now handled to get the selected path
    let run_result = run(terminal).await;

    ratatui::restore();

    // ‼️ After restoring the terminal, check if a path was selected
    if let Ok(Some(folder_path)) = run_result {
        // ‼️ Use std::process::Command for a blocking call to take over the terminal
        std::process::Command::new("nvim")
            .arg(folder_path)
            .status() // Wait for nvim to exit
            .map_err(|e| color_eyre::eyre::eyre!("Failed to start nvim: {}", e))?;
    } else if let Err(e) = run_result {
        // ‼️ Print any errors that occurred during the run
        eprintln!("Application error: {}", e);
    }
    // ‼️ If run_result was Ok(None) (user pressed 'q'), we just exit cleanly

    Ok(())
}

// ‼️ Changed signature to return the selected path, if any
async fn run(mut terminal: DefaultTerminal) -> Result<Option<PathBuf>> {
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

    // ‼️ Replaced journalctl setup with just the event stream
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
                            // ‼️ Quit
                            break Ok(None);
                        }
                        KeyCode::Char('j') | KeyCode::Down => {
                            // ‼️ Select next
                            if let Some(selected) = list_state.selected() {
                                let next = (selected + 1) % folders.len();
                                list_state.select(Some(next));
                            }
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            // ‼️ Select previous
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
                            // ‼️ Select folder and exit
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

// ‼️ Changed signature to accept folder list and the ListState
fn render(frame: &mut Frame, folders: &[String], list_state: &mut ListState) {
    // ‼️ Create ListItems from the folder list
    let items: Vec<ListItem> = folders.iter().map(|f| ListItem::new(f.as_str())).collect();

    // ‼️ Create a stateful List widget
    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Select a Folder"),
        )
        .style(Style::default().fg(Color::White))
        // ‼️ Add highlighting for the selected item
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Gray)
                .fg(Color::Black),
        )
        .highlight_symbol(">> ");

    // ‼️ Render the *stateful* widget, passing our persistent state
    frame.render_stateful_widget(list, frame.area(), list_state);
}

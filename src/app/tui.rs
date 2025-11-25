use color_eyre::Result;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{self, Stderr};

/// Type alias for the terminal backend used in this app.
pub type Tui = Terminal<CrosstermBackend<Stderr>>;

/// Initialize the terminal: Enable raw mode, enter alternate screen.
/// ‼️ Configured to use Stderr so Stdout is available for piping results.
pub fn init() -> Result<Tui> {
    enable_raw_mode()?;
    execute!(io::stderr(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

/// Restore the terminal: Disable raw mode, leave alternate screen.
pub fn restore() -> Result<()> {
    disable_raw_mode()?;
    execute!(io::stderr(), LeaveAlternateScreen)?;
    Ok(())
}


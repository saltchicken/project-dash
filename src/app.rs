pub mod events;
pub mod fs;
pub mod state;
pub mod tui;
pub mod ui;


pub use state::App;

use color_eyre::Result;
use futures::StreamExt;


pub async fn run(app: &mut App, terminal: &mut tui::Tui) -> Result<()> {
    let mut event_stream = crossterm::event::EventStream::new();

    while app.running {

        terminal.draw(|frame| ui::render(app, frame))?;

        if let Some(Ok(event)) = event_stream.next().await {

            events::handle_event(app, event);
        }
    }
    Ok(())
}
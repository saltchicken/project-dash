mod app;
mod fs;
mod tui;
mod ui;

use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // 1. Initialize Terminal (Stderr)
    let mut terminal = tui::init()?;

    // 2. Run App

    let mut app_result = app::App::new();

    let mut final_result = None;


    // This allows `app_result` to still be valid for the error check at the end.
    if let Ok(ref mut app) = app_result {
        let _ = app.run(&mut terminal).await;

        final_result = app.result.clone();
    }

    // 3. Restore Terminal (Stderr)

    tui::restore()?;

    // 4. Handle Results (Stdout/Stderr)
    match app_result {
        Err(e) => eprintln!("Application error: {}", e),
        Ok(_) => {
            if let Some(path) = final_result {
                println!("{}", path.display());
            }
        }
    }

    Ok(())
}
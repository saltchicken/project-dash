mod app;

use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = app::tui::init()?;

    let mut app_result = app::App::new();

    let mut final_result = None;

    if let Ok(ref mut app) = app_result {

        let _ = app::run(app, &mut terminal).await;
        final_result = app.result.clone();
    }

    app::tui::restore()?;

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
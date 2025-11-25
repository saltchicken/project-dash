use color_eyre::Result;
use std::env;
use std::path::PathBuf;

/// Locates the Desktop directory and returns its path.
/// ‼️ Extracted logic from original App::new for separation of concerns
pub fn get_desktop_path() -> Result<PathBuf> {
    let home_dir =
        env::var("HOME").map_err(|_| color_eyre::eyre::eyre!("Could not find HOME env var"))?;
    let desktop_path = PathBuf::from(home_dir).join("Desktop");

    if !desktop_path.is_dir() {
        return Err(color_eyre::eyre::eyre!(
            "~/Desktop directory not found at: {}",
            desktop_path.display()
        ));
    }
    Ok(desktop_path)
}

/// Scans the given directory and returns a sorted list of folder names.
/// ‼️ Extracted logic to allow re-use or testing separate from UI
pub fn get_folders(path: &PathBuf) -> Result<Vec<String>> {
    let mut folders: Vec<String> = std::fs::read_dir(path)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.file_name().into_string().unwrap_or_default())
        .filter(|s| !s.is_empty() && !s.starts_with('.'))
        .collect();

    if folders.is_empty() {
        return Err(color_eyre::eyre::eyre!(
            "No folders found in {}",
            path.display()
        ));
    }

    folders.sort(); // ‼️ Added sorting for better UX
    Ok(folders)
}

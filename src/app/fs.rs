use color_eyre::Result;
use std::env;
use std::path::PathBuf;

/// Locates the Projects directory and returns its path.
pub fn get_projects_path() -> Result<PathBuf> {
    let home_dir =
        env::var("HOME").map_err(|_| color_eyre::eyre::eyre!("Could not find HOME env var"))?;

    let projects_path = PathBuf::from(home_dir).join("Projects");

    if !projects_path.is_dir() {
        return Err(color_eyre::eyre::eyre!(
            "~/Projects directory not found at: {}",
            projects_path.display()
        ));
    }
    Ok(projects_path)
}

/// Scans the given directory and returns a sorted list of folder names.
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

    folders.sort();
    Ok(folders)
}


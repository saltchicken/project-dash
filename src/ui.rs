use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
};

/// Main render loop.
pub fn render(app: &mut App, frame: &mut Frame) {
    let area = centered_rect(60, 50, frame.area());

    let items: Vec<ListItem> = app
        .filtered_folders
        .iter()
        .map(|f| ListItem::new(f.as_str()))
        .collect();

    // ‼️ Styles extracted to make modifying the theme easier
    let highlight_style = Style::default()
        .add_modifier(Modifier::BOLD)
        .bg(Color::Gray)
        .fg(Color::Black);

    let list_block = Block::default()
        .borders(Borders::ALL)
        .title(format!("Select a Folder (Filter: {})", app.input_text));

    let list = List::new(items)
        .block(list_block)
        .style(Style::default().fg(Color::White))
        .highlight_style(highlight_style)
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, area, &mut app.list_state);
}

/// Helper to center a rect in the screen.
/// ‼️ Extracted utility function
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

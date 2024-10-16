use std::io;
use std::time::Duration;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use ratatui::{
    backend::Backend,
    layout::{Layout, Constraint, Direction},
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Terminal,
    prelude::*
};
use crate::app::App;
use crate::ports::ClientPortInfo;

/// Render the Info tab for the selected port, with individual fields
fn render_info_tab(f: &mut Frame, port: &ClientPortInfo, area: ratatui::layout::Rect) {
    // Create a vertical layout for individual fields
    let info_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ]
                .as_ref(),
        )
        .split(area);

    // Render Client Name
    let client_name_paragraph = Paragraph::new(Span::raw(format!("Client Name: {}", port.client_name)))
        .block(Block::default().borders(Borders::ALL).title("Client Name"));
    f.render_widget(client_name_paragraph, info_chunks[0]);

    // Render Client ID
    let client_id_paragraph = Paragraph::new(Span::raw(format!("Client ID: {}", port.client_id)))
        .block(Block::default().borders(Borders::ALL).title("Client ID"));
    f.render_widget(client_id_paragraph, info_chunks[1]);

    // Render Port Name
    let port_name_paragraph = Paragraph::new(Span::raw(format!("Port Name: {}", port.port_name)))
        .block(Block::default().borders(Borders::ALL).title("Port Name"));
    f.render_widget(port_name_paragraph, info_chunks[2]);

    // Render Port ID
    let port_id_paragraph = Paragraph::new(Span::raw(format!("Port ID: {}", port.port_id)))
        .block(Block::default().borders(Borders::ALL).title("Port ID"));
    f.render_widget(port_id_paragraph, info_chunks[3]);

    // Render Port Capabilities
    let port_cap_paragraph = Paragraph::new(Span::raw(format!("Capabilities: {:?}", port.port_cap)))
        .block(Block::default().borders(Borders::ALL).title("Capabilities"));
    f.render_widget(port_cap_paragraph, info_chunks[4]);
}

/// The main function to run the TUI
pub fn run_tui<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            // Layout: two vertical chunks
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(f.area());

            // Left Pane: List of Ports
            let port_items: Vec<ListItem> = app
                .ports
                .iter()
                .map(|port| ListItem::new(format!("{}: {}", port.client_name, port.port_name)))
                .collect();
            let port_list = List::new(port_items)
                .block(Block::default().borders(Borders::ALL).title("Ports"))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">> ");

            f.render_stateful_widget(port_list, chunks[0], &mut app.state);

            // Right Pane: Tabs (Info and Connect)
            let tabs = vec!["Info", "Connect"]
                .iter()
                .map(|t| Span::raw(*t))
                .collect::<Vec<_>>();
            let tabs_widget = Tabs::new(tabs)
                .block(Block::default().borders(Borders::ALL).title("Tabs"))
                .select(app.active_tab)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD));

            f.render_widget(tabs_widget, chunks[1]);

            // Display the selected tab content
            let right_pane_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)])
                .split(chunks[1]);

            match app.active_tab {
                0 => {
                    // Info Tab: Display selected port information with individual fields
                    if let Some(port) = app.selected_port_info() {
                        render_info_tab(f, port, right_pane_chunks[0]);
                    }
                }
                1 => {
                    // Connect Tab: Empty for now
                    let block = Block::default().borders(Borders::ALL).title("Connect");
                    f.render_widget(block, right_pane_chunks[0]);
                }
                _ => {}
            }
        })?;

        // Handle input events
        if crossterm::event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()), // Quit the TUI
                    KeyCode::Down => {
                        let i = match app.state.selected() {
                            Some(i) => {
                                if i >= app.ports.len() - 1 {
                                    0 // Wrap around to the top
                                } else {
                                    i + 1
                                }
                            }
                            None => 0,
                        };
                        app.state.select(Some(i));
                    }
                    KeyCode::Up => {
                        let i = match app.state.selected() {
                            Some(i) => {
                                if i == 0 {
                                    app.ports.len() - 1 // Wrap around to the bottom
                                } else {
                                    i - 1
                                }
                            }
                            None => 0,
                        };
                        app.state.select(Some(i));
                    }
                    KeyCode::Left => {
                        app.active_tab = 0; // Switch to Info tab
                    }
                    KeyCode::Right => {
                        app.active_tab = 1; // Switch to Connect tab
                    }
                    _ => {}
                }
            }
        }
    }
}

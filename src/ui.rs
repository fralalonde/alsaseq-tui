use crate::app::App;
use crate::ports::{list_addr, vec_ports, ClientPortInfo};
use alsa::Seq;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use ratatui::{
    backend::Backend,
    layout::Constraint::Percentage,
    layout::{Direction, Layout, Rect},
    prelude::*,
    style::{Style, Stylize},
    text::Line,
    text::{Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame, Terminal,
};

use alsa::seq::{Addr, PortSubscribeIter, QuerySubsType};
use std::io;
use std::time::Duration;

/// Render the upper part of the Info tab: port address, client name, port name, and caps
fn render_info_upper(f: &mut Frame, port: &ClientPortInfo, area: Rect) {
    let lines = vec![
        Line::from(vec![
            "Address ".red().bold(),
            Span::raw(format!("{}:{}", port.client_id, port.port_id)),
        ]),
        Line::from(vec![
            "Client ".red().bold(),
            Span::raw(port.client_name.to_string()),
        ]),
        Line::from(vec![
            "Port ".red().bold(),
            Span::raw(port.port_name.to_string()),
        ]),
        Line::from(vec![
            "Capabilities ".red().bold(),
            Span::raw(format!("{:?}", port.port_cap)),
        ]),
    ];

    let paragraph = Paragraph::new(Text::from(lines))
        .block(Block::default().borders(Borders::ALL).title("Port Info"));
    f.render_widget(paragraph, area);
}

fn render_info_lower(f: &mut ratatui::Frame, seq: &Seq, port: &ClientPortInfo, area: Rect) {
    // Retrieve all ports and their addresses
    let all_ports = list_addr(seq);

    let addr = Addr {
        client: port.client_id,
        port: port.port_id,
    };

    // Create an iterator to get the list of subscriptions (connected ports)
    let connected_ports: Vec<String> = PortSubscribeIter::new(seq, addr, QuerySubsType::READ)
        .filter_map(|sub| {
            // Look up the connected port's name using the address in the all_ports HashMap
            all_ports.get(&sub.get_sender()).cloned() // Return the port name if found
        })
        .collect();

    // Convert each connected port name into a ListItem for display
    let connection_items: Vec<ListItem> = connected_ports.into_iter().map(ListItem::new).collect();

    // Create a List widget with the connected ports
    let connections_list = List::new(connection_items)
        .block(Block::default().borders(Borders::ALL).title("Connections"))
        .highlight_symbol(">> ");

    f.render_widget(connections_list, area);
}

/// A placeholder for the actual logic that determines if a port is connected to another port
fn is_connected_to(selected: &ClientPortInfo, port: &ClientPortInfo) -> bool {
    // Implement your logic here to check if `potential_connection` is connected to `port`
    // For now, just return true for demo purposes
    true
}

/// The main function to run the TUI
pub fn run_tui<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Percentage(30), Percentage(70)])
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

            // Split the right pane into upper and lower chunks
            let right_pane_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Percentage(50), Percentage(50)])
                .split(chunks[1]);

            match app.active_tab {
                0 => {
                    // Info Tab: Display selected port information with individual fields
                    if let Some(port) = app.selected_port_info() {
                        render_info_upper(f, port, right_pane_chunks[0]);
                        render_info_lower(f, &app.seq, port, right_pane_chunks[1]);
                    }
                }
                1 => {
                    let block = Block::default().borders(Borders::ALL).title("Connect");
                    f.render_widget(block, right_pane_chunks[0]);
                }
                _ => {}
            }
        })?;

        // Handle input events
        if event::poll(Duration::from_millis(250))? {
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

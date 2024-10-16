use std::error::Error;
use std::io;
use std::io::stdout;
use clap::{Parser, Subcommand, crate_name};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::prelude::*;
use crate::app::App;

mod service;
mod ports;
mod config;
mod ui;
mod app;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List the current MIDI ports
    List,
    /// Connect the MIDI devices based on the config file
    Connect {
        #[arg(long, value_name = "FILE")]
        config: Option<String>,
    },
    /// Install the MIDI service for automatic routing
    Install,
    /// Launch the Text User Interface (TUI)
    Tui,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the CLI arguments
    let cli = Cli::parse();

    let app_name = crate_name!();
    let seq = ports::initialize_seq(app_name).unwrap();

    match &cli.command {
        Some(Commands::List) =>     {
            let ports = ports::list_ports(&seq);
            for (name, addr) in &ports {
                println!("{} [{}:{}]", name, addr.client, addr.port);
            }
        }
        Some(Commands::Connect { config }) => {
            let config_file = config.as_deref().unwrap_or("autoroute.conf");
            let config = config::load_config(config_file);
            let ports = ports::list_ports(&seq);
            ports::connect_ports(&seq, &config, &ports);
        }
        Some(Commands::Install) => {
            service::install_service(app_name);
        }
        Some(Commands::Tui) | None => {
            let mut app = App::new();  // Initialize App, which now owns the Seq instance

            let backend = CrosstermBackend::new(stdout());
            let mut terminal = Terminal::new(backend)?;

            execute!(io::stdout(), EnterAlternateScreen)?;

            ui::run_tui(&mut terminal, &mut app)?;

            execute!(io::stdout(), LeaveAlternateScreen)?;
        }
    }
    Ok(())
}

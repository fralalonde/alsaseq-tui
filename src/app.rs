use alsa::seq::Seq;
use ratatui::widgets::ListState;
use crate::ports::{ClientPortInfo, vec_ports};

/// The `App` struct now holds the `Seq` instance as well as the current state.
pub struct App {
    pub seq: Seq,               // Holds the ALSA sequencer instance
    pub ports: Vec<ClientPortInfo>,  // Holds port data
    pub state: ListState,       // ListState to track selected port
    pub active_tab: usize,      // Tracks the active tab (Info or Connect)
}

impl App {
    /// Create a new `App` with the `Seq` instance and initialize state
    pub fn new() -> Self {
        let seq = Seq::open(None, None, false).unwrap();  // Open the ALSA sequencer
        let ports = vec_ports(&seq);                      // Retrieve ports using the Seq instance

        let mut state = ListState::default();
        state.select(Some(0));  // Start with the first port selected

        App {
            seq,
            ports,
            state,      
            active_tab: 0,      // Start with the Info tab selected
        }
    }

    /// Get the currently selected port's information
    pub fn selected_port_info(&self) -> Option<&ClientPortInfo> {
        self.state.selected().and_then(|i| self.ports.get(i))
    }

    /// Refresh the list of ports, if necessary
    pub fn refresh_ports(&mut self) {
        self.ports = vec_ports(&self.seq);  // Refresh port data from ALSA
    }
}

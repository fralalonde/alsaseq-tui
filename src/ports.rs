use alsa::seq::{Addr, ClientIter, PortIter, Seq, PortSubscribe, PortCap};
use std::collections::HashMap;
use std::ffi::CString;

pub fn initialize_seq(client_name: &str) -> Result<Seq, Box<dyn std::error::Error>> {
    let seq = Seq::open(None, None, false)?;
    let client_name = CString::new(client_name).expect("Invalid CString");
    seq.set_client_name(&client_name).unwrap();
    Ok(seq)
}

pub fn list_ports(seq: &Seq) -> HashMap<String, Addr> {
    let mut ports = HashMap::new();

    // Iterate over clients and their ports
    for client in ClientIter::new(seq) {
        for port in PortIter::new(seq, client.get_client()) {
            let client_name = client.get_name().unwrap_or("Unknown Client");
            let port_name = port.get_name().unwrap_or("Unknown Port");
            let addr = Addr {
                client: client.get_client(),
                port: port.get_port(),
            };

            // Display client and port info with constant length for the [client:port] part
            println!(
                "[{:>3}:{:<3}] {}",  // Right-align client and left-align port
                addr.client, addr.port, format!("{}:{}", client_name, port_name)
            );

            ports.insert(format!("{}:{}", client_name, port_name), addr);
        }
    }

    ports
}

pub fn list_addr(seq: &Seq) -> HashMap<Addr, String> {
    let mut ports = HashMap::new();

    // Iterate over clients and their ports
    for client in ClientIter::new(seq) {
        for port in PortIter::new(seq, client.get_client()) {
            let client_name = client.get_name().unwrap_or("Unknown Client");
            let port_name = port.get_name().unwrap_or("Unknown Port");
            let addr = Addr {
                client: client.get_client(),
                port: port.get_port(),
            };
            ports.insert(addr, format!("{}:{}", client_name, port_name));
        }
    }

    ports
}

pub(crate) struct ClientPortInfo {
    pub client_name: String,
    pub client_id: i32,
    pub port_name: String,
    pub port_id: i32,
    pub port_cap: PortCap,
}

pub fn vec_ports(seq: &Seq) -> Vec<ClientPortInfo> {
    let mut ports = Vec::new();

    // Iterate over clients and their ports
    for client in ClientIter::new(seq) {
        for port in PortIter::new(seq, client.get_client()) {
            ports.push(ClientPortInfo {
                client_name: client.get_name().unwrap_or("Unknown Client").to_string(),
                client_id: client.get_client(),
                port_name: port.get_name().unwrap_or("Unknown Port").to_string(),
                port_id: port.get_port(),
                port_cap: port.get_capability(),
            });
        }
    }

    ports
}


pub fn connect_ports(seq: &Seq, config: &crate::config::Config, ports: &HashMap<String, Addr>) {
    for (source, dests) in &config.connections {
        if let Some(source_addr) = ports.get(source) {
            for dest in dests {
                if let Some(dest_addr) = ports.get(dest) {
                    println!(
                        "Connecting {} [{}:{}] to {} [{}:{}]",
                        source, source_addr.client, source_addr.port,
                        dest, dest_addr.client, dest_addr.port
                    );
                    let subs = PortSubscribe::empty().unwrap();
                    seq.subscribe_port(&subs).unwrap_or_else(|e| {
                        println!("Failed to connect {} to {}: {}", source, dest, e);
                    });
                } else {
                    println!("Device '{}' not present, ignored", dest);
                }
            }
        } else {
            println!("Device '{}' not present, ignored", source);
        }
    }
}

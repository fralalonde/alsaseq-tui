use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
pub struct Config {
    pub ignore: Vec<String>,
    pub connections: HashMap<String, Vec<String>>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            ignore: Vec::new(),
            connections: HashMap::new(),
        }
    }
}

pub fn load_config(config_file: &str) -> Config {
    let mut config = Config::new();

    if let Ok(lines) = read_lines(config_file) {
        for line in lines {
            if let Ok(l) = line {
                if l.starts_with("ignore") {
                    let device = l.split_whitespace().nth(1).unwrap().to_string();
                    config.ignore.push(device);
                } else if l.starts_with("connect") {
                    let parts: Vec<&str> = l.split("->").collect();
                    let source = parts[0].trim().to_string();
                    let dest = parts[1].trim().to_string();
                    config.connections.entry(source).or_default().push(dest);
                }
            }
        }
    }

    config
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

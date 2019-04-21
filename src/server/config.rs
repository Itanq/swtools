use serde::Deserialize;

use std::path::PathBuf;
use std::net::Ipv4Addr;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::error::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub path: PathBuf,
    pub localhost: Ipv4Addr,
    pub port: usize,
}

pub fn read_config_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<Error>> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let config = serde_json::from_reader(reader)?;

    Ok(config)
}
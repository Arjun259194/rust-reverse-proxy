use std::{collections::HashMap, fs::File, io::BufReader, process};

use serde::Deserialize;

use super::record::Record;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: Server,
    pub records: HashMap<String, Record>,
}

impl Config {
    pub fn new(path: Option<&str>) -> Config {
        let path = match path {
            None => "config.yaml",
            Some(path) => path,
        };
        let file = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!(
                    "Can't open the config.yaml file\nCheck if the file exits or not\n\nIf file exists move it to the config directory or pass the path to config file as argument."
                );
                eprintln!("{err}");
                process::exit(1)
            }
        };
        let reader = BufReader::new(file);

        match serde_yaml::from_reader::<_, Config>(reader) {
            Err(err) => {
                eprintln!(
                    "Can't parse the config.yaml file\nCheck for syntax error and double check the content of config file"
                );
                eprintln!("{err}");
                process::exit(1);
            }
            Ok(config) => config,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: String,
    pub host: String,
    pub cors: Option<String>,
    pub logging: Option<LogginLevel>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum LogginLevel {
    INFO,
    TRACE,
    DEBUG,
}

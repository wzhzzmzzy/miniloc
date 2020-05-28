use crate::{
    core
};

pub struct Config {
    pub filename: String,
    pub ignore_list: Vec<String>,
    pub ignore_file: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> core::Result<Config> {
        let mut config = Config {
            filename: String::new(),
            ignore_file: String::new(),
            ignore_list: Vec::new()
        };

        let filename = args[1].clone();

        config.filename = filename;
        Ok(config)
    }
}

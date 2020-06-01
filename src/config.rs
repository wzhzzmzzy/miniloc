use crate::{
    core,
    error
};

pub struct Config {
    pub filename: String,
    pub ignore_list: Vec<String>,
    pub ignore_file: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> core::Result<Config> {
        args.next();

        let mut config = Config {
            filename: String::new(),
            ignore_file: String::new(),
            ignore_list: Vec::new()
        };

        let filename = args.next().ok_or(
            error::Error::NoneError(String::from("No filename in params"))
        )?;

        config.filename = filename;
        Ok(config)
    }
}

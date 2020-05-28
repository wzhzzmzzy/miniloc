use std::env;
use miniloc::{
    error as loc_error,
    core,
    config as loc_config
};


fn main() -> Result<(), loc_error::LocError>{
    let args: Vec<String> = env::args().collect();
    let config = loc_config::Config::new(&args)?;

    let lines = core::calc_line_number(config)?;

    println!("{}", lines);

    Ok(())
}


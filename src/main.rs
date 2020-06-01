use std::env;
use miniloc::{
    core,
    config
};

fn main() -> core::Result<()>{
    let config = config::Config::new(env::args())?;

    let ret = core::calc_line_number(config)?;

    for lines in ret.iter() {
        println!("\n{}:\n{}", lines.0, lines.1);
    }

    Ok(())
}


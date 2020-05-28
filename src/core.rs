use std::io::{
    Read
};
use std::fs;
use std::fmt;

use crate::{
    error,
    config,
    core
};

pub type Result<T> = std::result::Result<T, error::Error>;

pub struct LineStatistic {
    blank: u32,
    code: u32,
    comment: u32,
    files: u32
}

impl fmt::Display for LineStatistic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "blank: {}\ncomment: {}\ncode: {}", self.blank, self.comment, self.code)
    }
}

fn walk_dir(dirname: &String, ret: &mut LineStatistic) -> Result<u32> {
    let mut count: u32 = 0;
    for entry in fs::read_dir(dirname)? {
        let path = entry?.path();
        let raw_path = path.to_str().ok_or(error::Error::NoneError)?;
        if path.is_dir() {
            count += walk_dir(&raw_path.to_string(), ret)?;
        } else {
            read_file(&raw_path.to_string(), ret)?;
            count += 1;
        }
    }
    Ok(count)
}

fn read_file(filename: &String, ret: &mut LineStatistic) -> Result<()> {
    let mut contents = String::new();
    let mut file = fs::File::open(filename)?;
    file.read_to_string(&mut contents)?;
    let lines: Vec<&str> = contents.lines().collect();
    for line in lines.into_iter() {
        if line.len() == 0 {
            ret.blank += 1;
        } else if line.trim().starts_with("//") {
            ret.comment += 1;
        } else {
            ret.code += 1;
        }
    }
    Ok(())
}

pub fn calc_line_number(config: crate::config::Config) -> Result<LineStatistic> {
    let mut lines = LineStatistic {
        blank: 0,
        comment: 0,
        code: 0,
        files: 0
    };

    let metadata = fs::metadata(config.filename.clone())?;
    let file_type = metadata.file_type();

    if file_type.is_dir() {
        let file_count = walk_dir(&config.filename, &mut lines)?;
        println!("Read {} files", file_count);
    } else if file_type.is_file() {
        read_file(&config.filename, &mut lines)?;
    }

    Ok(lines)
}



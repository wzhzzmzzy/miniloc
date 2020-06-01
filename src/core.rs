use std::{
    io::Read,
    collections::HashMap,
    fs,
    fmt
};

use crate::{
    error,
    config
};

pub type Result<T> = std::result::Result<T, error::Error>;

pub struct LineStatistic {
    blank: u32,
    code: u32,
    comment: u32,
    files: u32,
}

impl LineStatistic {
    fn new() -> LineStatistic {
        LineStatistic {
            blank: 0,
            code: 0,
            comment: 0,
            files: 0,
        }
    }
}

impl fmt::Display for LineStatistic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Read {} files\nblank: {}\ncomment: {}\ncode: {}", self.files, self.blank, self.comment, self.code)
    }
}

fn get_language(path: &String) -> Result<String> {
    let mut sp = path.split(".");
    let mut suffix = String::new();
    loop {
        suffix = match sp.next() {
            Some(v) => v.to_string(),
            None => break
        }
    }
    let judge_suffix = |arr: Vec<&str>| {
        arr.contains(&suffix.as_str())
    };
    if judge_suffix(vec!["rs"]) {
        Ok(String::from("Rust"))
    } else if judge_suffix(vec!["py"]) {
        Ok(String::from("Python"))
    } else if judge_suffix(vec!["js", "jsx", "node"]) {
        Ok(String::from("JavaScript"))
    } else if judge_suffix(vec!["ts"]) {
        Ok(String::from("TypeScript"))
    } else {
        Ok(String::from("Plain Text"))
    }
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


fn walk_dir(dirname: &String, ret: &mut HashMap<String, LineStatistic>) -> Result<()> {

    for entry in fs::read_dir(dirname)? {
        println!("[logging] {}", dirname);
        let path = entry?.path();
        let raw_path = path.to_str().ok_or(error::Error::NoneError(String::from("Path is empty")))?;
        if path.is_dir() {
            walk_dir(&raw_path.to_string(), ret)?;
        } else {
            let lan = get_language(&raw_path.to_string())?;
            ret.entry(lan.clone()).or_insert(LineStatistic::new());
            let mut lines = ret.get_mut(&lan).ok_or(error::Error::NoneError(String::from("Something wrong")))?;
            read_file(&raw_path.to_string(), lines)?;
            lines.files += 1;
        }
    }
    Ok(())
}

pub fn calc_line_number(config: config::Config) -> Result<HashMap<String, LineStatistic>> {
    let mut ret: HashMap<String, LineStatistic> = HashMap::new();

    let metadata = fs::metadata(config.filename.clone())?;
    let file_type = metadata.file_type();

    if file_type.is_dir() {
        walk_dir(&config.filename, &mut ret)?;
    } else if file_type.is_file() {
        let lan = get_language(&config.filename)?;
        ret.entry(lan.clone()).or_insert(LineStatistic::new());
        let mut lines = ret.get_mut(&lan).ok_or(error::Error::NoneError(String::from("Something wrong")))?;
        read_file(&config.filename, lines)?;
        lines.files += 1;
    }

    Ok(ret)
}

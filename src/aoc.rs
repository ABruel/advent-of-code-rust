use std::{env::current_dir, fs, path::PathBuf};
pub fn read_trivial(day: u8) -> String {
    let path = PathBuf::from(format!(
        "{}\\src\\inputs\\{:0>2}-trivial.txt",
        current_dir().unwrap().display(),
        day
    ));
    fs::read_to_string(path).unwrap()
}

pub fn read(day: u8) -> String {
    let path = PathBuf::from(format!(
        "{}\\src\\inputs\\{:0>2}.txt",
        current_dir().unwrap().display(),
        day
    ));
    fs::read_to_string(path).unwrap()
}

pub fn parse_int(s: &str) -> i32 {
    s.parse().unwrap()
}

use std::fs::read_to_string;
use std::path::Path;

pub(crate) fn get_input(file: &'static str) -> String {
    read_to_string(format!(
        "./inputs/{}.txt",
        Path::new(file).file_stem().unwrap().to_str().unwrap()
    ))
    .unwrap()
}

mod day01;
mod day02;
mod day03;

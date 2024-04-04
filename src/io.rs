use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use colored::Colorize;

pub fn read_file_lines_into_vec(path_buf: &PathBuf) -> Vec<String> {
    let result = std::fs::read_to_string(path_buf);
    if result.is_err() {
        exit_on_io_error("could not read file");
    }
    let content = result.unwrap();
    let mut lines: Vec<String> = vec!();
    for line in content.lines() {
        lines.push(line.to_string());
    }
    return lines;
}

pub fn write_vec_to_file(path: &str, content: Vec<String>) {
    let result = File::create(path);
    if result.is_err() {
        exit_on_io_error("could not open file to write");
    }
    let mut file = BufWriter::new(result.unwrap());
    content.into_iter().for_each(|s| writeln!(file, "{}", s).expect("unable to write"));
    file.flush().unwrap();
}

fn exit_on_io_error(message: &str) {
    eprintln!("{}", message.red());
    std::process::exit(0);
}
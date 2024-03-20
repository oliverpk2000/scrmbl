use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn read_file_lines_into_vec(path_buf: &PathBuf) -> Vec<String> {
    let content = std::fs::read_to_string(path_buf).expect("could not read file");
    let mut lines: Vec<String> = vec!();
    for line in content.lines() {
        lines.push(line.to_string());
    }
    return lines;
}

pub fn write_vec_to_file(path: &str, content: Vec<String>) {
    let file = File::create(path).expect("unable to create file");
    let mut file = BufWriter::new(file);
    content.into_iter().for_each(|s| writeln!(file, "{}", s).expect("unable to write"));
    file.flush().unwrap();
}
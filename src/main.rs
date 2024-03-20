use std::fs::File;
use std::io::Write;
use std::io::BufWriter;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use magic_crypt::{MagicCryptTrait, new_magic_crypt};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encrypt { path: std::path::PathBuf, password: String },
    Decrypt { path: std::path::PathBuf, password: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encrypt { path, password } => {
            println!("'scrmbl encrypt' was used, password is: {:?}, path is: {:?}", password, path);
            let lines = read_file_lines_into_vec(path, password);
            let encrypted_lines = cypher_str_vector(lines, password);
            let str_path = path.clone().into_os_string().into_string().unwrap();
            write_vec_to_file(str_path.as_str(), encrypted_lines);
        }
        Commands::Decrypt { path, password } => {
            println!("'scrmbl decrypt' was used, password is: {:?}, path is: {:?}", password, path);
            let lines = read_file_lines_into_vec(path, password);
            let decrypted_lines = decypher_str_vector(lines, password);
            let str_path = path.clone().into_os_string().into_string().unwrap();
            write_vec_to_file(str_path.as_str(), decrypted_lines);
        }
    }
}

fn read_file_lines_into_vec(path_buf: &PathBuf, password: &str) -> Vec<String> {
    let content = std::fs::read_to_string(path_buf).expect("could not read file");
    let mut lines: Vec<String> = vec!();
    for line in content.lines() {
        lines.push(line.to_string());
    }

    return lines;
}

fn cypher_str_vector(str_vec: Vec<String>, password: &str) -> Vec<String> {
    let encrypted_vec: Vec<_> = str_vec.into_iter().map(|s| encrypt_str(s.as_str(), password)).collect();
    return encrypted_vec;
}

fn decypher_str_vector(str_vec: Vec<String>, password: &str) -> Vec<String> {
    let decrypted_vec: Vec<_> = str_vec.into_iter().map(|s| decrypt_str(s.as_str(), password)).collect();
    return decrypted_vec;
}

fn encrypt_str(string: &str, password: &str) -> String {
    let key = new_magic_crypt!(password, 256);
    return key.encrypt_str_to_base64(string);
}

fn decrypt_str(string: &str, password: &str) -> String {
    let key = new_magic_crypt!(password, 256);
    return key.decrypt_base64_to_string(string).unwrap_or("".into());
}

fn write_vec_to_file(path: &str, content: Vec<String>) {
    let file = File::create(path).expect("unable to create file");
    let mut file = BufWriter::new(file);
    content.into_iter().for_each(|s| write!(file, "{}", s).expect("unable to write"));
    file.flush().unwrap();
}
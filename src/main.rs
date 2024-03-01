use std::path::PathBuf;
use bcrypt::DEFAULT_COST;
use clap::{Parser, Subcommand};
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
            read_file_lines_into_vec(path, password);
        }
        Commands::Decrypt { path, password } => {
            println!("'scrmbl decrypt' was used, password is: {:?}, path is: {:?}", password, path)
        }
    }
}

fn read_file_lines_into_vec(path_buf: &PathBuf, password:&str){
    let content = std::fs::read_to_string(path_buf).expect("could not read file");
    let mut lines: Vec<&str> = vec!();
    for line in content.lines() {
        lines.push(line);
    }

    println!("{:?}", cypher_str_vector(lines, password));
}

fn cypher_str_vector(str_vec:Vec<&str>, password: &str) -> Vec<String> {
    let encrypted_vec: Vec<_> = str_vec.into_iter().map(|s| encrypt_str(s, password)).collect();
    return encrypted_vec;
}

fn encrypt_str(string:&str, password:&str) ->String{
    let mut hashed_password: String = bcrypt::hash(password, DEFAULT_COST).unwrap();
    let password_bytes = hashed_password.as_bytes();
    let line_bytes = string.as_bytes();
    return "".to_string();
}
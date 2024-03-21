mod encryption;
mod io;


use clap::{Parser, Subcommand};
pub use magic_crypt::{MagicCryptTrait};

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
            let lines = io::read_file_lines_into_vec(path);
            let encrypted_lines = encryption::encode_str_vector(lines, password);
            let str_path = path.clone().into_os_string().into_string().unwrap();
            io::write_vec_to_file(str_path.as_str(), encrypted_lines);
        }
        Commands::Decrypt { path, password } => {
            let lines = io::read_file_lines_into_vec(path);
            let decrypted_lines = encryption::decode_str_vector(lines, password);
            let str_path = path.clone().into_os_string().into_string().unwrap();
            io::write_vec_to_file(str_path.as_str(), decrypted_lines);
        }
    }
}
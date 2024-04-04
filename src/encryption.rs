use colored::Colorize;
use magic_crypt::{MagicCryptTrait, new_magic_crypt};

pub fn encode_str_vector(str_vec: Vec<String>, password: &str) -> Vec<String> {
    let encrypted_vec: Vec<_> = str_vec.into_iter().map(|s| encrypt_str(s.as_str(), password)).collect();
    return encrypted_vec;
}

pub fn decode_str_vector(str_vec: Vec<String>, password: &str) -> Vec<String> {
    let decrypted_vec: Vec<_> = str_vec.into_iter().map(|s| decrypt_str(s.as_str(), password)).collect();
    return decrypted_vec;
}

fn encrypt_str(string: &str, password: &str) -> String {
    let key = new_magic_crypt!(password, 256);
    return key.encrypt_str_to_base64(string);
}

fn decrypt_str(string: &str, password: &str) -> String {
    let key = new_magic_crypt!(password, 256);
    let result = key.decrypt_base64_to_string(string);
    return match result {
        Ok(v) => v,
        Err(_) => {
            eprintln!("{}", "invalid password for line -> line not decrypted".red());
            string.to_string()
        }
    }
}
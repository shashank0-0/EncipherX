use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::{Aead, OsRng, generic_array::GenericArray};
use base64::{engine::general_purpose, Engine};
use clap::Parser;
use rand::RngCore;
use rpassword::read_password;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::PathBuf;

/// A simple cross-platform text encryption CLI tool
#[derive(Parser, Debug)]
#[command(
    name = "EncipherX",
    version = "1.0",
    author = "Shashank",
    about = "Encrypt and decrypt text using AES-256-GCM"
)]
struct Args {
    /// Show help information
    #[arg(short, long)]
    help: bool,
}

fn print_banner() {
    let banner = r#"
$$$$$$$$\                     $$\           $$\                          $$\   $$\ 
$$  _____|                    \__|          $$ |                         $$ |  $$ |
$$ |      $$$$$$$\   $$$$$$$\ $$\  $$$$$$\  $$$$$$$\   $$$$$$\   $$$$$$\ \$$\ $$  |
$$$$$\    $$  __$$\ $$  _____|$$ |$$  __$$\ $$  __$$\ $$  __$$\ $$  __$$\ \$$$$  / 
$$  __|   $$ |  $$ |$$ /      $$ |$$ /  $$ |$$ |  $$ |$$$$$$$$ |$$ |  \__|$$  $$<  
$$ |      $$ |  $$ |$$ |      $$ |$$ |  $$ |$$ |  $$ |$$   ____|$$ |     $$  /\$$\ 
$$$$$$$$\ $$ |  $$ |\$$$$$$$\ $$ |$$$$$$$  |$$ |  $$ |\$$$$$$$\ $$ |     $$ /  $$ |
\________|\__|  \__| \_______|\__|$$  ____/ \__|  \__| \_______|\__|     \__|  \__|
                                  $$ |                                             
                                  $$ |                                             
                                  \__|                                             
    "#;
    println!("{banner}");
}

fn get_encrypted_file_path() -> PathBuf {
    std::env::current_dir().unwrap().join("encrypted.txt")
}

fn save_to_file(data: &str) -> io::Result<()> {
    let mut file = File::create(get_encrypted_file_path())?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

fn read_from_file() -> io::Result<String> {
    let mut file = OpenOptions::new().read(true).open(get_encrypted_file_path())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn derive_key(password: &str) -> GenericArray<u8, typenum::U32> {
    let mut key = [0u8; 32];
    let bytes = password.as_bytes();
    let len = bytes.len().min(32);
    key[..len].copy_from_slice(&bytes[..len]);
    GenericArray::clone_from_slice(&key)
}

fn encrypt(plaintext: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let key = derive_key(password);
    let cipher = Aes256Gcm::new(&key);

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes())?;
    let mut combined = nonce_bytes.to_vec();
    combined.extend_from_slice(&ciphertext);

    Ok(general_purpose::STANDARD.encode(combined))
}

fn decrypt(encrypted_text: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let encrypted_bytes = general_purpose::STANDARD.decode(encrypted_text)?;
    if encrypted_bytes.len() < 12 {
        return Err("Invalid data".into());
    }

    let nonce = Nonce::from_slice(&encrypted_bytes[..12]);
    let ciphertext = &encrypted_bytes[12..];
    let key = derive_key(password);
    let cipher = Aes256Gcm::new(&key);

    let decrypted = cipher.decrypt(nonce, ciphertext)?;
    Ok(String::from_utf8(decrypted)?)
}

fn main() {
    let args = Args::parse();

    if args.help {
        println!(
            "
EncipherX (v1.0)
---------------------

USAGE:
    ./EncipherX

OPTIONS:
    -h, --help      Show this help message

After launching the app, you'll be prompted to:
  [1] Encrypt a message
  [2] Decrypt a message
  [3] Exit

Encrypted output will be saved in 'encrypted.txt'
            "
        );
        return;
    }

    print_banner();

    loop {
        println!("\nChoose an option:");
        println!("1. Encrypt text");
        println!("2. Decrypt text");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => {
                println!("Enter text to encrypt:");
                let mut plaintext = String::new();
                io::stdin().read_line(&mut plaintext).expect("Failed to read input");

                println!("Enter a password:");
                let password = read_password().expect("Failed to read password").trim().to_string();

                match encrypt(plaintext.trim(), &password) {
                    Ok(encrypted) => {
                        println!("\nEncrypted text:\n{}", encrypted);
                        save_to_file(&encrypted).expect("Failed to save file");
                        println!("Saved to 'encrypted.txt'");
                    }
                    Err(e) => println!("Encryption error: {}", e),
                }
            }
            "2" => {
                println!("Enter the encrypted text:");
                let mut encrypted_text = String::new();
                io::stdin().read_line(&mut encrypted_text).expect("Failed to read input");

                println!("Enter a password:");
                let password = read_password().expect("Failed to read password").trim().to_string();

                match decrypt(encrypted_text.trim(), &password) {
                    Ok(decrypted) => println!("\nDecrypted text:\n{}", decrypted),
                    Err(e) => println!("Decryption error: {}", e),
                }
            }
            "3" => break,
            _ => println!("Invalid option, try again."),
        }
    }
}

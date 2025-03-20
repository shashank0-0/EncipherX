use aes_gcm::{Aes256Gcm, Key, Nonce, KeyInit}; // Import KeyInit for `.new()`
use aes_gcm::aead::{Aead};
use rand::Rng;
use base64::{engine::general_purpose, Engine};
use rpassword::read_password;
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::path::PathBuf;
use generic_array::GenericArray;
use typenum::U32;

/// Get the correct file path based on the OS
fn get_encrypted_file_path() -> PathBuf {
    let mut path = std::env::current_dir().expect("Failed to get current directory");
    path.push("encrypted.txt"); // Works on both Windows and Linux
    path
}

/// Save encrypted text to a file
fn save_to_file(data: &str) -> io::Result<()> {
    let path = get_encrypted_file_path();
    let mut file = File::create(path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

/// Read encrypted text from a file
fn read_from_file() -> io::Result<String> {
    let path = get_encrypted_file_path();
    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Generate a 32-byte encryption key from a user-provided password
fn derive_key(password: &str) -> GenericArray<u8, U32> {
    let mut key = [0u8; 32];
    let bytes = password.as_bytes();
    let len = bytes.len().min(32);
    key[..len].copy_from_slice(&bytes[..len]);
    GenericArray::clone_from_slice(&key)
}

/// Encrypts a given text using AES-GCM
fn encrypt(plaintext: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let key = derive_key(password);
    let cipher = Aes256Gcm::new(&key);

    // Generate a random 12-byte nonce
    let mut rng = rand::thread_rng();
    let nonce_bytes: [u8; 12] = rng.gen(); // Fixed `gen()` issue
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes())
        .map_err(|_| "Encryption failed")?;

    // Combine nonce and ciphertext, then encode in base64
    let mut combined = nonce_bytes.to_vec();
    combined.extend_from_slice(&ciphertext);
    
    Ok(general_purpose::STANDARD.encode(combined))
}

/// Decrypts a given encrypted text using AES-GCM
fn decrypt(encrypted_text: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let key = derive_key(password);
    let cipher = Aes256Gcm::new(&key);

    let encrypted_bytes = general_purpose::STANDARD.decode(encrypted_text)?;
    
    if encrypted_bytes.len() < 12 {
        return Err("Invalid encrypted data".into());
    }

    let nonce = Nonce::from_slice(&encrypted_bytes[..12]);
    let ciphertext = &encrypted_bytes[12..];

    let decrypted_bytes = cipher.decrypt(nonce, ciphertext)
        .map_err(|_| "Decryption failed")?;

    Ok(String::from_utf8(decrypted_bytes)?)
}

fn main() {
    loop {
        println!("Choose an option:");
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
                        println!("Encrypted text: {}", encrypted);
                        save_to_file(&encrypted).expect("Failed to save file");
                        println!("Encrypted data saved to encrypted.txt");
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
                    Ok(decrypted) => println!("Decrypted text: {}", decrypted),
                    Err(e) => println!("Decryption error: {}", e),
                }
            }
            "3" => break,
            _ => println!("Invalid option, try again."),
        }
    }
}

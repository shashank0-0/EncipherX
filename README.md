# 🔐 Rust Text Encryptor

A **cross-platform text encryptor** built with **Rust and AES-GCM encryption**.  
Supports **Linux, Windows, and macOS**. Encrypt and decrypt text securely using a password.

---

## 🚀 Features
✅ AES-256-GCM encryption for **secure text encryption**  
✅ Password-based encryption & decryption  
✅ Saves encrypted text to a file (`encrypted.txt`)  
✅ **Cross-platform support** (Linux, Windows, macOS)  
✅ **Cross-compilation** (Build Windows `.exe` on Linux)  

---

## 🛠️ Installation & Setup (Windows, Linux, macOS)

### **🔹 1. Install Rust (Required for All OS)**  
Rust is required to build and run the program. Install it using the steps below:

#### **For Linux/macOS:**
Run the following command in the terminal:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

### **For Windows:**
1. Download the Rust Installer:

   * https://www.rust-lang.org/tools/install *

2. Run the Installer and choose "Default Installation"


3. Restart your terminal and verify installation by running:

rustc --version
cargo --version

If Rust is installed correctly, these commands will return version numbers.

### **🔹 2. Clone This Repository **

Run the following command in your terminal (Linux/macOS) or PowerShell (Windows):

git clone https://github.com/shashank0-0/text_encryptor.git
cd text_encryptor


---

### **🔹 3. Build the Project **

Run the following command to compile the program:

cargo build --release

This creates an optimized executable in target/release/.


---

### **🔹 4. Run the Program **

For Linux/macOS:

./target/release/text_encryptor

For Windows (CMD/PowerShell):

target\release\text_encryptor.exe


---

#  🔑 Usage

When you run the program, you'll see a menu like this:

Choose an option:
1. Encrypt text
2. Decrypt text
3. Exit

* // Encryption

1. Enter the text to encrypt


2. Enter a password


3. The encrypted text will be displayed and saved in encrypted.txt



Example:

Enter text to encrypt: Hello, World!
Enter a password: ******
Encrypted text: bXlxzFVsUV... (Base64 output)
Encrypted data saved to encrypted.txt

* // Decryption

1. Enter the encrypted text manually or from encrypted.txt


2. Enter the correct password


3. The decrypted text will be displayed



Example:

Enter a password: ******
Decrypted text: Hello, World!


---

# 📦 Cross-Compiling for Windows (From Linux)

If you want to build a Windows .exe file from Linux, install the Windows target:

rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu

The Windows .exe file will be generated at:

target/x86_64-pc-windows-gnu/release/text_encryptor.exe


---

📜 License

This project is MIT Licensed – feel free to use and modify it.


---

⭐ Support

If you found this useful, give it a ⭐ on GitHub!
For questions, contact me at GitHub Issues.

---


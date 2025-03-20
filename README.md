
# 🔐 Rust Text Encryptor

A **cross-platform text encryptor** built with **Rust and AES-GCM encryption**.  
Supports **Linux, Windows, and macOS**. Encrypt and decrypt text securely using a password.

---

## 🚀 Features
- ✅ AES-256-GCM encryption for **secure text encryption**  
- ✅ Password-based encryption & decryption  
- ✅ Saves encrypted text to a file (`encrypted.txt`)  
- ✅ **Cross-platform support** (Linux, Windows, macOS)  
- ✅ **Cross-compilation** (Build Windows `.exe` on Linux)  

---

## 🛠️ Installation & Setup (Windows, Linux, macOS)

### **1️⃣ Install Rust (Required for All OS)**  
Rust is required to build and run the program. Install it using the steps below:

#### **For Linux/macOS:**
Run the following command in the terminal:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

For Windows:

1. Download the Rust Installer

  * [Download Rust Installer](https://www.rust-lang.org/tools/install)

2. Run the Installer and choose "Default Installation"


3. Restart your terminal and verify installation by running:

rustc --version
cargo --version

If Rust is installed correctly, these commands will return version numbers.




---

2️⃣ Clone This Repository

Run the following command in your terminal (Linux/macOS) or PowerShell (Windows):

git clone https://github.com/shashank0-0/text_encryptor.git
cd text_encryptor


---

3️⃣ Build the Project

Run the following command to compile the program:

cargo build --release

This creates an optimized executable in target/release/.


---

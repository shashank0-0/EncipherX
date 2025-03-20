# 🔐 Text Encryptor

A simple and efficient text encryption and decryption tool written in Rust. Supports Windows, Linux, and macOS.

---

## ⚡ Installation & Setup (Windows, Linux, macOS)

### 1️⃣ Install Rust (Required for All OS)

Rust is required to build and run the program. Install it using the steps below:

#### For Linux/macOS:

Run the following command in the terminal:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### For Windows:

1. Download the Rust Installer: [Rust Installer](https://www.rust-lang.org/tools/install)  
2. Run the installer and choose **"Default Installation"**  
3. Restart your terminal and verify installation by running:

```sh
rustc --version
cargo --version
```

If Rust is installed correctly, these commands will return version numbers.

---

### 2️⃣ Clone This Repository

Run the following command in your terminal (Linux/macOS) or PowerShell (Windows):

```sh
git clone https://github.com/shashank0-0/text_encryptor.git
cd text_encryptor
```

---

### 3️⃣ Build the Project

Run the following command to compile the program:

```sh
cargo build --release
```

This creates an optimized executable in `target/release/`.

---

### 4️⃣ Run the Program

#### On Linux/macOS:

```sh
./target/release/text_encryptor
```

#### On Windows:

```sh
./target/release/text_encryptor.exe
```

---

## 📜 Usage

Once the program is running, follow the on-screen prompts:

1️⃣ **Encrypt a Message**  
   - Enter your text  
   - The program will generate an encrypted message  
   - The output will be saved in `encrypted.txt`  

2️⃣ **Decrypt a Message**  
   - Enter the previously encrypted text  
   - The original message will be displayed  

---

## 🚀 Contributing

Feel free to fork the repository and submit a pull request if you'd like to improve the project.

---

## 📜 License

This project is licensed under the **MIT License**.


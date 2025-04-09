###🔐 EncipherX

A simple and efficient text encryption and decryption tool written in Rust. Supports Windows, Linux and macOS.

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

---

### 2️⃣ Clone This Repository

Run the following command in your terminal (Linux/macOS) or PowerShell (Windows):

```sh
git clone https://github.com/shashank0-0/EncipherX.git
cd EncipherX
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
./target/release/EncipherX
```

---

#### For Windows:

### 🪟 Windows Installation (Using Prebuilt Executable)

1️⃣ Go to the [Releases](https://github.com/shashank0-0/EncipherX/releases) section of this repository.

2️⃣ Download the latest `EncipherX.exe` file from the assets.

3️⃣ Place the file in any directory you prefer (e.g., `C:\EncipherX\`).

4️⃣ Open Command Prompt (cmd) or PowerShell and navigate to the directory:

```powershell
cd C:\EncipherX\
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


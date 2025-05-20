# 🐱 catrs - A Rust Implementation of `cat`

`catrs` is a simple Rust-based command-line utility that mimics the basic functionality of the Unix `cat` command. It reads the contents of one or more text files and prints them to the terminal.

This project is part of my learning journey into Rust and low-level systems programming. Built with ❤️ for performance, readability, and simplicity.

---

## ✨ Features

- 🧾 Read and print contents of a single file
- 📂 Support for reading multiple files sequentially
- 🛑 Graceful error handling for missing or inaccessible files
- 💡 Clean and beginner-friendly Rust code

---

## 🛠️ Installation

### Option 1: Run from source (for developers)

```bash
git clone https://github.com/yourusername/catrs.git
cd catrs
cargo run -- file1.txt file2.txt
```

### Option 2: Build a release binary

```bash
cargo build --release
./target/release/catrs file1.txt file2.txt
```

- On Windows use .\target\release\catrs.exe or move it to a directory in your system path.

### Option 3: Install globally via Cargo

```bash
cargo install --git https://github.com/rajanchoksi79/catrs
```

- After installation, you can run catrs globally from anywhere in your terminal.

---

## Usage

```bash
catrs file.txt
```

```bash
catrs file1.txt file2.txt
```

- If one of the provided files doesn't exist, catrs will print an error message and continue with the next file.

---

## 📦 Example Output

```bash
--------------------------------
Start of the file - 1
--------------------------------
Hello, World!
This is a test.

--------------------------------
Start of the file - 2
--------------------------------
Second file here.
More content...
```

---

## 🧠 Why This Project?

- I'm learning systems programming and exploring low-level concepts through Rust. Rebuilding classic Unix tools is a great way to understand:

  - File I/O
  - Error handling
  - Working with command-line arguments
  - Structuring real CLI tools

---

## 📄 License

MIT — feel free to use.

---

## 👋 Author

- Rajan Choksi — Full Stack Developer diving into Rust & systems programming.

- 📢 Say hi on Twitter - https://x.com/Rajanchoksi_79 or check out more of my work on GitHub - https://github.com/rajanchoksi79/catrs

---

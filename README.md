<<<<<<< HEAD
# catrs
A Rust implementation of the classic Unix cat command.
=======
# 🐱 catrs

A simple, minimal clone of the classic Unix `cat` command — built in Rust.

`catrs` reads and prints the contents of one or more text files to standard output. This project is built for learning systems programming and CLI tool development in Rust.

---

## 📦 Features

- Read and display contents of one or more files
- Basic error handling for missing files
- Mimics standard `cat` behavior
- Written with clean, idiomatic Rust

---

## 🚀 Usage

### 🛠️ Build

```bash
cargo build --release
```

---

## Binary will be created at:

```bash
target/release/catrs
```

---

## Or run it directly with:

```bash
cargo run -- file1.txt file2.txt
```

---

## Example:

```bash
catrs notes.txt
```

```bash
catrs intro.txt chapter_one.txt
```

## To install locally:

```bash
cargo install --path .
```

- Make sure $HOME/.cargo/bin is in your system PATH so you can use catrs globally.

## License

- This project is open source under the MIT License.

## Author

- Rajan Choksi - @Rajanchoksi_79
>>>>>>> ac7d620 (second change in project)



# 🔍 MiniGrep in Rust

This project is a **functional replica** of the **`grep`** command-line utility from Linux. The primary goal is to apply concepts and learn how to build a CLI (Command Line Interface) application using the **Rust** programming language.

-----

## 📚 Study Reference

This `minigrep` follows the I/O project covered in the official Rust book.

You can follow along and attempt to replicate the code using the book:
[The Rust Programming Language: Chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

-----

## ✨ CLI Features

This updated version is prepared to be used as a real CLI, allowing the use of flags (options) just like the original `grep`.

  * **Supported Flags:**
      * **`-i` or `--ignore-case`:** Performs the search in a **case-insensitive** manner.
      * **`-h` or `--help`:** Displays the usage and help message.

> **Note:** You can check the code of the **unaltered version**, as presented in the book, in the [first commit](https://github.com/joaopabdala/minigrep/commit/a65f850db786963be73e56f2a9370b07ad196b4b) of this repository.

-----

## 🚀 How to Use and Install

### 1\. Pre-compiled Binary (Recommended)

You can download the binary from the **[release section](https://github.com/joaopabdala/minigrep/releases)** and install it to your system path.

Alternatively, execute the command directly by pointing to the binary:

```bash
./minigrep to poem.txt -i
```

### 2\. Installation via Cargo (For Rust Developers)

If you have Cargo installed, you can clone the repository and install it in your environment:

```bash
# Installs the executable into your Cargo binary PATH
cargo install --path .
```
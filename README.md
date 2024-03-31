# Netcat

![Static Badge](https://img.shields.io/badge/nc-1.0.0-blue)

Netcat is a computer networking utility for reading from and writing to network connections using TCP. See [this link](https://en.wikipedia.org/wiki/Netcat) to learn more about Netcat.

This is a cross-platform version written in Rust.

## Download and installation

### 1. Download the latest version

You can download the latest version of this tool in the "Releases" section of this Github repository.

### 2. Install the tool

This tool is just an executable, so you can place it anywhere on your computer and set a global environment variable to be able to use it globally.

## Build from source

### Prerequisites

Since this tool is written in Rust, you'll need the "rustup" toolchain in order to build this program from source. You can download it [here](https://www.rust-lang.org/tools/install).

### Build and install

Build the program with the following command:

```bash
cargo install --path .
```

> This command will build an optimized executable and install it so you'll be able to use it anywhere on your computer.

Enjoy!

# Netcat

![Static Badge](https://img.shields.io/badge/nc-1.0.0-blue)

Netcat is a computer networking utility for reading from and writing to network connections using TCP. [Learn more](https://en.wikipedia.org/wiki/Netcat).

This is a cross-platform version written in [Rust](https://rust-lang.org).

## Download and installation

### 1. Download the latest version

You can download the latest version of this tool in the "Releases" section of this Github repository. The ".exe" file is for Windows, the other one is for Unix systems.

### 2. Install the tool

This tool is just an executable, so you can place it anywhere on your computer and set a global environment variable to be able to use it globally.

## Build from source

### Prerequisites

Since this tool is written in Rust, you'll need the "rustup" toolchain in order to build this program from source. You can download it [here](https://www.rust-lang.org/tools/install).

### Download source code

Run the following command to clone this repository:

```bash
git clone https://github.com/eppeque/netcat
```

### Build and install

Build the program with the following command:

```bash
cargo install --path .
```

> This command will build an optimized executable and install it so you'll be able to use it anywhere on your computer.

## Usage

This tool allows you to connect to a TCP server. Here is how to use it in the CLI:

```bash
netcat 127.0.0.1 8080
```

The IPv4 address followed by the port number.

_Enjoy!_

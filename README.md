[![Rust](https://github.com/EncodePanda/crufty/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/EncodePanda/crufty/actions/workflows/rust.yml)

# Crufty

**Crufty** is a command-line tool that scans your projects for large build
artifacts and cleans them up safely. It supports many common languages and build
systems including Rust, Node.js, Python, Java, and more.

## Usage

```
> crufty help
A command-line tool that scans projects for large build artifacts and cleans them up

Usage: crufty <COMMAND>

Commands:
  scan  Scan for build artifacts in the current directory
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Usage example

### Basic Scan

```
> crufty scan

[+] Scanning: /home/encodepanda/Projects

[1] ./rust-api/target/            530 MB
[2] ./web-client/node_modules/    1.9 GB
[3] ./ml-notebook/.venv/          450 MB

Total: 2.88 GB in 3 directories
Use `crafty clean` to remove these safely.
```

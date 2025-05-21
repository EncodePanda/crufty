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
[+] Scanning: /Users/encodepanda/projects

[1] ./crufty/target                      419.6 MB
[2] ./too-many-lists/target              33.9 MB
[3] ./rust-web/target                    977.7 MB
[4] ./rshttp/target                      16.3 MB

Total size: 1.4 GB in 4 directories
Use `crufty clean` to remove these safely
```

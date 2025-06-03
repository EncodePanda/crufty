[![Rust](https://github.com/EncodePanda/crufty/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/EncodePanda/crufty/actions/workflows/rust.yml)

# Crufty

**Crufty** is a command-line tool that scans your projects for large build
artifacts and cleans them up safely. It supports many common languages and build
systems including Rust, Node.js, Python, Java, and more.

![crufty](crufty.gif)


## Usage

```
> crufty --help
A command-line tool that scans projects for large build artifacts and cleans them up safely

Usage: crufty <COMMAND>

Commands:
  scan   Scan for build artifacts in the current directory
  clean  Clean all build artifacts in the current directory
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Usage examples

### Basic Scan

```
> crufty scan
[+] Scanning: /Users/encodepanda/projects/ttemp

[1] ./crufty/target                      (Rust)   425.2 MB
[2] ./sandbox/99-scala-problems/target   (Scala)  252.9 KB
[3] ./rshttp/target                      (Rust)   16.3 MB
[4] ./tutorials/too-many-lists/target    (Rust)   33.9 MB
[5] ./tutorials/rust-web/target          (Rust)   977.7 MB

Total size: 1.4 GB in 5 directories
Use `crufty clean` to remove these safely
```

### Clean-up

```
> crufty clean

5 directories were removed, restoring 1.4 GB
```

## Install

Currently, we support the install process solely via `crates.io`

```
> cargo install crufty
```

# Crafty

**Crafty** is a command-line tool that scans your projects for large build
artifacts and cleans them up safely. It supports many common languages and build
systems including Rust, Node.js, Python, Java, and more.

## Usage example

### Basic Scan

```
> crafty scan

[+] Scanning: /home/encodepanda/Projects

[1] ./rust-api/target/            530 MB
[2] ./web-client/node_modules/    1.9 GB
[3] ./ml-notebook/.venv/          450 MB

Total: 2.88 GB in 3 directories
Use `crafty clean` to remove these safely.
```

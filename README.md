# Neko 🐱

**Neko** is a simple Rust clone of the `cat` command line utility. It reads files sequentially and outputs their contents to the standard output. This project is an exploration of Rust programming concepts, offering basic functionality similar to `cat`.

## Features & Usage

[ ] -A, --show-all equivalent to -vET
[ ] -b, --number-nonblank number nonempty output lines, overrides -n
[ ] -e equivalent to -vE
[ ] -E, --show-ends display $ at end of each line
[x] -n, --number number all output lines
[ ] -s, --squeeze-blank suppress repeated empty output lines
[ ] -t equivalent to -vT
[ ] -T, --show-tabs display TAB characters as ^I
[ ] -u (ignored)
[ ] -v, --show-nonprinting use ^ and M- notation, except for LFD and TAB
[x] --help display this help and exit
[ ] --version output version information and exit

## Installation

You can clone the repository and build `neko` locally using Cargo.

```bash
git clone https://github.com/mahesh-143/neko.git
cd neko
cargo build --release
```

The compiled binary will be located in the `target/release` directory.

````

## Example

```bash
./neko file.txt
# Outputs the contents of file.txt
````

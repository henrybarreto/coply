# Coply
The `Coply` is a simple tool to copy files.

## How use
Actually, it is just a Rust binary what receive two parameters, files paths, and do what it needs, but I will add some functionality basic, like a help and version command.

- If you pass a file and a folder It will brake
- If you pass a folder and a file it will brake
- If you pass a folder and another folder, it will brake
- If you pass a file and another file, it will work

You can compile and use the binary, or just use the cargo command to it works.

### Building binary
```sh
cargo build --release
cp target/release/coply /usr/bin
coply file_a.txt file_b.txt
```
Using run
```sh
cargo run file_a.txt file_b.txt
```


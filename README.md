# RUSTRIS

Rustris is a tile matching video game written in the rust programming language.

![Rustris screenshot](doc/screenshot01.png)

## Install prerequisites

Install wasm-pack:

```bash
cargo install wasm-pack
```

More infos here: [https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm]

## Build

Build with 'cargo build' in the top level directory:

```bash
cargo build
```

## Run command-line game

Run the built game by 'cargo run':

```bash
cargo run
```

## Build WASM 

```bash
cd rustris-wasm/
wasm-pack build --target web
python3 -m http.server --bind 127.0.0.1 --directory . 8080
# navigate your browser to http://127.0.0.1:8080/rustris.html
```

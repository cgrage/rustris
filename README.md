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

## Make it run in a browser

### Install three.js

```bash
cd rustris-wasm/
npm install --save three
```

### Build WASM to npm package

```bash
wasm-pack build --target web
```

### Serve the site

```bash
python3 -m http.server --bind 127.0.0.1 --directory . 8080
```

Now navigate your browser to http://127.0.0.1:8080/rustris.html

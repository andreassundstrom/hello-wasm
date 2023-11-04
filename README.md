# Rust WebAssembly

This is a basic demo of compiling rust into web-assembly and hosting it with vite. Showing how to:

- Manipulate the DOM from WASM
- Read/write from WASM using serialization
- Read directly from WASM:s linear memory

## To run the project

Build the WASM with wasm-pack (download it [here](https://rustwasm.github.io/wasm-pack/installer/)).

```
wasm-pack build
```

Intall npm-dependencies and start dev server

```
cd .\web-app
npm install
npm run dev
```

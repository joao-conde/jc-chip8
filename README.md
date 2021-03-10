# Chip-8 emulator

Chip-8 emulator written in Rust and compilable to WASM.

I first wrote it in TypeScript: [chip8-ts](https://github.com/joao-conde/chip8-emulator-ts). I decided to re-write one in Rust because I love the language and wanted to explore compiling an emulator to WASM since I want to use it for my NES emulator.

## Building

At `Cargo.toml` level, use the following to build and output to the demo directory using [wasm-pack](https://rustwasm.github.io/wasm-pack/):

```bash
wasm-pack build --release --target=web --no-typescript -d demo/
```

Then just serve the `demo` folder and open `index.html` in a browser.

## References

1. [Cowgod's Chip-8 Technical Reference v1.0](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
2. [Wikipedia](https://en.wikipedia.org/wiki/CHIP-8)
3. [My previous TS implementation](https://github.com/joao-conde/chip8-emulator-ts)

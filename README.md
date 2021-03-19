# Chip-8 emulator

Chip-8 emulator written in Rust and compilable to WASM.

Wrote it in Rust because I love the language and wanted to explore handling sound, video and user input with JavaScript + WASM.

## Building

At `Cargo.toml` level, use the following to build and output to the `web` directory using [wasm-pack](https://rustwasm.github.io/wasm-pack/):

```bash
wasm-pack build --release --target=web --no-typescript -d web/ -- --features web
```

Then just serve the `web` folder and open `index.html` in a browser.

## References

1. [Cowgod's Chip-8 Technical Reference v1.0](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
2. [Wikipedia](https://en.wikipedia.org/wiki/CHIP-8)
3. [My previous TypeScript implementation](https://github.com/joao-conde/chip8-emulator-ts)

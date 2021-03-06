# CHIP-8 emulator

CHIP-8 emulator written in Rust and compiled to WASM.

I first wrote it in TypeScript: [chip8-ts](https://github.com/joao-conde/chip8-emulator-ts). Re-writing it in Rust because I love the language and to explore WASM compilation which I will use in my NES emulator.

### References

1. https://en.wikipedia.org/wiki/CHIP-8
2. https://www.reddit.com/r/EmuDev/comments/eb2nac/the_chip8_archive_a_collection_of_modern_chip8/
3. http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
4. http://devernay.free.fr/hacks/chip8/C8TECH10.HTM (Cowgods)
5. https://github.com/hugoferreira/chip8
6. https://github.com/loktar00/chip8/
7. https://github.com/stianeklund/chip8
8. https://github.com/reu/chip8.js
9. http://emulator101.com/
10. https://www.taniarascia.com/writing-an-emulator-in-javascript-chip8/
11. https://johnearnest.github.io/chip8Archive/

### Building

`wasm-pack build --release --target=web -d demo/`

`git subtree push --prefix demo origin gh-pages`

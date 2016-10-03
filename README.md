# gbemu
A WIP Gameboy emulator that I implemented to checkout Rust and learn something about emulators alongside. 
As this was my first project in Rust, the code is probably not as idiomatic as it could be. 

This emulator is not bug free! In fact, the only game I tested that runs perfect is Tetris.
I suspect there are some errors left in the interrupt and timer code.
If you instead want a Rust Gameboy emulator that runs flawlessly, I refer you to https://github.com/Gekkio/mooneye-gb. 

You will need a Gameboy BIOS ROM to run the emulator!
To start the emulator, execute: 

```
cargo run -- --bios path_to_bios.bin path_to_game.gb
```
Tested under Linux with Rust 0.12.

Things currently not implemented:
- Sound
- Gameboy Color support

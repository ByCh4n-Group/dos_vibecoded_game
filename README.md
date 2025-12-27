# DOS Vibecoded Game

This is a Rust-based game using the Tetra engine, simulating an old MS-DOS environment.

## Features

- **BIOS Boot Screen**: Simulates an old Award BIOS boot sequence with memory check and drive detection.
- **MS-DOS Boot**: Simulates the MS-DOS startup sequence with `AUTOEXEC.BAT` commands.
- **Menu**: A classic MS-DOS style blue menu.

## Requirements

- Rust (latest stable)
- SDL2 libraries (Tetra dependency)

## How to Run

```bash
cargo run
```

## Controls

- **Menu**: Use Up/Down arrow keys to navigate.

## Notes

- The game uses a system font copied to `resources/DOS_font.ttf`. If the text looks weird or doesn't load, you might need to provide a proper DOS VGA font file at that location.
# dos_vibecoded_game

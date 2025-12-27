# DOS Vibecoded Game

This is a Rust-based game using the Tetra engine, simulating an old MS-DOS environment.

<img width="640" height="512" alt="image" src="https://github.com/user-attachments/assets/e3c2eaab-521a-4f72-9f15-8f770c9e4e5f" />
<img width="640" height="512" alt="image" src="https://github.com/user-attachments/assets/be7fce5e-1bbb-4e6a-850f-751a27c9c34b" />
<img width="640" height="512" alt="image" src="https://github.com/user-attachments/assets/153d57f4-c34c-4ab9-83a3-c9a70263429e" />

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

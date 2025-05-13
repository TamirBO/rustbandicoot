**Rustbandicoot** – PlayStation 1 Emulator in Rust

⚠️ Under Development: Only the CPU core is fully implemented. GPU, SPU, BIOS handling, and SDL integration are works in progress.

A command-line PS1 emulator prototype written in Rust, currently focusing on CPU instruction emulation and basic debugger support.

---

## Features

* CPU Emulation: Implemented MIPS R3000A instruction set for PS1 CPU.
* Interactive Debugger: Step through CPU instructions, inspect registers and memory, set breakpoints.

*Planned:* GPU rendering, audio emulation, BIOS support, SDL2 frontend.   GPU rendering, audio emulation, BIOS support, SDL2 frontend.

---

## Prerequisites

* Rust (1.70 or later)

---

## Building

Clone the repository and build:

```bash
git clone https://github.com/TamirBO/rustbandicoot.git
cd rustbandicoot
cargo build --release
```

Executable is in `target/release/`.

---

## Running

Currently, only BIOS boot is supported. Run the emulator to launch the PS1 BIOS:

```bash
cargo run --release --bin debugger
```

> **Note:** Create a `binaries/` directory in the project root and place your PS1 BIOS image named **SCPH1001.BIN** inside it before running.

---

## Project Structure

```
.
├── .idea/             # IDE configuration files
├── debugger/          # Debugger UI and CLI launcher
├── disassembler/      # Disassembler and instruction decoding
├── docs/              # Design documents and CPU specifications
├── ps/                # CPU core implementation (MIPS R3000A)
├── .gitignore         # Git ignore rules
├── Cargo.toml         # Project manifest
├── Cargo.lock         # Locked dependencies
└── rustfmt.toml       # Formatting configuration
```

---

## License

Licensed under the MIT License.

```
```

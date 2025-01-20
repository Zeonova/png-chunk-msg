
# PNG Chunk Message Encoder/Decoder in Rust

This project is an implementation of a PNG message encoding/decoding tool using Rust, inspired by the [PNGme Book](https://jrdngr.github.io/pngme_book/chapter_4.html). It provides a command-line interface (CLI) and a graphical user interface (GUI) built using the Iced framework.

## Project Structure

```
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── cli
│   ├── 382px-PNG_Test.png
│   ├── Cargo.toml
│   ├── ruSt.png
│   └── src
├── core
│   ├── Cargo.toml
│   └── src
├── gui
│   ├── Cargo.toml
│   └── src
├── src
│   └── main.rs
```

### Folders:
- **cli**: Contains the command-line interface implementation, including assets like `382px-PNG_Test.png` and the main encoding/decoding logic.
- **core**: Houses the core logic of the PNG encoding/decoding process, which can be used by both the CLI and GUI.
- **gui**: Contains the GUI application built with the Iced framework for a more user-friendly interface.

## Features

### Command-Line Interface (CLI)
The CLI provides four main commands for interacting with PNG images and secret messages:

1. **Encode a message**: 
   ```
   png-chunk-msg-cli encode ./dice.png ruSt "This is a secret message!"
   ```

2. **Decode a message**:
   ```
   png-chunk-msg-cli decode ./dice.png ruSt
   ```

3. **Remove a chunk**:
   ```
   png-chunk-msg-cli remove ./dice.png ruSt
   ```

4. **Print chunk information**:
   ```
   png-chunk-msg-cli print ./dice.png
   ```

### Graphical User Interface (GUI)
The GUI offers a more intuitive way to interact with the PNG message encoding/decoding system. It is powered by the Iced framework, providing a cross-platform application to encode, decode, and manage chunks in PNG files.

## Requirements

- Rust (latest stable version)
- Cargo (Rust package manager)
- Iced (for the GUI)

## Installation

For the CLI:

```bash
cargo run -p png-chunk-msg-cli help
```

For the GUI:

```bash
cargo run -p pcm-gui  
```


# Brainsuck 🧠

A minimalist Brainfuck interpreter written in **Crust** (C-style Rust using `#![no_std]`) with custom `crlib` for C bindings and additional utilities.

## Features

* ✅ All 8 Brainfuck commands supported
* ✅ 30,000 memory cells
* ✅ File input/output
* ✅ Interactive input with `,` command
* ✅ Proper bracket matching for loops
* ✅ Built with custom `crlib` for C bindings and additional functions

## Brainfuck Commands

| Command | Description |
|---------|-------------|
| `+` | Increment memory cell |
| `-` | Decrement memory cell |
| `>` | Move pointer right |
| `<` | Move pointer left |
| `.` | Output character |
| `,` | Input character |
| `[` | Jump forward if cell is 0 |
| `]` | Jump backward if cell is non-zero |

## Installation

```bash
git clone https://github.com/0l3d/brainsuck.git
cd brainsuck
make
```

## Usage

```bash
./brainsuck <brainfuck_file>
```

## Sample Programs

### Hello World (`hello.bf`)

```brainfuck
++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.
```

### Game of Life (`life.bf`)

Conway's Game of Life cellular automaton simulator with interactive interface.

**How it works:**
- Displays a 10x10 grid (expandable to 166x166) 
- Use coordinates like "be" to toggle cell at column b, row e
- Press Enter to advance one generation
- Type "q" to quit
- Grid wraps around (toroidal)
- Follows standard Game of Life rules: cells live/die based on neighbor count
- Living cells shown as `*`, dead cells as `-`

## Run Examples

```bash
./brainsuck examples/hello.bf
./brainsuck examples/life.bf
./brainsuck examples/calculator.bf
```

## Technical Details

* **Language**: Crust (C-style Rust with `#![no_std]`)
* **Memory**: 30,000 byte cells
* **Dependencies**: Custom `crlib` for C bindings and additional functions
* **Architecture**: Uses C bindings instead of standard library

## Project Structure

```
brainsuck/
├── crlib.rs             # Custom C library bindings
├── interpreter.rs       # Main interpreter
├── examples/
│   ├── hello.bf         # Hello World example
│   └── life.bf          # Game of Life example
│   └── calculator.bf          # Game of Life example
├── Makefile
├── LICENSE.md
└── README.md
```

## License

This project is licensed under the **GPL-3.0 License**.

---
Created by **0l3d**

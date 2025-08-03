# Brainsuck 🧠

A minimalist Brainfuck interpreter and compiler written in **C**

## Features

- All 8 Brainfuck commands supported
- 30,000 memory cells
- File input/output
- Interactive input with `,` command
- Proper bracket matching for loops
- Compiles brainfuck code to FASM (x86_64 linux).

## Brainfuck Commands

| Command | Description                       |
| ------- | --------------------------------- |
| `+`     | Increment memory cell             |
| `-`     | Decrement memory cell             |
| `>`     | Move pointer right                |
| `<`     | Move pointer left                 |
| `.`     | Output character                  |
| `,`     | Input character                   |
| `[`     | Jump forward if cell is 0         |
| `]`     | Jump backward if cell is non-zero |

## Installation

```bash
git clone https://github.com/0l3d/brainsuck.git
cd brainsuck
gcc -Os brainsuck.c -o brainsuck
```

## Usage

```bash
./brainsuck <brainfuck_file>
```

### Hello World (`hello.bf`)

```brainfuck
++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.
```

## Run Examples

```bash
./brainsuck examples/hello.bf
./brainsuck examples/life.bf
./brainsuck examples/calculator.bf
```

## License

This project is licensed under the **GPL-3.0 License**.

## Author

Created by **0l3d**.

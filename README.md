# wasm-custom-section

> Write a custom section in a Wasm module

## Install

```
$ cargo install wasm-custom-section
```

## Usage

### Add a new custom section

```
$ wasm-custom-section ./input.wasm add SECTION_NAME < FILE
```

Output file will be written at `FILE.out`.

### List custom sections

```
$ wasm-custom-section ./input.wasm list

Section `name` (9 bytes)
Section `test` (3232 bytes)
```

### Show a specific custom sections

```
$ wasm-custom-section ./input.wasm show SECTION_NAME

Section `test` (3232 bytes):
Length: 3232 (0xca0) bytes
0000:   75 73 65 20  63 6c 61 70  3a 3a 7b 41  72 67 2c 20   use clap::{Arg,
0010:   43 6f 6d 6d  61 6e 64 7d  3b 0a 75 73  65 20 70 72   Command};.use pr
0020:   65 74 74 79  5f 68 65 78  3a 3a 73 69  6d 70 6c 65   etty_hex::simple
0030:   5f 68 65 78  3b 0a 75 73  65 20 73 74  64 3a 3a 66   _hex;.use std::f
0040:   73 3b 0a 75  73 65 20 73  74 64 3a 3a  69 6f 3b 0a   s;.use std::io;.
0050:   75 73 65 20  73 74 64 3a  3a 69 6f 3a  3a 52 65 61   use std::io::Rea
0060:   64 3b 0a 75  73 65 20 73  74 64 3a 3a  73 74 72 3b   d;.use std::str;
0070:   0a 0a 74 79  70 65 20 42  6f 78 45 72  72 20 3d 20   ..type BoxErr =
0080:   42 6f 78 3c  64 79 6e 20  73 74 64 3a  3a 65 72 72   Box<dyn std::err
0090:   6f 72 3a 3a  45 72 72 6f  72 3e 3b 0a  0a 66 6e 20   or::Error>;..fn
00a0:   6d 61 69 6e  28 29 20 2d  3e 20 52 65  73 75 6c 74   main() -> Result

...
```

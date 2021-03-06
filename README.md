# MIPS assembler, linker, simulator, and debugger

## Build
This project is written in Rust, so first install Rust:
https://www.rust-lang.org/tools/install

Now that Rust is installed, use `cargo` to build the project
(with optimizations):
```
cargo build --release
```

The compiled binaries are located in `target/release/`, namely `mips-assembler`,
`mips-linker`, `mips-simulator`, and `mips-debugger`.

## Goals
- Assembler
  - [X] Parse assembly code
  - [X] Assemble code into object files
  - [ ] Support all directives and instructions
  - [X] Handle global (extern) references
  - [X] Support data sections like sdata
- Linker
  - [X] Support transforming one independent object module (only exports
        main) into an executable file (R2K format).
  - [X] Support linking (at least) two object modules together such that a
        global exported by one and used by another is wired up correctly.
  - [X] Link in the r2k_startup module if `__r2k__entry__` is not defined. This
        handles calling `main` and closing the program at the end. This should
        also be set as the entry point of the program.
  - [X] All sections (including rdata/data/sdata) are relocated correctly.
- Simulator
  - [X] Create a simulated MIPS CPU
  - [X] Parse R instructions
  - [ ] Execute all R instructions
  - [X] Parse I instructions
  - [ ] Execute all I instructions
  - [X] Parse J instructions
  - [X] Execute all J instructions
- Debugger
  - [X] Drive the simulator
  - [X] Inspect the registers
  - [ ] Inspect the surrounding code/instructions
  - [X] Breakpoints

## Test Programs
The `programs` directory contains some test programs which have been assembled
and linked with a MIPS toolchain I refer to as "R2K" (the assembler is "rasm",
the linker is "rlink", the simulator is "rsim", and the debugger is "rbug").
This was the tool I used in college when learning MIPS, but access to it was
very restricted (only available on university servers, execute permissions
only), and I have not found it anywhere online. I used it to provide assembled
binaries while my own assembler and linker were still in development.

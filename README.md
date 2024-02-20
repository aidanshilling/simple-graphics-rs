# Simple Graphics RS

### Features

**Language**: [Rust](https://www.rust-lang.org/)

- Crates used:
  These will automatically be installed to the project using cargo which comes built in with rust on tux.
  1. [Pest](https://pest.rs/) - Used for parsing files
  2. [Clap](https://docs.rs/clap/latest/clap/) - Used for parsing command line args
  3. [itertools](https://docs.rs/itertools/latest/itertools/) - Used for sorting vectors
  4. [nalgebra](https://nalgebra.org/) - Used for matrix operations

Matrix operations were all done with nalgebra. 

Formal postscript parser written with Pest. 

Command line parsing with Clap. Running the program with the `--help` flag will print out all the args, their uses and their default values.

All output is sent to stdout so you must redirect into a file using `>` or `>>`.

### SMF Files

This program takes in a `.smf` file and outputs an image in `.ps` format to stdout. All .smf and .ps related code can be found in the `ps` and `smf` folders respectively.

### Usage
**WIP**

1. Populate main method as needed.
2. Build the program by running `cargo build`.
3. Run `./target/debug/simple-graphics-rs --help` for instructions on how to use the arguments.

### The code

The entrypoint is located in `src/main.rs`. There is one other file in the main src directory `args.rs` which contains the command line argument parsing.

There are 4 crates (libraries): `ps`, `smf`, `transform_2d` and `transform_3d`


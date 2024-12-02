# Advent of Code 2024
This repository contains the programs I have written to complete the [Advent of Code 2024](https://adventofcode.com/2024) daily programming puzzles.

I'm using [Rust](https://www.rust-lang.org/) to complete these challenges to broaden my understanding of the language.

## Running the Rust programs

The following commands are for UNIX systems but most commands should also work on Windows by using [PowerShell](https://learn.microsoft.com/en-us/powershell/scripting/overview), if the command does not work, the text above the command help you figure out the right equivalent for PowerShell.

### Installing Rust
To run the rust programs, you must have Rust installed. To install Rust, navigate to [the Rust install page](https://www.rust-lang.org/tools/install).

### Nightly (Leftover from 2022, might not be needed)
There are some features not yet available in the stable version of Rust that I am using, so you will need the nightly version of Rust. To use the nightly version, run the following command:
```sh
rustup toolchain install nightly
```

To start using the nightly toolchain, run the following command:
```
rustup default nightly
```

### Building the binaries
Next, navigate to the rust directory:
```sh
cd rust
```

To build the binaries, run the following command:
```sh
cargo build --release
```

### Running the binaries
To run one the solver for day 1, run the binary called `day1` in the `target/release` directory:
Note: on Windows, the binary is called `day1.exe`, but the command still works.
```sh
./target/release/day1
```

All other binaries follow the same format, day 2's binary is `day2`, etc.
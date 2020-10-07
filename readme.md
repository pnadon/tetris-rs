# Tetris-rs

A Tetris game implemented in Rust, inspired by [a similar version written in C++](https://github.com/k-vernooy/tetris).
Big thanks to [k-vernooy](https://github.com/k-vernooy/tetris) for open-sourcing his work!

# Installation

## MacOS
```shell
brew tap pnadon/tap
brew install tetris-rs
```

## Linux
- Ensure you have Rust & Cargo installed
```shell
cargo install --git https://github.com/pnadon/tetris-rs.git
```

# Notes

At first the primary goal for this project was to take [k-vernooy's Tetris game](https://github.com/k-vernooy/tetris) and more or less directly port it to Rust.

Shortly after I started however, I began to make drastic changes to the structure of the project so that it better fit my own coding style as well as Rust's own features. The current version of the project differs drastically from the original, with various features (and likely bugs) added.

A surprisingly big different between the Rust and C++ implementation is the difference between indexing. Since C++ uses an `int` for indexing, the `int` is allowed to be a negative value (up until it is finally used for indexing). Thus, the coordinates for a tetrimino could be negative before finally being added to an offset and used for indexing. On the other hand Rust uses a `usize` for indexing, which is unsigned and caused some issues in the implementation, which were finally resolved by rewriting some parts so that coordinates would always be a non-negative number.

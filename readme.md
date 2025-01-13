# B\[rain\]rot

A [Brainf*ck](https://en.wikipedia.org/wiki/Brainfuck) interpreter.

## Installation

1. With nix `nix build github:0xmycf/brot#brot`
2. With git `git clone https://github.com/0xmycf/brot`
  + then `cargo build --release` or `nix build .#brot`

## Usage

1. With Nix `./result/bin/brot <filename>`
2. With git and cargo: `./target/release/brot <filename>`

## Example programas

There are multiple examples in the `./examples/` directory.

A few (at least *collatz* and *cellular*) of these are taken verbatim from [brainfuck.org](https://brainfuck.org/).

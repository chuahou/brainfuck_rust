# Brainfuck (Rust)

[![Build Status](https://travis-ci.org/chuahou/brainfuck_rust.svg?branch=master)](https://travis-ci.org/chuahou/brainfuck_rust)

Yet another brainfuck interpreter (this time in Rust), because I'm bored and want to do easy
projects. Once again, this badge says it best:

[![forthebadge](https://forthebadge.com/images/badges/you-didnt-ask-for-this.svg)](https://forthebadge.com)

## Usage

To build using Cargo, run

    cargo build

To run using Cargo, use

    cargo run -- <arguments>

or directly call the binary with

    bin/brainfuck_rust <arguments>

To read a brainfuck program from a file, run

    [BINARY] --file <INPUT FILE>
    [BINARY] --file helloworld.b # example

Alternatively, you can read a program directly from the command line arguments using

    [BINARY] --program <PROGRAM>
    [BINARY] --program ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++. # example

For more information, run

    [BINARY] --help

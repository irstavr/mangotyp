# mangotyp - Make Rust to speak to Typescript

A small prototype as a learning aid for building a translation utility from Rust to Typescript types.

It supports primitives, structs, and enums, among other kinds. It is not intended for use in production. 
It will provide a warning message and only ignore unsupported kinds.

### How to use

    cargo run -- --input=<path-to-input-rust-file> --output=<path-to-output-typescript-file>

### How to test

    cargo test

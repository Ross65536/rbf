# rbf

Rust brainfuck interpreter I've implemented for fun.

## Spec Details

### EOF

When a EOF is reached on `stdin` the value `-1` is written on the current record.

### Record

Record are 8 byte signed integers (`i64`). 

There are records with negative indexes and there is no limit to the number of records.

## CLI 

The application accepts as argument the file of the brainfuck source, otherwise the `stdin` is treated as the source.

## Instructions

### Building 

    cargo build

### Tests

Test must be run single-thread.

Run with:

    cargo test -- --test-threads=1


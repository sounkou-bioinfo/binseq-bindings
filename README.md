# BINSEQ C-Bindings

This document provides an overview of the C bindings for the BINSEQ library, a high-performance library for working with fixed-length DNA sequences in the BINSEQ (.bq) file format.

## Installation

Prerequisites:

- [`cargo`](https://rustup.rs/)
- C compiler
- Make

### Building from Source

1. Clone the repository:

```bash
git clone github.com:noamteyssier/binseq-bindings.git
cd binseq-bindings
```

2. Compile the rust dynamic library

```bash
cargo build --release
```

The compiled library will be available at either `./target/release/libbinseq.so` (linux) or `./target/release/libbinseq.dylib` (macOS).

The C-Header will be auto-generated at `./binseq-c/binseq.h`.

3. Building and Running examples

```bash
cd binseq-c/examples/
make

./simple_example <path_to_file>.bq
```

## Generating BINSEQ Files

You can use [`bqtools`](https://github.com/noamteyssier/bqtools) to encode FASTA or FASTQ files into BINSEQ.

```bash
# Encode into BINSEQ
bqtools encode <fastq> -o <output>.bq

# Decode from BINSEQ into FASTQ
bqtools decode <binseq> -fq -o <fastq>
```

BINSEQ is built for paired records and encodes them both into a single file.

```bash
# Encode record pairs into BINSEQ
bqtools encode <R1> <R2> -o <output>.bq
```

# BINSEQ Bindings

This repository provides language bindings for the BINSEQ library, a high-performance Rust library for working with fixed-length DNA sequences in the BINSEQ (.bq) file format.

Currently supported bindings:

- C bindings via FFI
- C++ bindings via CXX

## Installation

Prerequisites:

- [`cargo`](https://rustup.rs/) - Rust's package manager
- C/C++ compiler (gcc, clang, or MSVC)
- Make (for building examples)

### Building from Source

1. Clone the repository:

```bash
git clone github.com:arcinstitute/binseq-bindings.git
cd binseq-bindings
```

2. Compile the Rust libraries:

```bash
cargo build --release
```

The compiled libraries will be available at:

- C bindings: `./target/release/libbinseq.so` (Linux) or `./target/release/libbinseq.dylib` (macOS)
- C++ bindings: `./target/release/libbinseq_cxx.a` (static library)

Headers will be auto-generated at:

- C header: `./binseq-c/binseq.h`
- C++ header: `./target/cxxbridge/binseq-cxx/src/lib.rs.h`

## Using the C Bindings

The C bindings provide a simple, C-compatible API for working with BINSEQ files.

### Building and Running the C Example

```bash
cd binseq-c/examples/
make
./simple_example <path_to_file>.bq
```

### Example C Usage

```c
// Create a reader
struct BinseqReader *reader = binseq_reader_open("sequence.bq");

// Get file info
size_t num_records = binseq_reader_num_records(reader);
uint32_t seq_len = binseq_reader_slen(reader);

// Create decoding context and record container
struct BinseqContext *ctx = binseq_context_new();
struct BinseqRecord *record = binseq_record_new();

// Process records
for (size_t i = 0; i < num_records; i++) {
    binseq_reader_get_record(reader, i, record);

    // Decode sequence
    size_t len = binseq_record_decode_primary(record, ctx);

    // Access sequence data
    const char *seq = binseq_context_primary_ptr(ctx);
    // Process sequence...
}

// Cleanup
binseq_record_free(record);
binseq_context_free(ctx);
binseq_reader_close(reader);
```

## Using the C++ Bindings

The C++ bindings provide a more idiomatic C++ API with better type safety.

### Building and Running the C++ Example

```bash
cd binseq-cxx/examples/
make
./simple_example <path_to_file>.bq
```

### Example C++ Usage

```cpp
#include "rust/cxx.h"
#include "binseq-cxx/src/lib.rs.h"

// Open a reader
auto reader = open_mmap_reader("sequence.bq");

// Get file info
size_t num_records = reader->num_records();
uint32_t slen = reader->get_slen();

// Create buffer for sequence data
rust::Vec<uint8_t> sbuf;

// Process records
for (size_t i = 0; i < num_records; i++) {
    auto record = reader->get_record(i);

    // Decode sequence
    sbuf.clear();
    record->decode_s(sbuf);

    // Convert to string if needed
    std::string seq_str(sbuf.begin(), sbuf.end());

    // Process sequence...
}
```

## Generating BINSEQ Files

You can use [`bqtools`](https://github.com/noamteyssier/bqtools) to encode FASTA or FASTQ files into BINSEQ.

```bash
# Encode into BINSEQ
bqtools encode <fastq> -o <output>.bq

# Decode from BINSEQ into FASTQ
bqtools decode <binseq> -fq -o <fastq>
```

BINSEQ is built for paired records and encodes them both into a single file:

```bash
# Encode record pairs into BINSEQ
bqtools encode <R1> <R2> -o <output>.bq
```

## BINSEQ Format

BINSEQ (.bq) is a binary file format designed for efficient storage of fixed-length DNA sequences. It uses 2-bit encoding for nucleotides (A=00, C=01, G=10, T=11) and focuses exclusively on sequence data, optimizing for modern high-throughput sequencing applications.

For more details on the BINSEQ format, see the original [BINSEQ documentation](https://github.com/arcinstitute/binseq).

## Performance

The BINSEQ format and these bindings are designed for high-performance processing of sequence data. Performance benchmarks can be run with the included examples.

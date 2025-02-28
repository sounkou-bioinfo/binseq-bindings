.PHONY: all clean c-bindings example

# Default target
all: c-bindings example

# Build the Rust library with C bindings
c-bindings:
	cargo build --release --manifest-path ../Cargo.toml

# Build the C example
example: c-bindings
	$(CC) -o simple_example simple_example.c -L../target/release -lbinseq

# Clean build artifacts
clean:
	rm -f simple_example binseq.h

# Run the example with a binseq file
run: example
	./simple_example $(FILE)

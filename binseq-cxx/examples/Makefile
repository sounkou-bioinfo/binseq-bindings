.PHONY: all clean build-rust run

CXX = g++
CXXFLAGS = -std=c++17
INCLUDES = -I../../target/cxxbridge
LDFLAGS = -L../../target/release
LDLIBS = -lbinseq_cxx
TARGET = simple_example

all: build-rust $(TARGET)

# Build the Rust library first
build-rust:
	cd ../.. && cargo build --release

# Build the C++ example
$(TARGET): simple_example.cpp
	$(CXX) $(CXXFLAGS) $(INCLUDES) -o $@ $< $(LDFLAGS) $(LDLIBS)

# Clean build artifacts
clean:
	rm -f $(TARGET)

# Run the example with a binseq file
run: all
	./$(TARGET) $(FILE)

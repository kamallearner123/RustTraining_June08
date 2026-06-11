# Makefile for C++ Parsing Rust Message

CXX = g++
CXXFLAGS = -Wall -Wextra -std=c++17
LDFLAGS = -lpthread -ldl -lm

RUST_LIB_DIR = rust_lib/target/debug
RUST_LIB = $(RUST_LIB_DIR)/librust_lib.a

all: main

# Build the Rust library
$(RUST_LIB):
	cd rust_lib && cargo build

# Build the C++ executable
main: main.cpp $(RUST_LIB)
	$(CXX) $(CXXFLAGS) -o $@ main.cpp $(RUST_LIB) $(LDFLAGS)

clean:
	rm -f main
	cd rust_lib && cargo clean

.PHONY: all clean

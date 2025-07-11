# Makefile for TCA9534 C FFI example
# This demonstrates how to build C code that uses the TCA9534 Rust FFI library

# Compiler settings
CC = gcc
CFLAGS = -Wall -Wextra -std=c99 -I../include
LDFLAGS = -L../../target/release -ltca9534

# Target executable
TARGET = c_example
SOURCE = c_example.c

# Default target
all: $(TARGET)

# Build the C example
$(TARGET): $(SOURCE) ../../target/release/libtca9534.lib
	$(CC) $(CFLAGS) -o $(TARGET) $(SOURCE) $(LDFLAGS)

# Build the Rust library if it doesn't exist
../../target/release/libtca9534.lib:
	cd .. && cargo build --release --features capi

# Clean build artifacts
clean:
	rm -f $(TARGET)
	rm -f $(TARGET).exe

# Force rebuild
rebuild: clean all

# Run the example
run: $(TARGET)
	./$(TARGET)

# Windows-specific targets
windows: $(TARGET).exe

$(TARGET).exe: $(SOURCE) ../../target/release/tca9534.lib
	$(CC) $(CFLAGS) -o $(TARGET).exe $(SOURCE) -L../../target/release -ltca9534 -lws2_32 -lbcrypt -luserenv


.PHONY: all clean rebuild run windows help 
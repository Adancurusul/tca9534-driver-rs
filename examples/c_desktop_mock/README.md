# TCA9534 C Desktop Mock Example

A desktop C example demonstrating the TCA9534 C FFI interface with mock I2C implementation.

## Requirements

- **C compiler** (GCC, Clang, MSVC)
- **TCA9534 C FFI library** (libtca9534.a)

## Build

**Step 1: Enable staticlib output**

Edit `../../Cargo.toml` and uncomment the `crate-type` line:

```toml
[lib]
name = "tca9534"
crate-type = ["staticlib"] # use when build with capi feature
```

**Step 2: Build the library and example**

```bash
# Compile the Rust library first
cargo build --release --features capi

# Compile the C example
gcc -o c_example c_example.c -L../../target/release -ltca9534
```

**Note**: Remember to comment out the `crate-type` line after building if you want to use the library as a normal Rust dependency.

## Features

The example demonstrates:
- **Basic Pin Control**: Configure pins as input/output, set pin levels
- **Port Operations**: Control all 8 pins at once
- **Input Reading**: Read pin states with polarity inversion
- **Address Management**: Switch between different I2C addresses

## Usage

```bash
./c_example
```

## Example Output

```
TCA9534 C FFI Interface Example
================================

=== Example 1: Basic Pin Control ===
Device initialized successfully!
I2C Write: addr=0x20, data=[0x03, 0xFE]
Pin 0 configured as output
I2C Write: addr=0x20, data=[0x01, 0x01]
Pin 0 set to HIGH
...
```

## Mock I2C Implementation

The example uses a mock I2C implementation that:
- Simulates register reads/writes
- Prints all I2C transactions
- Maintains internal register state

This allows testing the C FFI interface without real hardware.

---

Perfect for testing TCA9534 C integration before hardware deployment. 
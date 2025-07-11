# TCA9534 C FFI Interface

This document provides instructions on how to use the TCA9534 C FFI (Foreign Function Interface) from C/C++ applications.

## Overview

The TCA9534 C FFI interface provides a complete C-compatible API for controlling the TCA9534 I2C IO expander. This interface allows C/C++ applications to use the robust Rust TCA9534 driver implementation while maintaining the familiar C programming model.

## Architecture

```
┌─────────────────────────────────────┐
│           C Application             │
├─────────────────────────────────────┤
│         C FFI API Layer             │  <- User Interface
├─────────────────────────────────────┤
│    C Function Pointer Transport     │  <- Adapts to SyncTransport
├─────────────────────────────────────┤
│       TCA9534 Rust Core             │  <- Existing Rust Implementation
├─────────────────────────────────────┤
│       SyncTransport Trait          │  <- Existing trait interface
└─────────────────────────────────────┘
```

## Features

- **Complete API Coverage**: All TCA9534 functionality available through C interface
- **No-std Compatible**: Suitable for embedded systems without standard library
- **Zero-cost Abstraction**: Minimal runtime overhead
- **Transport Agnostic**: Works with any I2C implementation via function pointers
- **Error Handling**: Comprehensive error reporting with C-compatible error codes
- **Multiple Devices**: Support for multiple TCA9534 devices on the same bus

## Examples

Two complete examples are provided:

1. **STM32 Integration**: `../tca_ffi_demo/` - Real STM32 hardware integration
2. **Desktop Mock**: `examples/c_desktop_mock/` - Desktop testing with mock I2C

## Building

### Prerequisites

- Rust toolchain (stable)
- C compiler (GCC, Clang, or MSVC)
- For STM32: STM32CubeIDE and ARM toolchain

### Building for Desktop (x86_64)

**Step 1: Enable staticlib output**

Edit `Cargo.toml` and uncomment the `crate-type` line:

```toml
[lib]
name = "tca9534"
crate-type = ["staticlib"] # use when build with capi feature
```

**Step 2: Build the library**

```bash
# Build the static library with C FFI support
cargo build --release --features capi

# Build the desktop mock example
cd examples/c_desktop_mock
gcc -o c_example c_example.c -L../../target/release -ltca9534
```

### Building for STM32 (ARM)

**Step 1: Enable staticlib output**

Edit `Cargo.toml` and uncomment the `crate-type` line:

```toml
[lib]
name = "tca9534"
crate-type = ["staticlib"] # use when build with capi feature
```

**Step 2: Build for ARM**

```bash
# Build for ARM Cortex-M4F with hardware FPU
cargo build --release --target thumbv7em-none-eabihf --no-default-features --features capi

# Copy the library to STM32 project
cp target/thumbv7em-none-eabihf/release/libtca9534.a ../tca_ffi_demo/Core/Src/
```

**Important**: 
- Use `thumbv7em-none-eabihf` target for STM32 with FPU support
- Remember to comment out the `crate-type = ["staticlib"]` line after building if you want to use the library as a normal Rust dependency

## Quick Start

### 1. Include the Header

```c
#include "tca9534.h"
```

### 2. Implement I2C Functions

You need to provide I2C transport functions that match your platform:

```c
// Example I2C function implementations
int my_i2c_write(void* ctx, uint8_t addr, const uint8_t* data, size_t len) {
    // Your I2C write implementation
    return 0; // Return 0 for success, non-zero for error
}

int my_i2c_read(void* ctx, uint8_t addr, uint8_t* data, size_t len) {
    // Your I2C read implementation
    return 0; // Return 0 for success, non-zero for error
}

int my_i2c_write_read(void* ctx, uint8_t addr, const uint8_t* wr_data, 
                     size_t wr_len, uint8_t* rd_data, size_t rd_len) {
    // Your I2C write-read implementation
    return 0; // Return 0 for success, non-zero for error
}
```

### 3. Initialize and Use

```c
// Setup I2C operations structure
tca9534_i2c_ops_t i2c_ops = {
    .write = my_i2c_write,
    .read = my_i2c_read,
    .write_read = my_i2c_write_read
};

// Initialize device handle
tca9534_handle_t device;
void* i2c_context = &my_i2c_bus; // Your I2C context

tca9534_error_t result = tca9534_init(&device, TCA9534_ADDR_000, 
                                     i2c_context, &i2c_ops);
if (result != TCA9534_OK) {
    printf("Failed to initialize TCA9534: %d\n", result);
    return -1;
}

// Configure pin 0 as output and set it high
tca9534_set_pin_config(&device, 0, TCA9534_PIN_OUTPUT);
tca9534_set_pin_output(&device, 0, TCA9534_LEVEL_HIGH);

// Read pin 1 input
tca9534_pin_level_t level;
result = tca9534_read_pin_input(&device, 1, &level);
if (result == TCA9534_OK) {
    printf("Pin 1 is %s\n", (level == TCA9534_LEVEL_HIGH) ? "HIGH" : "LOW");
}
```

## API Reference

### Error Codes

```c
typedef enum {
    TCA9534_OK = 0,                    // Success
    TCA9534_ERROR_INVALID_PIN = -1,    // Invalid pin number (0-7)
    TCA9534_ERROR_I2C_WRITE = -2,      // I2C write error
    TCA9534_ERROR_I2C_READ = -3,       // I2C read error
    TCA9534_ERROR_NULL_PTR = -4,       // Null pointer error
    TCA9534_ERROR_INIT_FAILED = -5     // Initialization failed
} tca9534_error_t;
```

### Pin Configuration

```c
typedef enum {
    TCA9534_PIN_OUTPUT = 0,    // Pin as output
    TCA9534_PIN_INPUT = 1      // Pin as input
} tca9534_pin_config_t;

typedef enum {
    TCA9534_LEVEL_LOW = 0,     // Logic low
    TCA9534_LEVEL_HIGH = 1     // Logic high
} tca9534_pin_level_t;
```

### Key Functions

#### Initialization
- `tca9534_init()`: Initialize device with specific address
- `tca9534_init_default()`: Initialize device with default address (0x20)

#### Pin Operations
- `tca9534_set_pin_config()`: Configure pin as input/output
- `tca9534_set_pin_output()`: Set output pin level
- `tca9534_read_pin_input()`: Read input pin level
- `tca9534_toggle_pin_output()`: Toggle output pin

#### Port Operations (More Efficient)
- `tca9534_set_port_config()`: Configure all pins at once
- `tca9534_write_output_port()`: Set all outputs at once
- `tca9534_read_input_port()`: Read all inputs at once

### I2C Addresses

```c
#define TCA9534_ADDR_000  0x20  // A2=0, A1=0, A0=0 (default)
#define TCA9534_ADDR_001  0x21  // A2=0, A1=0, A0=1
#define TCA9534_ADDR_010  0x22  // A2=0, A1=1, A0=0
#define TCA9534_ADDR_011  0x23  // A2=0, A1=1, A0=1
#define TCA9534_ADDR_100  0x24  // A2=1, A1=0, A0=0
#define TCA9534_ADDR_101  0x25  // A2=1, A1=0, A0=1
#define TCA9534_ADDR_110  0x26  // A2=1, A1=1, A0=0
#define TCA9534_ADDR_111  0x27  // A2=1, A1=1, A0=1
```

### Useful Constants

```c
#define TCA9534_ALL_INPUTS           0xFF  // All pins as inputs
#define TCA9534_ALL_OUTPUTS          0x00  // All pins as outputs
#define TCA9534_ALL_OUTPUTS_LOW      0x00  // All outputs low
#define TCA9534_ALL_OUTPUTS_HIGH     0xFF  // All outputs high
```

## Error Handling

Always check return values:

```c
tca9534_error_t result = tca9534_set_pin_output(&device, 0, TCA9534_LEVEL_HIGH);
if (result != TCA9534_OK) {
    switch (result) {
        case TCA9534_ERROR_INVALID_PIN:
            printf("Invalid pin number\n");
            break;
        case TCA9534_ERROR_I2C_WRITE:
            printf("I2C write failed\n");
            break;
        case TCA9534_ERROR_NULL_PTR:
            printf("Null pointer error\n");
            break;
        default:
            printf("Unknown error: %d\n", result);
            break;
    }
}
```

## Platform Integration

### STM32 Integration

For STM32 projects, see the complete example in `../tca_ffi_demo/`:

- **STM32 HAL Integration**: `tca9534_stm32.c/h` provides convenience functions
- **Hardware FPU Support**: Uses `thumbv7em-none-eabihf` target
- **LED Control Demo**: Shows animated LED patterns
- **Input Monitoring**: Demonstrates switch reading

### Desktop Testing

For desktop development and testing, see `examples/c_desktop_mock/`:

- **Mock I2C Implementation**: No hardware required
- **Complete API Coverage**: Tests all C FFI functions
- **Debug Output**: Prints all I2C transactions
- **Cross-platform**: Works on Windows, Linux, macOS

## Best Practices

1. **Always check return values** from all functions
2. **Use port operations** for better performance when controlling multiple pins
3. **Initialize once** and reuse the device handle
4. **Handle I2C errors** appropriately in your transport functions

## Common Issues

1. **Compilation Errors**: 
   - Ensure correct ARM target for embedded systems
   - Check that `--features capi` is used when building

2. **Linking Errors**: 
   - Verify library path and include paths
   - Check that library was built for correct target architecture

3. **Runtime Errors**: 
   - Verify I2C connections and pull-up resistors
   - Check TCA9534 power supply (typically 3.3V)
   - Validate I2C transport function implementations

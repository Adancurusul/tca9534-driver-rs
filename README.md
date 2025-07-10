# TCA9534 Driver

[![Crates.io](https://img.shields.io/crates/v/tca9534.svg)](https://crates.io/crates/tca9534)
[![Documentation](https://docs.rs/tca9534/badge.svg)](https://docs.rs/tca9534)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/your-username/tca9534-driver)

A platform-independent Rust driver for the **TCA9534** I2C IO expander, with optional [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) integration.

## 🚀 Key Features

- 🔧 **Zero Dependencies**: Core driver works without any external dependencies
- 🎯 **Optional embedded-hal Integration**: Seamless integration when needed
- 🔄 **Sync & Async Support**: Both synchronous and asynchronous operations
- 📦 **`no_std` Compatible**: Perfect for embedded systems
- 🎨 **Ergonomic API**: Simple and intuitive interface
- 📖 **Well Documented**: Comprehensive documentation and examples

## 🏗️ Architecture Design

### **Layered Architecture**

```
┌─────────────────────────────────────┐
│           User Code Layer            │
├─────────────────────────────────────┤
│  Convenience Constructors           │ <- embedded-hal optional
│  (new_with_default_address)         │ 
├─────────────────────────────────────┤
│  TCA9534 Driver Core                │ <- Always available
├─────────────────────────────────────┤
│  Transport Traits                   │ <- Always available
│  (SyncTransport/AsyncTransport)     │
├─────────────────────────────────────┤
│  Transport Implementation Layer     │ <- embedded-hal optional
└─────────────────────────────────────┘
```

### 📋 **Feature-Based Compilation Strategy**

**Core Principle: Driver core and transport traits are always available, embedded-hal integration is optional**

- **Default Mode** (`full-async`): Complete async functionality with embedded-hal integration
- **Minimal Mode** (`default-features = false`): Zero dependencies, custom transport only
- **Sync Mode** (`embedded-hal`): Synchronous operations with embedded-hal I2C traits
- **Async Mode** (`async` + `embedded-hal-async`): Async/await support with embedded-hal integration

## About TCA9534

The TCA9534 is an 8-bit I2C IO expander that provides:

- 8 GPIO pins that can be individually configured as inputs or outputs
- Polarity inversion for input pins
- I2C interface with configurable address (0x20-0x27)
- Low power consumption
- 3.3V and 5V operation

## Usage

### Dependencies

```toml
[dependencies]
# Default - includes full async support (async + embedded-hal + embedded-hal-async)
tca9534 = "0.1"

# Minimal - no external dependencies, sync only
tca9534 = { version = "0.1", default-features = false }

# Sync with embedded-hal support only
tca9534 = { version = "0.1", default-features = false, features = ["embedded-hal"] }

# Async only (no embedded-hal)
tca9534 = { version = "0.1", default-features = false, features = ["async"] }

# Custom combinations
tca9534 = { version = "0.1", default-features = false, features = ["async", "embedded-hal"] }

# With defmt logging support
tca9534 = { version = "0.1", features = ["defmt"] }
```

#### Available Features

- **`async`** - Enables async/await support for TCA9534Async
- **`embedded-hal`** - Enables embedded-hal v1.0 I2C trait integration
- **`embedded-hal-async`** - Enables embedded-hal-async I2C trait integration  
- **`full-async`** - Combines `async` + `embedded-hal` + `embedded-hal-async` (included in default)
- **`defmt`** - Enables defmt logging support

**Default features**: `["full-async"]` - provides complete async functionality out of the box.

### Basic Example (Synchronous)

**Note**: For sync-only usage, disable default features in your Cargo.toml:
```toml
tca9534 = { version = "0.1", default-features = false, features = ["embedded-hal"] }
```

```rust
use tca9534::{TCA9534Sync, PinConfig, PinLevel, addresses};

// Initialize I2C bus (platform specific)
let i2c = setup_i2c(); // Your I2C initialization code

// Create TCA9534 driver with address 0x20
let mut tca9534 = TCA9534Sync::new(i2c, addresses::ADDR_000);

// Or use default address constructor  
let mut tca9534 = TCA9534Sync::new_with_default_address(i2c);

//Or use transport which implements SyncTransport
let transport = MyI2c::new(your_own_i2c);
let mut tca9534 = TCA9534Sync::new(transport, addresses::ADDR_000);

// Initialize the device
tca9534.init()?;

// Configure pin 0 as output, others as input
tca9534.set_pin_config(0, PinConfig::Output)?;
for pin in 1..8 {
    tca9534.set_pin_config(pin, PinConfig::Input)?;
}

// Set pin 0 to high
tca9534.set_pin_output(0, PinLevel::High)?;

// Read pin 1 input
let pin1_level = tca9534.read_pin_input(1)?;

// Toggle pin 0
tca9534.toggle_pin_output(0)?;
```

### Custom Transport Example (No Dependencies)

**Note**: For zero dependencies, use minimal mode:
```toml
tca9534 = { version = "0.1", default-features = false }
```

```rust
use tca9534::{TCA9534Sync, PinConfig, PinLevel, addresses, SyncTransport};

// Implement your own transport
struct MyI2cTransport {
    // Your I2C implementation
}

impl SyncTransport for MyI2cTransport {
    type Error = MyError;
    
    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        // Your I2C write implementation
    }
    
    fn read(&mut self, addr: u8, bytes: &mut [u8]) -> Result<(), Self::Error> {
        // Your I2C read implementation
    }
    
    fn write_read(&mut self, addr: u8, wr_bytes: &[u8], rd_bytes: &mut [u8]) -> Result<(), Self::Error> {
        // Your I2C write_read implementation
    }
}

// Use with custom transport
let transport = MyI2cTransport::new();
let mut tca9534 = TCA9534Sync::new(transport, addresses::ADDR_000);
```

### Async Example

**Note**: Async support is enabled by default with `full-async` feature.

```rust
use tca9534::{TCA9534Async, PinConfig, PinLevel, addresses};

// Initialize async I2C bus (platform specific)
let i2c = setup_async_i2c(); // Your async I2C initialization code

// Create TCA9534 driver
let mut tca9534 = TCA9534Async::new(i2c, addresses::ADDR_000);

// Initialize the device
tca9534.init().await?;

// Configure and use pins
tca9534.set_pin_config(0, PinConfig::Output).await?;
tca9534.set_pin_output(0, PinLevel::High).await?;

let input_level = tca9534.read_pin_input(1).await?;
```

### Port-wide Operations

```rust
// Configure all pins at once (1=input, 0=output)
tca9534.set_port_config(0b11110000)?; // Pins 0-3 output, 4-7 input

// Set all output pins at once
tca9534.write_output_port(0b00001111)?; // Set pins 0-3 high

// Read all input pins at once
let input_state = tca9534.read_input_port()?;
```

## API Overview

### Core Functions

- `new(transport, address)` - Create new driver instance
- `init()` - Initialize device with default settings
- `set_pin_config(pin, config)` - Configure pin as input or output
- `set_pin_output(pin, level)` - Set output pin high or low
- `read_pin_input(pin)` - Read input pin level
- `toggle_pin_output(pin)` - Toggle output pin state

### Port-wide Operations

- `set_port_config(config)` - Configure all pins at once
- `write_output_port(value)` - Set all output pins at once
- `read_input_port()` - Read all input pins at once
- `read_output_port()` - Read current output register value

### Advanced Features

- `set_pin_polarity(pin, polarity)` - Set input polarity (normal/inverted)
- `set_port_polarity(polarity)` - Set polarity for all pins
- `address()` / `set_address(addr)` - Get/set I2C address

## Register Map

| Register | Address | Description |
|----------|---------|-------------|
| Input Port | 0x00 | Read input pin levels |
| Output Port | 0x01 | Set output pin levels |
| Polarity Inversion | 0x02 | Configure input polarity |
| Configuration | 0x03 | Configure pin directions |

## I2C Addresses

The TCA9534 supports 8 different I2C addresses based on the A2, A1, A0 pins:

| A2 | A1 | A0 | Address | Constant |
|----|----|----|---------|----------|
| 0  | 0  | 0  | 0x20    | `addresses::ADDR_000` |
| 0  | 0  | 1  | 0x21    | `addresses::ADDR_001` |
| 0  | 1  | 0  | 0x22    | `addresses::ADDR_010` |
| 0  | 1  | 1  | 0x23    | `addresses::ADDR_011` |
| 1  | 0  | 0  | 0x24    | `addresses::ADDR_100` |
| 1  | 0  | 1  | 0x25    | `addresses::ADDR_101` |
| 1  | 1  | 0  | 0x26    | `addresses::ADDR_110` |
| 1  | 1  | 1  | 0x27    | `addresses::ADDR_111` |

## Error Handling

The driver provides minimal error handling focused on essential validation:

- **`InvalidPin`** - Pin number out of range (must be 0-7)
- **`I2cError(E)`** - Underlying I2C transport error

Additional error types can be added as needed for your specific use case.

## Platform Support

### With embedded-hal Integration

This driver works with any platform that implements the `embedded-hal` I2C traits:

- **STM32** (via `stm32hal` family crates)
- **ESP32** (via `esp-hal`)
- **Raspberry Pi Pico** (via `rp2040-hal`)
- **Arduino-style boards** (via `arduino-hal`)
- **Linux** (via `linux-embedded-hal`)
- And many more!

### Without Dependencies

You can use this driver on any platform by implementing the `SyncTransport` or `AsyncTransport` traits for your I2C implementation.

## Features

- `embedded-hal` - Enable embedded-hal I2C trait integration
- `embedded-hal-async` - Enable embedded-hal async I2C trait integration
- `async` - Enable async/await support (requires async transport)
- `defmt` - Enable defmt logging support

## Examples

See the [`examples/`](examples/) directory for complete examples:

- [`basic_usage`](examples/basic_usage/) - Complete example using STM32G431 with embassy-rs

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## References

- [TCA9534 Datasheet](https://www.ti.com/lit/ds/symlink/tca9534.pdf)
- [embedded-hal Documentation](https://docs.rs/embedded-hal) 
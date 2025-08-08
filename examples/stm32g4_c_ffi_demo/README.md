# TCA9534 STM32 C FFI Demo

This project demonstrates how to integrate the TCA9534 I2C IO expander driver (written in Rust) into an STM32 project using C FFI.

## Hardware Requirements

- **STM32G431CB** microcontroller (Cortex-M4F with FPU)
- **TCA9534** I2C IO expander
- **I2C connection**: SCL, SDA, VCC, GND

## Software Architecture

```
STM32 Application (main.c)
    ↓
STM32 Convenience Layer (tca9534_stm32.c/h)
    ↓
TCA9534 C FFI API (tca9534.h)
    ↓
TCA9534 Rust Driver (libtca9534.a)
    ↓
STM32 HAL I2C
```

## File Structure

```
tca_ffi_demo/
├── Core/
│   ├── Inc/
│   │   ├── tca9534.h           # TCA9534 C FFI header
│   │   └── tca9534_stm32.h     # STM32 convenience functions
│   ├── Src/
│   │   ├── main.c              # Main application
│   │   ├── tca9534_stm32.c     # STM32 HAL integration
│   │   └── libtca9534.a        # Compiled Rust library !! Need to configure the link path of libtca9534.a in cubeide
└── README.md
```

## Build Configuration

### 1. Enable staticlib output

Edit `Cargo.toml` in the TCA9534 driver directory and uncomment the `crate-type` line:

```toml
[lib]
name = "tca9534"
crate-type = ["staticlib"] # use when build with capi feature
```

### 2. Compile Rust Library

```bash
# In the TCA9534 driver directory
cargo build --release --target thumbv7em-none-eabihf --features capi
```

**Important**: 
- Use `thumbv7em-none-eabihf` target for STM32G431CB (hard float support)
- Remember to comment out the `crate-type` line after building if you want to use the library as a normal Rust dependency

### 3. STM32CubeIDE Configuration

1. **I2C Setup**: Enable I2C1 peripheral, set to 100kHz
2. **Linker**: Add `libtca9534.a` to project and configure library path
3. **Include Paths**: Add `../Core/Inc` to include path


## Demo Features

The demo application shows:
- **LED Chase Pattern**: Animated sequence on pins 0-3
- **Input Monitoring**: Reading switches on pins 4-7
- **Speed Control**: Inputs affect pattern speed

## Common Issues

1. **Compilation Errors**: Ensure correct ARM target (`thumbv7em-none-eabihf`)
2. **Linking Errors**: Check `libtca9534.a` path in project settings
3. **Runtime Errors**: Verify I2C connections and TCA9534 power supply (3.3V)

## Error Codes

- `TCA9534_OK`: Success
- `TCA9534_ERROR_INVALID_PIN`: Invalid pin number (0-7)
- `TCA9534_ERROR_I2C_WRITE/READ`: I2C communication failed
- `TCA9534_ERROR_NULL_PTR`: Null pointer
- `TCA9534_ERROR_INIT_FAILED`: Initialization failed

---

This demo shows how to integrate Rust drivers into STM32 C projects using FFI. 
//! TCA9534 I2C IO Expander driver
//!
//! This is a platform-independent Rust driver for the TCA9534, an 8-bit I2C
//! IO expander, with optional [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) integration.
//!
//! Both synchronous and asynchronous versions are provided, selectable via feature flags.
//!
//! ## Example Usage
//!
//! ### Synchronous Usage
//!
//! ```rust,ignore
//! use tca9534::{Tca9534Sync, PinConfig, PinLevel, addresses};
//!
//! // Initialize I2C bus (platform specific)
//! let i2c = setup_i2c(); // Your I2C initialization code
//!
//! // Create TCA9534 driver with address 0x20 and initialize the device
//! let mut tca9534 = Tca9534Sync::new(i2c, addresses::ADDR_000)?;
//!
//! // Or use default address constructor  
//! let mut tca9534 = Tca9534Sync::with_default_address(i2c)?;
//!
//! // Configure pin 0 as output, others as input
//! tca9534.set_pin_config(0, PinConfig::Output)?;
//! tca9534.set_pin_config(1, PinConfig::Input)?;
//!
//! // Set pin 0 to high
//! tca9534.set_pin_output(0, PinLevel::High)?;
//!
//! // Read pin 1 input
//! let pin1_level = tca9534.read_pin_input(1)?;
//! ```
//!
//! ### Asynchronous Usage (with async feature)
//!
//! ```rust,ignore
//! use tca9534::{Tca9534Async, PinConfig, PinLevel, addresses};
//!
//! // Initialize async I2C bus (platform specific)
//! let i2c = setup_async_i2c(); // Your async I2C initialization code
//!
//! // Create TCA9534 driver with address 0x20 and initialize the device
//! let mut tca9534 = Tca9534Async::new(i2c, addresses::ADDR_000).await?;
//!
//! // Configure and use pins
//! tca9534.set_pin_config(0, PinConfig::Output).await?;
//! tca9534.set_pin_output(0, PinLevel::High).await?;
//!
//! let input_level = tca9534.read_pin_input(1).await?;
//! ```



#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]


mod error;
mod registers;
mod transport;

// TCA9534 driver implementations
mod tca9534;

// C FFI module (feature-gated)
#[cfg(feature = "capi")]
pub mod ffi;

// Re-export common types

pub use error::{Tca9534CoreError, Tca9534Error};
pub use registers::*;
pub use transport::SyncTransport;

#[cfg(feature = "async")]
pub use transport::AsyncTransport;

// Re-export driver implementations from tca9534 module

pub use tca9534::Tca9534Sync;

#[cfg(feature = "async")]
pub use tca9534::Tca9534Async;

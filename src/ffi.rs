//! C FFI (Foreign Function Interface) module for TCA9534 driver
//!
//! This module provides a C-compatible interface for the TCA9534 driver library.
//! It implements a transport layer adapter that converts C function pointers to
//! the Rust SyncTransport trait, along with all the necessary FFI functions.

use crate::error::Tca9534CoreError;
use crate::registers::{PinConfig, PinLevel, PinPolarity, Register};
use crate::tca9534::Tca9534Sync;
use crate::transport::SyncTransport;

use core::ffi::c_void;

// =============================================================================
// Panic handler for no_std environment
// =============================================================================

#[cfg(feature = "capi")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // In a real implementation, you might want to log the panic or take other actions
    // For now, we'll just loop forever
    loop {}
}

// =============================================================================
// C-compatible types and constants
// =============================================================================

/// C-compatible I2C operations function pointers
#[repr(C)]
pub struct CI2cOps {
    pub write: Option<unsafe extern "C" fn(*mut c_void, u8, *const u8, usize) -> i32>,
    pub read: Option<unsafe extern "C" fn(*mut c_void, u8, *mut u8, usize) -> i32>,
    pub write_read: Option<unsafe extern "C" fn(*mut c_void, u8, *const u8, usize, *mut u8, usize) -> i32>,
}

/// C-compatible device handle
#[repr(C)]
pub struct CHandle {
    pub address: u8,
    pub transport_ctx: *mut c_void,
    pub ops: *mut CI2cOps,
}

/// C-compatible error codes
#[repr(C)]
pub enum CError {
    Ok = 0,
    InvalidPin = -1,
    I2cWrite = -2,
    I2cRead = -3,
    NullPtr = -4,
    InitFailed = -5,
}

/// C-compatible pin configuration
#[repr(C)]
pub enum CPinConfig {
    Input = 1,
    Output = 0,
}

/// C-compatible pin level
#[repr(C)]
pub enum CPinLevel {
    Low = 0,
    High = 1,
}

/// C-compatible pin polarity
#[repr(C)]
pub enum CPinPolarity {
    Normal = 0,
    Inverted = 1,
}

// =============================================================================
// Transport adapter implementation
// =============================================================================

/// Transport adapter that wraps C function pointers
pub struct CTransportAdapter {
    ctx: *mut c_void,
    ops: *mut CI2cOps,
}

impl CTransportAdapter {
    /// Create a new transport adapter from C function pointers
    pub fn new(ctx: *mut c_void, ops: *mut CI2cOps) -> Self {
        Self { ctx, ops }
    }
    
    /// Validate that all required function pointers are present
    fn validate_ops(&self) -> Result<(), CError> {
        if self.ops.is_null() {
            return Err(CError::NullPtr);
        }
        
        let ops = unsafe { &*self.ops };
        
        if ops.write.is_none() || ops.read.is_none() || ops.write_read.is_none() {
            return Err(CError::NullPtr);
        }
        
        Ok(())
    }
}

impl SyncTransport for CTransportAdapter {
    type Error = CError;
    
    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.validate_ops()?;
        
        let ops = unsafe { &*self.ops };
        let write_fn = ops.write.unwrap();
        
        let result = unsafe {
            write_fn(self.ctx, addr, bytes.as_ptr(), bytes.len())
        };
        
        if result == 0 {
            Ok(())
        } else {
            Err(CError::I2cWrite)
        }
    }
    
    fn read(&mut self, addr: u8, bytes: &mut [u8]) -> Result<(), Self::Error> {
        self.validate_ops()?;
        
        let ops = unsafe { &*self.ops };
        let read_fn = ops.read.unwrap();
        
        let result = unsafe {
            read_fn(self.ctx, addr, bytes.as_mut_ptr(), bytes.len())
        };
        
        if result == 0 {
            Ok(())
        } else {
            Err(CError::I2cRead)
        }
    }
    
    fn write_read(&mut self, addr: u8, wr_bytes: &[u8], rd_bytes: &mut [u8]) -> Result<(), Self::Error> {
        self.validate_ops()?;
        
        let ops = unsafe { &*self.ops };
        let write_read_fn = ops.write_read.unwrap();
        
        let result = unsafe {
            write_read_fn(
                self.ctx,
                addr,
                wr_bytes.as_ptr(),
                wr_bytes.len(),
                rd_bytes.as_mut_ptr(),
                rd_bytes.len(),
            )
        };
        
        if result == 0 {
            Ok(())
        } else {
            Err(CError::I2cRead)
        }
    }
}

// =============================================================================
// Error conversion functions
// =============================================================================

// Note: rust_error_to_c_error function removed as it was unused

/// Convert TCA9534 core errors to C error codes
fn core_error_to_c_error(error: Tca9534CoreError) -> CError {
    match error {
        Tca9534CoreError::InvalidPin => CError::InvalidPin,
    }
}

/// Implement From trait for CError from Tca9534CoreError
impl From<Tca9534CoreError> for CError {
    fn from(error: Tca9534CoreError) -> Self {
        core_error_to_c_error(error)
    }
}

// =============================================================================
// Type conversion functions
// =============================================================================

/// Convert C pin configuration to Rust pin configuration
fn c_pin_config_to_rust(config: CPinConfig) -> PinConfig {
    match config {
        CPinConfig::Input => PinConfig::Input,
        CPinConfig::Output => PinConfig::Output,
    }
}

/// Convert C pin level to Rust pin level
fn c_pin_level_to_rust(level: CPinLevel) -> PinLevel {
    match level {
        CPinLevel::Low => PinLevel::Low,
        CPinLevel::High => PinLevel::High,
    }
}

/// Convert Rust pin level to C pin level
fn rust_pin_level_to_c(level: PinLevel) -> CPinLevel {
    match level {
        PinLevel::Low => CPinLevel::Low,
        PinLevel::High => CPinLevel::High,
    }
}

/// Convert C pin polarity to Rust pin polarity
fn c_pin_polarity_to_rust(polarity: CPinPolarity) -> PinPolarity {
    match polarity {
        CPinPolarity::Normal => PinPolarity::Normal,
        CPinPolarity::Inverted => PinPolarity::Inverted,
    }
}

// =============================================================================
// FFI functions implementation
// =============================================================================

/// Type alias for the internal driver with C transport adapter
type CDriverType = Tca9534Sync<CTransportAdapter>;

/// Internal storage for driver instances
/// Note: In a real implementation, you might want to use a more sophisticated
/// storage mechanism, but for simplicity, we'll store the driver inside the handle
#[repr(C)]
pub struct InternalHandle {
    pub c_handle: CHandle,
    pub driver: Option<CDriverType>,
}

// =============================================================================
// Public C FFI functions
// =============================================================================

/// Initialize TCA9534 device
#[no_mangle]
pub unsafe extern "C" fn tca9534_init(
    handle: *mut CHandle,
    address: u8,
    transport_ctx: *mut c_void,
    ops: *mut CI2cOps,
) -> CError {
    // Validate input parameters
    if handle.is_null() || ops.is_null() {
        return CError::NullPtr;
    }
    
    // Initialize the C handle
    let c_handle = &mut *handle;
    c_handle.address = address;
    c_handle.transport_ctx = transport_ctx;
    c_handle.ops = ops;
    
    // Create transport adapter
    let transport = CTransportAdapter::new(transport_ctx, ops);
    
    // Create and initialize the driver
    match CDriverType::new(transport, address) {
        Ok(_driver) => {
            // Store the driver in the handle (this is a simplified approach)
            // In a real implementation, you'd want a more sophisticated storage mechanism
            CError::Ok
        }
        Err(_err) => CError::InitFailed,
    }
}

/// Initialize TCA9534 device with default address
#[no_mangle]
pub unsafe extern "C" fn tca9534_init_default(
    handle: *mut CHandle,
    transport_ctx: *mut c_void,
    ops: *mut CI2cOps,
) -> CError {
    tca9534_init(handle, 0x20, transport_ctx, ops)
}

/// Read a register value
#[no_mangle]
pub unsafe extern "C" fn tca9534_read_register(
    handle: *mut CHandle,
    reg_addr: u8,
    value: *mut u8,
) -> CError {
    if handle.is_null() || value.is_null() {
        return CError::NullPtr;
    }
    
    let c_handle = &mut *handle;
    let transport = CTransportAdapter::new(c_handle.transport_ctx, c_handle.ops);
    
    match CDriverType::new(transport, c_handle.address) {
        Ok(mut driver) => {
            let register = match reg_addr {
                0x00 => Register::InputPort,
                0x01 => Register::OutputPort,
                0x02 => Register::Polarity,
                0x03 => Register::Config,
                _ => return CError::InvalidPin,
            };
            
            match driver.read_register(register) {
                Ok(val) => {
                    *value = val;
                    CError::Ok
                }
                Err(_) => CError::I2cRead,
            }
        }
        Err(_) => CError::InitFailed,
    }
}

/// Write a register value
#[no_mangle]
pub unsafe extern "C" fn tca9534_write_register(
    handle: *mut CHandle,
    reg_addr: u8,
    value: u8,
) -> CError {
    if handle.is_null() {
        return CError::NullPtr;
    }
    
    let c_handle = &mut *handle;
    let transport = CTransportAdapter::new(c_handle.transport_ctx, c_handle.ops);
    
    match CDriverType::new(transport, c_handle.address) {
        Ok(mut driver) => {
            let register = match reg_addr {
                0x00 => Register::InputPort,
                0x01 => Register::OutputPort,
                0x02 => Register::Polarity,
                0x03 => Register::Config,
                _ => return CError::InvalidPin,
            };
            
            match driver.write_register(register, value) {
                Ok(_) => CError::Ok,
                Err(_) => CError::I2cWrite,
            }
        }
        Err(_) => CError::InitFailed,
    }
}

/// Read input port (all 8 pins at once)
#[no_mangle]
pub unsafe extern "C" fn tca9534_read_input_port(
    handle: *mut CHandle,
    port_value: *mut u8,
) -> CError {
    if handle.is_null() || port_value.is_null() {
        return CError::NullPtr;
    }
    
    let c_handle = &mut *handle;
    let transport = CTransportAdapter::new(c_handle.transport_ctx, c_handle.ops);
    
    match CDriverType::new(transport, c_handle.address) {
        Ok(mut driver) => {
            match driver.read_input_port() {
                Ok(val) => {
                    *port_value = val;
                    CError::Ok
                }
                Err(_) => CError::I2cRead,
            }
        }
        Err(_) => CError::InitFailed,
    }
}

/// Write output port (all 8 pins at once)
#[no_mangle]
pub unsafe extern "C" fn tca9534_write_output_port(
    handle: *mut CHandle,
    port_value: u8,
) -> CError {
    if handle.is_null() {
        return CError::NullPtr;
    }
    
    let c_handle = &mut *handle;
    let transport = CTransportAdapter::new(c_handle.transport_ctx, c_handle.ops);
    
    match CDriverType::new(transport, c_handle.address) {
        Ok(mut driver) => {
            match driver.write_output_port(port_value) {
                Ok(_) => CError::Ok,
                Err(_) => CError::I2cWrite,
            }
        }
        Err(_) => CError::InitFailed,
    }
}

/// Read current output port register value
#[no_mangle]
pub unsafe extern "C" fn tca9534_read_output_port(
    handle: *mut CHandle,
    port_value: *mut u8,
) -> CError {
    if handle.is_null() || port_value.is_null() {
        return CError::NullPtr;
    }
    
    let c_handle = &mut *handle;
    let transport = CTransportAdapter::new(c_handle.transport_ctx, c_handle.ops);
    
    match CDriverType::new(transport, c_handle.address) {
        Ok(mut driver) => {
            match driver.read_output_port() {
                Ok(val) => {
                    *port_value = val;
                    CError::Ok
                }
                Err(_) => CError::I2cRead,
            }
        }
        Err(_) => CError::InitFailed,
    }
}

/// Read a single pin input
#[no_mangle]
pub unsafe extern "C" fn tca9534_read_pin_input(
    handle: *mut CHandle,
    pin: u8,
    level: *mut CPinLevel,
) -> CError {
    if handle.is_null() || level.is_null() {
        return CError::NullPtr;
    }
    
    if pin > 7 {
        return CError::InvalidPin;
    }
    
    let c_handle = &mut *handle;
    let transport = CTransportAdapter::new(c_handle.transport_ctx, c_handle.ops);
    
    match CDriverType::new(transport, c_handle.address) {
        Ok(mut driver) => {
            match driver.read_pin_input(pin) {
                Ok(pin_level) => {
                    *level = rust_pin_level_to_c(pin_level);
                    CError::Ok
                }
                Err(_) => CError::I2cRead,
            }
        }
        Err(_) => CError::InitFailed,
    }
}

/// Set a single pin output
#[no_mangle]
pub unsafe extern "C" fn tca9534_set_pin_output(
    handle: *mut CHandle,
    pin: u8,
    level: CPinLevel,
) -> CError {
    if handle.is_null() {
        return CError::NullPtr;
    }
    
    if pin > 7 {
        return CError::InvalidPin;
    }
    
    let c_handle = &mut *handle;
    let transport = CTransportAdapter::new(c_handle.transport_ctx, c_handle.ops);
    
    match CDriverType::new(transport, c_handle.address) {
        Ok(mut driver) => {
            let rust_level = c_pin_level_to_rust(level);
            match driver.set_pin_output(pin, rust_level) {
                Ok(_) => CError::Ok,
                Err(_) => CError::I2cWrite,
            }
        }
        Err(_) => CError::InitFailed,
    }
}

/// Toggle a single pin output
#[no_mangle]
pub unsafe extern "C" fn tca9534_toggle_pin_output(
    handle: *mut CHandle,
    pin: u8,
) -> CError {
    if handle.is_null() {
        return CError::NullPtr;
    }
    
    if pin > 7 {
        return CError::InvalidPin;
    }
    
    let c_handle = &mut *handle;
    let transport = CTransportAdapter::new(c_handle.transport_ctx, c_handle.ops);
    
    match CDriverType::new(transport, c_handle.address) {
        Ok(mut driver) => {
            match driver.toggle_pin_output(pin) {
                Ok(_) => CError::Ok,
                Err(_) => CError::I2cWrite,
            }
        }
        Err(_) => CError::InitFailed,
    }
}

/// Configure a single pin direction
#[no_mangle]
pub unsafe extern "C" fn tca9534_set_pin_config(
    handle: *mut CHandle,
    pin: u8,
    config: CPinConfig,
) -> CError {
    if handle.is_null() {
        return CError::NullPtr;
    }
    
    if pin > 7 {
        return CError::InvalidPin;
    }
    
    let c_handle = &mut *handle;
    let transport = CTransportAdapter::new(c_handle.transport_ctx, c_handle.ops);
    
    match CDriverType::new(transport, c_handle.address) {
        Ok(mut driver) => {
            let rust_config = c_pin_config_to_rust(config);
            match driver.set_pin_config(pin, rust_config) {
                Ok(_) => CError::Ok,
                Err(_) => CError::I2cWrite,
            }
        }
        Err(_) => CError::InitFailed,
    }
}

/// Configure all pins direction at once
#[no_mangle]
pub unsafe extern "C" fn tca9534_set_port_config(
    handle: *mut CHandle,
    config: u8,
) -> CError {
    if handle.is_null() {
        return CError::NullPtr;
    }
    
    let c_handle = &mut *handle;
    let transport = CTransportAdapter::new(c_handle.transport_ctx, c_handle.ops);
    
    match CDriverType::new(transport, c_handle.address) {
        Ok(mut driver) => {
            match driver.set_port_config(config) {
                Ok(_) => CError::Ok,
                Err(_) => CError::I2cWrite,
            }
        }
        Err(_) => CError::InitFailed,
    }
}

/// Read port configuration
#[no_mangle]
pub unsafe extern "C" fn tca9534_read_port_config(
    handle: *mut CHandle,
    config: *mut u8,
) -> CError {
    if handle.is_null() || config.is_null() {
        return CError::NullPtr;
    }
    
    let c_handle = &mut *handle;
    let transport = CTransportAdapter::new(c_handle.transport_ctx, c_handle.ops);
    
    match CDriverType::new(transport, c_handle.address) {
        Ok(mut driver) => {
            match driver.read_port_config() {
                Ok(val) => {
                    *config = val;
                    CError::Ok
                }
                Err(_) => CError::I2cRead,
            }
        }
        Err(_) => CError::InitFailed,
    }
}

/// Configure a single pin polarity
#[no_mangle]
pub unsafe extern "C" fn tca9534_set_pin_polarity(
    handle: *mut CHandle,
    pin: u8,
    polarity: CPinPolarity,
) -> CError {
    if handle.is_null() {
        return CError::NullPtr;
    }
    
    if pin > 7 {
        return CError::InvalidPin;
    }
    
    let c_handle = &mut *handle;
    let transport = CTransportAdapter::new(c_handle.transport_ctx, c_handle.ops);
    
    match CDriverType::new(transport, c_handle.address) {
        Ok(mut driver) => {
            let rust_polarity = c_pin_polarity_to_rust(polarity);
            match driver.set_pin_polarity(pin, rust_polarity) {
                Ok(_) => CError::Ok,
                Err(_) => CError::I2cWrite,
            }
        }
        Err(_) => CError::InitFailed,
    }
}

/// Configure all pins polarity at once
#[no_mangle]
pub unsafe extern "C" fn tca9534_set_port_polarity(
    handle: *mut CHandle,
    polarity: u8,
) -> CError {
    if handle.is_null() {
        return CError::NullPtr;
    }
    
    let c_handle = &mut *handle;
    let transport = CTransportAdapter::new(c_handle.transport_ctx, c_handle.ops);
    
    match CDriverType::new(transport, c_handle.address) {
        Ok(mut driver) => {
            match driver.set_port_polarity(polarity) {
                Ok(_) => CError::Ok,
                Err(_) => CError::I2cWrite,
            }
        }
        Err(_) => CError::InitFailed,
    }
}

/// Read port polarity configuration
#[no_mangle]
pub unsafe extern "C" fn tca9534_read_port_polarity(
    handle: *mut CHandle,
    polarity: *mut u8,
) -> CError {
    if handle.is_null() || polarity.is_null() {
        return CError::NullPtr;
    }
    
    let c_handle = &mut *handle;
    let transport = CTransportAdapter::new(c_handle.transport_ctx, c_handle.ops);
    
    match CDriverType::new(transport, c_handle.address) {
        Ok(mut driver) => {
            match driver.read_port_polarity() {
                Ok(val) => {
                    *polarity = val;
                    CError::Ok
                }
                Err(_) => CError::I2cRead,
            }
        }
        Err(_) => CError::InitFailed,
    }
}

/// Set I2C address (useful for multiple devices)
#[no_mangle]
pub unsafe extern "C" fn tca9534_set_address(
    handle: *mut CHandle,
    address: u8,
) {
    if !handle.is_null() {
        let c_handle = &mut *handle;
        c_handle.address = address;
    }
}

/// Get current I2C address
#[no_mangle]
pub unsafe extern "C" fn tca9534_get_address(
    handle: *const CHandle,
) -> u8 {
    if handle.is_null() {
        return 0;
    }
    
    let c_handle = &*handle;
    c_handle.address
} 
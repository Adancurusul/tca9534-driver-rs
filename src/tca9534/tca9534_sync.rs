use crate::error::*;
use crate::registers::*;
use crate::transport::SyncTransport;

/// TCA9534 synchronous driver structure
#[derive(Debug)]
pub struct Tca9534<T> {
    transport: T,
    address: u8,
}

/// Synchronous implementation
impl<T> Tca9534<T>
where
    T: SyncTransport,
{
    /// Create a new TCA9534 driver instance
    pub fn new(transport: T, address: u8) -> Result<Self, T::Error> {
        let mut ans = Self { transport, address };
        ans.init()?;
        Ok(ans)
    }

    /// Create a new TCA9534 driver instance with default address
    pub fn with_default_address(transport: T) -> Result<Self, T::Error> {
        let mut ans = Self {
            transport,
            address: addresses::ADDR_000,
        };
        ans.init()?;
        Ok(ans)
    }

    /// Set I2C address (useful for multiple devices)
    pub fn set_address(&mut self, address: u8) {
        self.address = address;
    }

    /// Get current I2C address
    pub fn address(&self) -> u8 {
        self.address
    }

    /// Initialize the device with default settings
    fn init(&mut self) -> Result<(), T::Error> {
        // Set all pins as inputs (default state)
        self.write_register(Register::Config, 0xFF)?;

        // Set all outputs to low (when configured as outputs)
        self.write_register(Register::OutputPort, 0x00)?;

        // Set all polarities to normal (non-inverted)
        self.write_register(Register::Polarity, 0x00)?;

        Ok(())
    }

    /// Read a register
    pub fn read_register(&mut self, reg: Register) -> Result<u8, T::Error> {
        let mut buffer = [0u8; 1];
        self.transport
            .write_read(self.address, &[reg.addr()], &mut buffer)?;
        Ok(buffer[0])
    }

    /// Write to a register
    pub fn write_register(&mut self, reg: Register, value: u8) -> Result<(), T::Error> {
        self.transport.write(self.address, &[reg.addr(), value])
    }

    /// Read all input pins at once
    pub fn read_input_port(&mut self) -> Result<u8, T::Error> {
        self.read_register(Register::InputPort)
    }

    /// Read a specific input pin
    pub fn read_pin_input(&mut self, pin: u8) -> Result<PinLevel, T::Error>
    where
        T::Error: From<Tca9534CoreError>,
    {
        if pin > 7 {
            return Err(Tca9534CoreError::InvalidPin.into());
        }

        let port_value = self.read_input_port()?;
        let pin_value = (port_value >> pin) & 0x01;
        Ok(if pin_value == 0 {
            PinLevel::Low
        } else {
            PinLevel::High
        })
    }

    /// Write all output pins at once
    pub fn write_output_port(&mut self, value: u8) -> Result<(), T::Error> {
        self.write_register(Register::OutputPort, value)
    }

    /// Read current output port register value
    pub fn read_output_port(&mut self) -> Result<u8, T::Error> {
        self.read_register(Register::OutputPort)
    }

    /// Set a specific output pin
    pub fn set_pin_output(&mut self, pin: u8, level: PinLevel) -> Result<(), T::Error>
    where
        T::Error: From<Tca9534CoreError>,
    {
        if pin > 7 {
            return Err(Tca9534CoreError::InvalidPin.into());
        }

        let mut current_value = self.read_output_port()?;
        match level {
            PinLevel::High => current_value |= 1 << pin,
            PinLevel::Low => current_value &= !(1 << pin),
        }
        self.write_output_port(current_value)
    }

    /// Toggle a specific output pin
    pub fn toggle_pin_output(&mut self, pin: u8) -> Result<(), T::Error>
    where
        T::Error: From<Tca9534CoreError>,
    {
        if pin > 7 {
            return Err(Tca9534CoreError::InvalidPin.into());
        }

        let mut current_value = self.read_output_port()?;
        current_value ^= 1 << pin;
        self.write_output_port(current_value)
    }

    /// Configure pin direction (input/output)
    pub fn set_pin_config(&mut self, pin: u8, config: PinConfig) -> Result<(), T::Error>
    where
        T::Error: From<Tca9534CoreError>,
    {
        if pin > 7 {
            return Err(Tca9534CoreError::InvalidPin.into());
        }

        let mut current_config = self.read_register(Register::Config)?;
        match config {
            PinConfig::Input => current_config |= 1 << pin,
            PinConfig::Output => current_config &= !(1 << pin),
        }
        self.write_register(Register::Config, current_config)
    }

    /// Configure all pins direction at once
    pub fn set_port_config(&mut self, config: u8) -> Result<(), T::Error> {
        self.write_register(Register::Config, config)
    }

    /// Read port configuration
    pub fn read_port_config(&mut self) -> Result<u8, T::Error> {
        self.read_register(Register::Config)
    }

    /// Set pin polarity (normal/inverted)
    pub fn set_pin_polarity(&mut self, pin: u8, polarity: PinPolarity) -> Result<(), T::Error>
    where
        T::Error: From<Tca9534CoreError>,
    {
        if pin > 7 {
            return Err(Tca9534CoreError::InvalidPin.into());
        }

        let mut current_polarity = self.read_register(Register::Polarity)?;
        match polarity {
            PinPolarity::Normal => current_polarity &= !(1 << pin),
            PinPolarity::Inverted => current_polarity |= 1 << pin,
        }
        self.write_register(Register::Polarity, current_polarity)
    }

    /// Configure all pins polarity at once
    pub fn set_port_polarity(&mut self, polarity: u8) -> Result<(), T::Error> {
        self.write_register(Register::Polarity, polarity)
    }

    /// Read port polarity configuration
    pub fn read_port_polarity(&mut self) -> Result<u8, T::Error> {
        self.read_register(Register::Polarity)
    }
}

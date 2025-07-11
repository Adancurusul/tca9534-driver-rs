/// Core TCA9534 errors that don't depend on transport.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tca9534CoreError {
    /// Invalid pin number (must be 0-7)
    InvalidPin,
    // /// Invalid register address
    // InvalidRegister,
    // /// Device initialization failed
    // InitializationFailed,
    // /// Operation timeout
    // Timeout,
    // /// Device not responding on I2C bus
    // DeviceNotResponding,
    // /// Invalid state or configuration
    // InvalidState,
}

/// TCA9534 driver error type.
#[derive(Debug)]
pub enum Tca9534Error<I2cE = ()> {
    /// Core TCA9534 error
    Core(Tca9534CoreError),
    /// I2C communication error
    I2c(I2cE),
}

impl<I2cE> From<Tca9534CoreError> for Tca9534Error<I2cE> {
    fn from(err: Tca9534CoreError) -> Self {
        Tca9534Error::Core(err)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Tca9534CoreError {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            Self::InvalidPin => defmt::write!(fmt, "InvalidPin"),
            // Self::InvalidRegister => defmt::write!(fmt, "InvalidRegister"),
            // Self::InitializationFailed => defmt::write!(fmt, "InitializationFailed"),
            // Self::Timeout => defmt::write!(fmt, "Timeout"),
            // Self::DeviceNotResponding => defmt::write!(fmt, "DeviceNotResponding"),
            // Self::InvalidState => defmt::write!(fmt, "InvalidState"),
        }
    }
}

#[cfg(feature = "defmt")]
impl<I2cE> defmt::Format for Tca9534Error<I2cE> {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            Self::Core(core_err) => defmt::write!(fmt, "Core({})", core_err),
            Self::I2c(_) => defmt::write!(fmt, "I2cError"),
        }
    }
}

impl core::fmt::Display for Tca9534CoreError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidPin => write!(f, "Invalid pin number (must be 0-7)"),
            // Self::InvalidRegister => write!(f, "Invalid register address"),
            // Self::InitializationFailed => write!(f, "Device initialization failed"),
            // Self::Timeout => write!(f, "Operation timeout"),
            // Self::DeviceNotResponding => write!(f, "Device not responding on I2C bus"),
            // Self::InvalidState => write!(f, "Invalid state or configuration"),
        }
    }
}

impl<I2cE> core::fmt::Display for Tca9534Error<I2cE>
where
    I2cE: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Core(core_err) => write!(f, "{}", core_err),
            Self::I2c(err) => write!(f, "I2C error: {:?}", err),
        }
    }
}

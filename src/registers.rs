/// TCA9534 register definitions
///
/// Based on TCA9534 datasheet: <https://www.ti.com/lit/ds/symlink/tca9534.pdf>

/// Register enumeration
#[derive(Debug, Copy, Clone)]
pub enum Register {
    /// Input port register (0x00) - Read only
    /// Reflects the incoming logic levels of the pins, regardless of whether the pin is defined as an input or an output
    InputPort = 0x00,
    
    /// Output port register (0x01) - Read/Write
    /// The Output Port register shows the outgoing logic levels of the pins defined as outputs
    OutputPort = 0x01,
    
    /// Polarity Inversion register (0x02) - Read/Write  
    /// This register allows the user to invert the polarity of the Input Port register data
    Polarity = 0x02,
    
    /// Configuration register (0x03) - Read/Write
    /// The Configuration register configures the directions of the I/O pins
    /// 1 = pin is configured as an input (default)
    /// 0 = pin is configured as an output
    Config = 0x03,
}

impl Register {
    /// Get register address
    pub fn addr(self) -> u8 {
        self as u8
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Register {
    fn format(&self, fmt: defmt::Formatter) {
        match *self {
            Register::InputPort => defmt::write!(fmt, "InputPort"),
            Register::OutputPort => defmt::write!(fmt, "OutputPort"),
            Register::Polarity => defmt::write!(fmt, "Polarity"),
            Register::Config => defmt::write!(fmt, "Config"),
        }
    }
}

/// Pin configuration (direction)
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PinConfig {
    /// Pin configured as input (high impedance) - default
    Input = 1,
    /// Pin configured as output (can drive high or low)
    Output = 0,
}

impl PinConfig {
    /// Get pin config bit value
    pub fn bits(self) -> u8 {
        self as u8
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for PinConfig {
    fn format(&self, fmt: defmt::Formatter) {
        match *self {
            PinConfig::Input => defmt::write!(fmt, "Input"),
            PinConfig::Output => defmt::write!(fmt, "Output"),
        }
    }
}

/// Pin polarity setting
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PinPolarity {
    /// Normal polarity (default) - GPIO register bit reflects same value on the input pin
    Normal = 0,
    /// Inverted polarity - GPIO register bit reflects inverted value on the input pin
    Inverted = 1,
}

impl PinPolarity {
    /// Get polarity bit value
    pub fn bits(self) -> u8 {
        self as u8
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for PinPolarity {
    fn format(&self, fmt: defmt::Formatter) {
        match *self {
            PinPolarity::Normal => defmt::write!(fmt, "Normal"),
            PinPolarity::Inverted => defmt::write!(fmt, "Inverted"),
        }
    }
}

/// Pin logic level
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PinLevel {
    /// Logic low (0V)
    Low = 0,
    /// Logic high (VCC)
    High = 1,
}

impl PinLevel {
    /// Get level bit value
    pub fn bits(self) -> u8 {
        self as u8
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for PinLevel {
    fn format(&self, fmt: defmt::Formatter) {
        match *self {
            PinLevel::Low => defmt::write!(fmt, "Low"),
            PinLevel::High => defmt::write!(fmt, "High"),
        }
    }
}

/// Pin number type (0-7)
pub type Pin = u8;

/// Port value type (8-bit value representing all pins)
pub type PortValue = u8;

/// Configuration constants
pub mod config {
    /// All pins configured as inputs
    pub const ALL_INPUTS: u8 = 0xFF;
    
    /// All pins configured as outputs
    pub const ALL_OUTPUTS: u8 = 0x00;
    
    /// All pins normal polarity
    pub const ALL_NORMAL_POLARITY: u8 = 0x00;
    
    /// All pins inverted polarity
    pub const ALL_INVERTED_POLARITY: u8 = 0xFF;
    
    /// All outputs low
    pub const ALL_OUTPUTS_LOW: u8 = 0x00;
    
    /// All outputs high
    pub const ALL_OUTPUTS_HIGH: u8 = 0xFF;
}

/// Common I2C addresses for TCA9534 based on A2, A1, A0 pins
pub mod addresses {
    /// A2=0, A1=0, A0=0 (default)
    pub const ADDR_000: u8 = 0x20;
    /// A2=0, A1=0, A0=1
    pub const ADDR_001: u8 = 0x21;
    /// A2=0, A1=1, A0=0
    pub const ADDR_010: u8 = 0x22;
    /// A2=0, A1=1, A0=1
    pub const ADDR_011: u8 = 0x23;
    /// A2=1, A1=0, A0=0
    pub const ADDR_100: u8 = 0x24;
    /// A2=1, A1=0, A0=1
    pub const ADDR_101: u8 = 0x25;
    /// A2=1, A1=1, A0=0
    pub const ADDR_110: u8 = 0x26;
    /// A2=1, A1=1, A0=1
    pub const ADDR_111: u8 = 0x27;
} 
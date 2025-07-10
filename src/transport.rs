/// A synchronous I2C transport.
pub trait SyncTransport {
    /// The type of error that can be returned by the transport.
    type Error;

    /// Writes data to the I2C bus.
    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error>;

    /// Reads data from the I2C bus.
    fn read(&mut self, addr: u8, bytes: &mut [u8]) -> Result<(), Self::Error>;

    /// Writes and then reads data from the I2C bus.
    fn write_read(
        &mut self,
        addr: u8,
        wr_bytes: &[u8],
        rd_bytes: &mut [u8],
    ) -> Result<(), Self::Error>;
}

#[cfg(feature = "embedded-hal")]
#[allow(async_fn_in_trait)]
impl<I2C> SyncTransport for I2C
where
    I2C: embedded_hal::i2c::I2c,
{
    type Error = crate::error::TCA9534Error<I2C::Error>;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        I2C::write(self, addr, bytes).map_err(crate::error::TCA9534Error::I2c)
    }

    fn read(&mut self, addr: u8, bytes: &mut [u8]) -> Result<(), Self::Error> {
        I2C::read(self, addr, bytes).map_err(crate::error::TCA9534Error::I2c)
    }

    fn write_read(
        &mut self,
        addr: u8,
        wr_bytes: &[u8],
        rd_bytes: &mut [u8],
    ) -> Result<(), Self::Error> {
        I2C::write_read(self, addr, wr_bytes, rd_bytes).map_err(crate::error::TCA9534Error::I2c)
    }
}

/// An asynchronous I2C transport.
#[cfg(feature = "async")]
#[allow(async_fn_in_trait)]
pub trait AsyncTransport {
    /// The type of error that can be returned by the transport.
    type Error;

    /// Writes data to the I2C bus asynchronously.
    async fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error>;

    /// Reads data from the I2C bus asynchronously.
    async fn read(&mut self, addr: u8, bytes: &mut [u8]) -> Result<(), Self::Error>;

    /// Writes and then reads data from the I2C bus asynchronously.
    async fn write_read(
        &mut self,
        addr: u8,
        wr_bytes: &[u8],
        rd_bytes: &mut [u8],
    ) -> Result<(), Self::Error>;
}

#[cfg(all(feature = "async", feature = "embedded-hal-async"))]
impl<I2C> AsyncTransport for I2C
where
    I2C: embedded_hal_async::i2c::I2c,
{
    type Error = crate::error::TCA9534Error<I2C::Error>;

    async fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        I2C::write(self, addr, bytes)
            .await
            .map_err(crate::error::TCA9534Error::I2c)
    }

    async fn read(&mut self, addr: u8, bytes: &mut [u8]) -> Result<(), Self::Error> {
        I2C::read(self, addr, bytes)
            .await
            .map_err(crate::error::TCA9534Error::I2c)
    }

    async fn write_read(
        &mut self,
        addr: u8,
        wr_bytes: &[u8],
        rd_bytes: &mut [u8],
    ) -> Result<(), Self::Error> {
        I2C::write_read(self, addr, wr_bytes, rd_bytes)
            .await
            .map_err(crate::error::TCA9534Error::I2c)
    }
}

// #[cfg(feature = "async")]
// impl<I2C> AsyncTransport for embedded_hal_async::i2c::I2cDevice
// where
//     I2C: embedded_hal_async::i2c::I2c,
// {
//     type Error = TCA9534Error<I2C::Error>;

//     async fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
//         self.i2c.write(addr, bytes).await.map_err(TCA9534Error::I2c)
//     }

//     async fn read(&mut self, addr: u8, bytes: &mut [u8]) -> Result<(), Self::Error> {
//         self.i2c.read(addr, bytes).await.map_err(TCA9534Error::I2c)
//     }

//     async fn write_read(
//         &mut self,
//         addr: u8,
//         wr_bytes: &[u8],
//         rd_bytes: &mut [u8],
//     ) -> Result<(), Self::Error> {
//         self.i2c.write_read(addr, wr_bytes, rd_bytes).await.map_err(TCA9534Error::I2c)
//     }
// }

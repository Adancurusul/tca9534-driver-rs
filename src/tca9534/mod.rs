// Synchronous implementation (always available)
mod tca9534_sync;

// Asynchronous implementation (feature-gated)
#[cfg(feature = "async")]
mod tca9534_async;

// Re-export driver implementations  
pub use tca9534_sync::TCA9534 as TCA9534Sync;

#[cfg(feature = "async")]
pub use tca9534_async::TCA9534 as TCA9534Async;

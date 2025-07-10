// Synchronous implementation (always available).
mod tca9534_sync;

// Asynchronous implementation (feature-gated).
#[cfg(feature = "async")]
mod tca9534_async;

// Re-export driver implementations.

pub use tca9534_sync::Tca9534 as Tca9534Sync;

#[cfg(feature = "async")]
pub use tca9534_async::Tca9534 as Tca9534Async;

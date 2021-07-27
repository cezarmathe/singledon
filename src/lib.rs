//! Singledon (singletons).

#[cfg(feature = "async")]
mod r#async;
mod sync;

#[cfg(feature = "async")]
pub use self::r#async::*;
pub use self::sync::*;

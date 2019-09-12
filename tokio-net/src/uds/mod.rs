//! Unix Domain Sockets for Tokio.
//!
//! This crate provides APIs for using Unix Domain Sockets with Tokio.

mod datagram;
// mod frame;
mod incoming;
mod listener;
pub mod split;
mod stream;
mod ucred;

pub use self::datagram::UnixDatagram;
pub use self::listener::UnixListener;
pub use self::stream::UnixStream;
pub use self::ucred::UCred;

/// Concrete future types returned from the uds types.
///
/// These may come in handy if you need to be able to name specific tokio types to avoid dynamic
/// dispatch until [`impl Trait` in type aliases](https://github.com/rust-lang/rust/issues/63063)
/// stabilizes.
pub mod futures {
    #[cfg(feature = "async-traits")]
    pub use super::incoming::Incoming;
}

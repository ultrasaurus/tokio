//! TCP bindings for `tokio`.
//!
//! This module contains the TCP networking types, similar to the standard
//! library, which can be used to implement networking protocols.
//!
//! Connecting to an address, via TCP, can be done using [`TcpStream`]'s
//! [`connect`] method, which returns a future which returns a `TcpStream`.
//!
//! To listen on an address [`TcpListener`] can be used. `TcpListener`'s
//! [`incoming`][incoming_method] method can be used to accept new connections.
//! It return the [`Incoming`] struct, which implements a stream which returns
//! `TcpStream`s.
//!
//! [`TcpStream`]: struct.TcpStream.html
//! [`connect`]: struct.TcpStream.html#method.connect
//! [`TcpListener`]: struct.TcpListener.html
//! [incoming_method]: struct.TcpListener.html#method.incoming
//! [`Incoming`]: struct.Incoming.html

#[cfg(feature = "async-traits")]
mod incoming;
mod listener;
pub mod split;
mod stream;

pub use self::listener::TcpListener;
pub use self::stream::TcpStream;

/// Concrete future types returned from the tcp types.
///
/// These may come in handy if you need to be able to name specific tokio types to avoid dynamic
/// dispatch until [`impl Trait` in type aliases](https://github.com/rust-lang/rust/issues/63063)
/// stabilizes.
pub mod futures {
    #[cfg(feature = "async-traits")]
    pub use super::incoming::Incoming;
}

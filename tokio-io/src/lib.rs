#![doc(html_root_url = "https://docs.rs/tokio-io/0.2.0-alpha.4")]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![doc(test(no_crate_inject, attr(deny(rust_2018_idioms))))]

//! Core I/O traits and combinators when working with Tokio.
//!
//! A description of the high-level I/O combinators can be [found online] in
//! addition to a description of the [low level details].
//!
//! [found online]: https://tokio.rs/docs/
//! [low level details]: https://tokio.rs/docs/going-deeper-tokio/core-low-level/

mod async_buf_read;
mod async_read;
mod async_write;

#[cfg(feature = "util")]
mod io;

#[cfg(feature = "util")]
/// Concrete future types returned from the `Async*Ext` traits.
///
/// These may come in handy if you need to be able to name specific tokio types to avoid dynamic
/// dispatch until [`impl Trait` in type aliases](https://github.com/rust-lang/rust/issues/63063)
/// stabilizes.
pub mod futures {
    use super::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};

    /// Futures related to [`AsyncReadExt`].
    pub mod read {
        pub use crate::io::{
            chain::Chain, copy::Copy, read::Read, read_exact::ReadExact, read_to_end::ReadToEnd,
            read_to_string::ReadToString, take::Take,
        };
    }

    /// Futures related to [`AsyncBufReadExt`].
    pub mod buf_read {
        pub use crate::io::{lines::Lines, read_line::ReadLine, read_until::ReadUntil};
    }

    /// Futures related to [`AsyncWriteExt`].
    pub mod write {
        pub use crate::io::{flush::Flush, shutdown::Shutdown, write::Write, write_all::WriteAll};
    }
}

#[cfg(feature = "util")]
pub mod split;

pub use self::async_buf_read::AsyncBufRead;
pub use self::async_read::AsyncRead;
pub use self::async_write::AsyncWrite;

#[cfg(feature = "util")]
pub use self::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};

// Re-export `Buf` and `BufMut` since they are part of the API
pub use bytes::{Buf, BufMut};

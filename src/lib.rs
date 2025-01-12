#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod lib {
    mod core {
        #[cfg(not(feature = "std"))]
        pub use core::*;

        #[cfg(feature = "std")]
        pub use std::*;
    }

    pub use self::core::fmt::{self, Debug, Display};
    pub use self::core::iter;
    pub use self::core::marker::PhantomData;

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::boxed::Box;
    #[cfg(feature = "std")]
    pub use std::boxed::Box;

    #[cfg(any(feature = "alloc", feature = "std"))]
    pub use self::core::error;
    #[cfg(any(feature = "alloc", feature = "std"))]
    pub use self::core::future;
}

pub mod app;
pub mod common;
pub mod error;

#[cfg(any(feature = "alloc", feature = "std"))]
pub mod interface;

#[cfg(feature = "transport")]
pub mod transport;

type Result<T> = core::result::Result<T, error::ModbusError>;

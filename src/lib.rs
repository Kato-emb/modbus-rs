#![cfg_attr(not(feature = "std"), no_std)]

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
}

pub mod app;
pub mod common;
pub mod error;

type Result<T> = core::result::Result<T, error::ModbusError>;

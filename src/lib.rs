#![cfg_attr(not(feature = "std"), no_std)]

mod lib {
    mod core {
        #[cfg(not(feature = "std"))]
        pub use core::*;

        #[cfg(feature = "std")]
        pub use std::*;
    }

    pub use self::core::{iter, num, ptr, str};
    pub use self::core::{u16, u32, u64, u8, usize};

    pub use self::core::convert;
    pub use self::core::default;
    pub use self::core::fmt::{self, Debug, Display};
    pub use self::core::marker::{self, PhantomData};

    #[cfg(all(not(feature = "std")))]
    pub use heapless::Vec;
    #[cfg(feature = "std")]
    pub use std::vec::Vec;
}

mod error;
mod model;

type Result<T> = core::result::Result<T, error::ModbusError>;

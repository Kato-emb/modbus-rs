pub mod client;
pub mod model;
pub mod types;

type Result<T> = core::result::Result<T, crate::error::ModbusApplicationError>;

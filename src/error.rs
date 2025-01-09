use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum ModbusError {
    #[error("Modbus application error: {0}")]
    ApplicationError(#[from] ModbusApplicationError),
}

#[derive(Debug, PartialEq, Error)]
pub enum ModbusApplicationError {
    #[error("Buffer overflow occurred")]
    BufferOverflow,
    #[error("No space left in buffer")]
    NoSpaceLeft,
    #[error("Undefined function code: {0}")]
    UndefinedFunctionCode(u8),
    #[error("Undefined function code: {0}")]
    UndefinedExceptionCode(u8),
    #[error("Data out of range")]
    OutOfRange,
}

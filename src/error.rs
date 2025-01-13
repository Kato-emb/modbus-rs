use crate::lib::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModbusError {
    #[error("Modbus application error: {0}")]
    ApplicationError(#[from] ModbusApplicationError),
    #[error("Modbus Frame error: {0}")]
    FrameError(#[from] ModbusFrameError),
    #[error("Modbus transport error: {0}")]
    TransportError(#[from] ModbusTransportError),
}

#[derive(Debug, Error)]
pub enum ModbusApplicationError {}

#[derive(Debug, Error)]
pub enum ModbusTransportError {
    #[cfg(any(feature = "alloc", feature = "std"))]
    #[error(transparent)]
    TransportError(Box<dyn error::Error + Send + Sync>),
    #[error("Timeout occurred")]
    Timeout,
    #[error("Frame incomplete")]
    FrameIncomplete,
}

#[derive(Debug, Error)]
pub enum ModbusFrameError {
    #[error("Modbus PDU error: {0}")]
    PduError(#[from] ModbusPduError),
    #[error("Modbus ADU error: {0}")]
    AduError(#[from] ModbusAduError),
    #[error("Modbus buffer error: {0}")]
    BufferError(#[from] BufferError),
}

#[derive(Debug, Error)]
pub enum ModbusPduError {
    #[error("Undefined function code: {0}")]
    UndefinedFunctionCode(u8),
    #[error("Undefined exception code: {0}")]
    UndefinedExceptionCode(u8),
    #[error("Unexpected code: {0}")]
    UnexpectedCode(u8),
    #[error("Data out of range")]
    OutOfRange,
}

#[derive(Debug, Error)]
pub enum ModbusAduError {}

#[derive(Debug, Error)]
pub enum BufferError {
    #[error("Buffer overflow occurred")]
    BufferOverflow,
    #[error("No space left in buffer")]
    NoSpaceLeft,
}

#[cfg(feature = "rtu")]
#[derive(Debug, Error)]
pub enum ModbusRtuError {
    #[error("Invalid slave address: {0}")]
    InvalidSlaveAddress(u8),
    #[error("CRC validation failure")]
    CrcValidationFailure,
}

#[cfg(feature = "tcp")]
#[derive(Debug, Error)]
pub enum ModbusTcpError {}

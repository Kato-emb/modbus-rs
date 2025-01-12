use crate::lib::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModbusError {
    #[error("Modbus application error: {0}")]
    ApplicationError(#[from] ModbusApplicationError),
    #[error("Modbus transport error: {0}")]
    TransportError(#[from] ModbusTransportError),
    #[error("Modbus PDU error: {0}")]
    PduError(#[from] ModbusPduError),
}

#[derive(Debug, Error)]
pub enum ModbusApplicationError {
    #[error("Undefined function code: {0}")]
    UndefinedFunctionCode(u8),
    #[error("Undefined exception code: {0}")]
    UndefinedExceptionCode(u8),
    #[error("Unexpected code: src={0}, dst={1}")]
    UnexpectedCode(u8, u8),
    #[error("Data out of range")]
    OutOfRange,
}

#[derive(Debug, Error)]
pub enum ModbusTransportError {
    #[cfg(any(feature = "alloc", feature = "std"))]
    #[error(transparent)]
    TransportError(Box<dyn error::Error + Send + Sync>),
    #[cfg(feature = "rtu")]
    #[error(transparent)]
    RtuError(#[from] ModbusRtuError),
    #[cfg(feature = "tcp")]
    TcpError(#[from] ModbusTcpError),
    #[error("Timeout occurred")]
    Timeout,
}

#[derive(Debug, Error)]
pub enum ModbusPduError {
    #[error("Buffer overflow occurred")]
    BufferOverflow,
    #[error("No space left in buffer")]
    NoSpaceLeft,
}

#[cfg(feature = "rtu")]
#[derive(Debug, Error)]
pub enum ModbusRtuError {
    #[error(transparent)]
    SerialError(#[from] tokio_serial::Error),
    #[error("Invalid slave address: {0}")]
    InvalidSlaveAddress(u8),
    #[error("CRC validation failure")]
    CrcValidationFailure,
    #[error("Frame incomplete")]
    FrameIncomplete,
}

#[cfg(feature = "tcp")]
#[derive(Debug, Error)]
pub enum ModbusTcpError {}

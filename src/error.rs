use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModbusError {
    #[error("Modbus application error: {0}")]
    ApplicationError(#[from] ModbusApplicationError),
    #[error("Modbus transport error: {0}")]
    TransportError(#[from] ModbusTransportError),
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
    #[error(transparent)]
    BufferError(#[from] BufferError),
}

#[derive(Debug, Error)]
pub enum ModbusTransportError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[cfg(feature = "rtu")]
    #[error(transparent)]
    RtuError(#[from] ModbusRtuError),
    #[cfg(feature = "tcp")]
    TcpError(#[from] ModbusTcpError),
    #[error(transparent)]
    BufferError(#[from] BufferError),
    #[error("Timeout occurred")]
    Timeout,
}

#[derive(Debug, Error)]
pub enum BufferError {
    #[error("Buffer overflow occurred")]
    BufferOverflow,
    #[error("Buffer underflow occurred")]
    BufferUnderflow,
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

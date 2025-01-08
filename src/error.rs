use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModbusError {
    #[error("Modbus application error: {0}")]
    ApplicationError(#[from] ModbusApplicationError),
}

#[derive(Debug, Error)]
pub enum ModbusApplicationError {
    #[error("Buffer overflow occurred")]
    BufferOverflow,
    #[error("No space left in buffer")]
    NoSpaceLeft,
}

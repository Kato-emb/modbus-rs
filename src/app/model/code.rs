use crate::{error::ModbusApplicationError, lib::*};

/// Modbus function code
#[derive(Clone, Copy, PartialEq)]
pub enum FunctionCode {
    Public(PublicFunctionCode),
    UserDefined(u8),
}

impl From<u8> for FunctionCode {
    fn from(value: u8) -> Self {
        // MSB is reserved for exception code
        let value = value & 0x7F;

        match PublicFunctionCode::try_from(value) {
            Ok(code) => Self::Public(code),
            Err(_) => Self::UserDefined(value),
        }
    }
}

impl From<FunctionCode> for u8 {
    fn from(value: FunctionCode) -> Self {
        match value {
            FunctionCode::Public(code) => code as u8,
            FunctionCode::UserDefined(code) => code,
        }
    }
}

impl Debug for FunctionCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Public(code) => write!(f, "{:?}", code),
            Self::UserDefined(code) => write!(f, "{}", code),
        }
    }
}

/// Public Modbus function codes
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum PublicFunctionCode {
    ReadCoils = 0x01,
    ReadDiscreteInputs = 0x02,
    ReadHoldingRegisters = 0x03,
    ReadInputRegisters = 0x04,
    WriteSingleCoil = 0x05,
    WriteSingleRegister = 0x06,
    ReadExceptionStatus = 0x07,
    Diagnostics = 0x08,
    GetCommEventCounter = 0x0B,
    GetCommEventLog = 0x0C,
    WriteMultipleCoils = 0x0F,
    WriteMultipleRegisters = 0x10,
    ReportServerId = 0x11,
    ReadFileRecord = 0x14,
    WriteFileRecord = 0x15,
    MaskWriteRegister = 0x16,
    ReadWriteMultipleRegisters = 0x17,
    ReadFifoQueue = 0x18,
    EncapsulatedInterfaceTransport = 0x2B,
}

impl TryFrom<u8> for PublicFunctionCode {
    type Error = ModbusApplicationError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::ReadCoils),
            0x02 => Ok(Self::ReadDiscreteInputs),
            0x03 => Ok(Self::ReadHoldingRegisters),
            0x04 => Ok(Self::ReadInputRegisters),
            0x05 => Ok(Self::WriteSingleCoil),
            0x06 => Ok(Self::WriteSingleRegister),
            0x07 => Ok(Self::ReadExceptionStatus),
            0x08 => Ok(Self::Diagnostics),
            0x0B => Ok(Self::GetCommEventCounter),
            0x0C => Ok(Self::GetCommEventLog),
            0x0F => Ok(Self::WriteMultipleCoils),
            0x10 => Ok(Self::WriteMultipleRegisters),
            0x11 => Ok(Self::ReportServerId),
            0x14 => Ok(Self::ReadFileRecord),
            0x15 => Ok(Self::WriteFileRecord),
            0x16 => Ok(Self::MaskWriteRegister),
            0x17 => Ok(Self::ReadWriteMultipleRegisters),
            0x18 => Ok(Self::ReadFifoQueue),
            0x2B => Ok(Self::EncapsulatedInterfaceTransport),
            _ => Err(ModbusApplicationError::UndefinedFunctionCode(value)),
        }
    }
}

impl From<PublicFunctionCode> for u8 {
    fn from(value: PublicFunctionCode) -> Self {
        value as u8
    }
}

impl fmt::Debug for PublicFunctionCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

/// Modbus exception code
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExceptionCode {
    IllegalFunction = 0x01,
    IllegalDataAddress = 0x02,
    IllegalDataValue = 0x03,
    ServerDeviceFailure = 0x04,
    Acknowledge = 0x05,
    ServerDeviceBusy = 0x06,
    MemoryParityError = 0x08,
    GatewayPathUnavailable = 0x0A,
    GatewayTargetDeviceFailedToRespond = 0x0B,
    __Unknown = 0xFF,
}

impl TryFrom<u8> for ExceptionCode {
    type Error = ModbusApplicationError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::IllegalFunction),
            0x02 => Ok(Self::IllegalDataAddress),
            0x03 => Ok(Self::IllegalDataValue),
            0x04 => Ok(Self::ServerDeviceFailure),
            0x05 => Ok(Self::Acknowledge),
            0x06 => Ok(Self::ServerDeviceBusy),
            0x08 => Ok(Self::MemoryParityError),
            0x0A => Ok(Self::GatewayPathUnavailable),
            0x0B => Ok(Self::GatewayTargetDeviceFailedToRespond),
            _ => Err(ModbusApplicationError::UndefinedExceptionCode(value)),
        }
    }
}

impl From<ExceptionCode> for u8 {
    fn from(value: ExceptionCode) -> Self {
        value as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_code_function_code_from() {
        assert_eq!(
            FunctionCode::from(0x01),
            FunctionCode::Public(PublicFunctionCode::ReadCoils)
        );
        assert_eq!(FunctionCode::from(0x0A), FunctionCode::UserDefined(0x0A));
    }

    #[test]
    fn test_model_code_public_function_code_try_from() {
        assert_eq!(
            PublicFunctionCode::try_from(0x01).unwrap(),
            PublicFunctionCode::ReadCoils
        );
        assert!(PublicFunctionCode::try_from(0x80).is_err());
    }

    #[test]
    fn test_model_code_exception_code_try_from() {
        assert_eq!(
            ExceptionCode::try_from(0x01).unwrap(),
            ExceptionCode::IllegalFunction
        );
        assert!(ExceptionCode::try_from(0x80).is_err());
    }
}

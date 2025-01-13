use crate::{error::ModbusPduError, lib::*};

use super::{fcode::PublicFunctionCode, Pdu};

pub mod request;
pub mod response;

/// Modbus request implementation
#[derive(Clone, PartialEq)]
pub struct Request<T> {
    inner: Pdu,
    _marker: PhantomData<T>,
}

impl<T> Debug for Request<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Request")
            .field("function_code", &self.inner.function_code())
            .field("data", &self.inner.data())
            .finish()
    }
}

impl<T> Request<T> {
    pub fn into_inner(self) -> Pdu {
        self.inner
    }
}

/// Modbus response implementation
#[derive(Clone, PartialEq)]
pub struct Response<T> {
    inner: Pdu,
    _marker: PhantomData<T>,
}

impl<T> Debug for Response<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Response")
            .field("function_code", &self.inner.function_code())
            .field("data", &self.inner.data())
            .finish()
    }
}

impl<T> Response<T> {
    pub fn into_inner(self) -> Pdu {
        self.inner
    }
}

pub trait PublicFunction {
    fn function_code() -> PublicFunctionCode;
}

impl<T: PublicFunction> TryFrom<Pdu> for Request<T> {
    type Error = ModbusPduError;

    fn try_from(value: Pdu) -> Result<Self, Self::Error> {
        check_function_code(&value, T::function_code() as u8)?;

        Ok(Self {
            inner: value,
            _marker: PhantomData,
        })
    }
}

impl<T: PublicFunction> TryFrom<Pdu> for Response<T> {
    type Error = ModbusPduError;

    fn try_from(value: Pdu) -> Result<Self, Self::Error> {
        check_function_code(&value, T::function_code() as u8)?;

        Ok(Self {
            inner: value,
            _marker: PhantomData,
        })
    }
}

impl TryFrom<(Pdu, u8)> for Response<UserDefined> {
    type Error = ModbusPduError;

    fn try_from((pdu, function_code): (Pdu, u8)) -> Result<Self, Self::Error> {
        check_function_code(&pdu, function_code)?;

        Ok(Self {
            inner: pdu,
            _marker: PhantomData,
        })
    }
}

fn check_function_code(pdu: &Pdu, function_code: u8) -> Result<(), ModbusPduError> {
    if pdu.function_code() != Some(function_code) {
        Err(ModbusPduError::UnexpectedCode(function_code))
    } else {
        Ok(())
    }
}

/// Read Coils
///
/// This function code is used to read from 1 to 2000 contiguous status of coils in a remote device.
///
/// # Code
/// * Function Code : `0x01`
/// # Request
/// * Starting Address : `u16`
/// * Quantity of Coils : `u16`
/// # Response
/// * Byte Count : `u8`
/// * Coil Status : `[u8; N]`
#[derive(Debug, Clone, PartialEq)]
pub struct ReadCoils;

impl PublicFunction for ReadCoils {
    fn function_code() -> PublicFunctionCode {
        PublicFunctionCode::ReadCoils
    }
}

/// Read Discrete Inputs
///
/// This function code is used to read from 1 to 2000 contiguous status of discrete inputs in a remote device.
///
/// # Code
/// * Function Code : `0x02`
/// # Request
/// * Starting Address : `u16`
/// * Quantity of Inputs : `u16`
/// # Response
/// * Byte Count : `u8`
/// * Input Status : `[u8; N]`
#[derive(Debug, Clone, PartialEq)]
pub struct ReadDiscreteInputs;

impl PublicFunction for ReadDiscreteInputs {
    fn function_code() -> PublicFunctionCode {
        PublicFunctionCode::ReadDiscreteInputs
    }
}

/// Read Holding Registers
///
/// This function code is used to read the contents of a contiguous block of holding registers in a remote device.
///
/// # Code
/// * Function Code : `0x03`
/// # Request
/// * Starting Address : `u16`
/// * Quantity of Registers : `u16`
/// # Response
/// * Byte Count : `u8`
/// * Register Value : `[u16; N]`
#[derive(Debug, Clone, PartialEq)]
pub struct ReadHoldingRegisters;

impl PublicFunction for ReadHoldingRegisters {
    fn function_code() -> PublicFunctionCode {
        PublicFunctionCode::ReadHoldingRegisters
    }
}

/// Read Input Registers
///
/// This function code is used to read from 1 to 125 contiguous input registers in a remote device. The Request PDU specifies the starting register address and the number of registers.
///
/// # Code
/// * Function Code : `0x04`
/// # Request
/// * Starting Address : `u16`
/// * Quantity of Registers : `u16`
/// # Response
/// * Byte Count : `u8`
/// * Register Value : `[u16; N]`
#[derive(Debug, Clone, PartialEq)]
pub struct ReadInputRegisters;

impl PublicFunction for ReadInputRegisters {
    fn function_code() -> PublicFunctionCode {
        PublicFunctionCode::ReadInputRegisters
    }
}

/// Write Single Coil
///
/// This function code is used to write a single output to either ON or OFF in a remote device.
///
/// # Code
/// * Function Code : `0x05`
/// # Request
/// * Output Address : `u16`
/// * Output Value : `bool`
/// # Response
/// * Output Address : `u16`
/// * Output Value : `bool`
#[derive(Debug, Clone, PartialEq)]
pub struct WriteSingleCoil;

impl PublicFunction for WriteSingleCoil {
    fn function_code() -> PublicFunctionCode {
        PublicFunctionCode::WriteSingleCoil
    }
}

/// Write Single Register
///
/// This function code is used to write a single holding register in a remote device. The Request PDU specifies the address of the register to be written.
///
/// # Code
/// * Function Code : `0x06`
/// # Request
/// * Register Address : `u16`
/// * Register Value : `u16`
/// # Response
/// * Register Address : `u16`
/// * Register Value : `u16`
#[derive(Debug, Clone, PartialEq)]
pub struct WriteSingleRegister;

impl PublicFunction for WriteSingleRegister {
    fn function_code() -> PublicFunctionCode {
        PublicFunctionCode::WriteSingleRegister
    }
}

/// User Defined
///
/// This function code is used to define user defined function code.
///
/// # Code
/// * Function Code : `u8`
/// # Request
/// * Data : `[u8; 252]`
/// # Response
/// * Data : `[u8; 252]`
#[derive(Debug, Clone, PartialEq)]
pub struct UserDefined;

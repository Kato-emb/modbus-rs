use crate::{common::Pdu, lib::*};

pub mod code;
pub mod request;
pub mod response;

/// Modbus request implementation
#[derive(Clone, PartialEq)]
pub struct Request<T> {
    inner: Pdu,
    _marker: PhantomData<T>,
}

impl<T> From<Pdu> for Request<T> {
    fn from(value: Pdu) -> Self {
        Self {
            inner: value,
            _marker: PhantomData,
        }
    }
}

/// Modbus response implementation
#[derive(Clone, PartialEq)]
pub struct Response<T> {
    inner: Pdu,
    _marker: PhantomData<T>,
}

impl<T> From<Pdu> for Response<T> {
    fn from(value: Pdu) -> Self {
        Self {
            inner: value,
            _marker: PhantomData,
        }
    }
}

/// Function code descriptions
mod function {
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

    #[derive(Debug, Clone, PartialEq)]
    pub struct WriteSingleCoil;

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
}

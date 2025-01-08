use super::{
    code::{ExceptionCode, FunctionCode, PublicFunctionCode},
    Pdu,
};
use crate::{error::ModbusApplicationError, lib::*, Result};

pub mod request;
pub mod response;

const MAX_DATA_SIZE: usize = 252;
#[cfg(feature = "std")]
pub type DataVec = Vec<u8>;
#[cfg(not(feature = "std"))]
pub type DataVec = Vec<u8, MAX_DATA_SIZE>;

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

#[derive(Clone, PartialEq)]
pub struct ExceptionResponse {
    inner: Pdu,
}

impl From<Pdu> for ExceptionResponse {
    fn from(value: Pdu) -> Self {
        Self { inner: value }
    }
}

impl ExceptionResponse {
    pub fn new(function_code: FunctionCode, exception_code: ExceptionCode) -> Result<Self> {
        let mut pdu = Pdu::new(u8::from(function_code) | 0x80)?;
        pdu.put_u8(exception_code.into())?;
        Ok(Self { inner: pdu })
    }

    pub fn function_code(&self) -> FunctionCode {
        FunctionCode::from(self.inner.function_code())
    }

    pub fn exception_code(&self) -> Result<ExceptionCode> {
        let code = self
            .inner
            .get(0)
            .ok_or(ModbusApplicationError::MissingData)?;

        let code = ExceptionCode::try_from(*code)?;
        Ok(code)
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

#[derive(Debug, Clone, PartialEq)]
pub enum RequestPdu {
    ReadCoils(Request<ReadCoils>),
    ReadDiscreteInputs(Request<ReadDiscreteInputs>),
    ReadHoldingRegisters(Request<ReadHoldingRegisters>),
    // ReadInputRegisters(Request<ReadInputRegisters>),
    // WriteSingleCoil(Request<WriteSingleCoil>),
    WriteSingleRegister(Request<WriteSingleRegister>),
    UserDefined(Request<UserDefined>),
}

impl From<Pdu> for RequestPdu {
    fn from(value: Pdu) -> Self {
        let function_code = FunctionCode::from(value.function_code());

        match function_code {
            FunctionCode::Public(PublicFunctionCode::ReadCoils) => {
                RequestPdu::ReadCoils(Request::<ReadCoils>::from(value))
            }
            FunctionCode::Public(PublicFunctionCode::ReadDiscreteInputs) => {
                RequestPdu::ReadDiscreteInputs(Request::<ReadDiscreteInputs>::from(value))
            }
            FunctionCode::Public(PublicFunctionCode::ReadHoldingRegisters) => {
                RequestPdu::ReadHoldingRegisters(Request::<ReadHoldingRegisters>::from(value))
            }
            // FunctionCode::Public(PublicFunctionCode::ReadInputRegisters) => {
            //     RequestPdu::ReadInputRegisters(Request::<ReadInputRegisters>::from(value))
            // }
            // FunctionCode::Public(PublicFunctionCode::WriteSingleCoil) => {
            //     RequestPdu::WriteSingleCoil(Request::<WriteSingleCoil>::from(value))
            // }
            FunctionCode::Public(PublicFunctionCode::WriteSingleRegister) => {
                RequestPdu::WriteSingleRegister(Request::<WriteSingleRegister>::from(value))
            }
            _ => RequestPdu::UserDefined(Request::<UserDefined>::from(value)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResponsePdu {
    ReadCoils(Response<ReadCoils>),
    ReadDiscreteInputs(Response<ReadDiscreteInputs>),
    ReadHoldingRegisters(Response<ReadHoldingRegisters>),
    // ReadInputRegisters(Response<ReadInputRegisters>),
    // WriteSingleCoil(Response<WriteSingleCoil>),
    WriteSingleRegister(Response<WriteSingleRegister>),
    UserDefined(Response<UserDefined>),
}

impl From<Pdu> for ResponsePdu {
    fn from(value: Pdu) -> Self {
        let function_code = FunctionCode::from(value.function_code());

        match function_code {
            FunctionCode::Public(PublicFunctionCode::ReadCoils) => {
                ResponsePdu::ReadCoils(Response::<ReadCoils>::from(value))
            }
            FunctionCode::Public(PublicFunctionCode::ReadDiscreteInputs) => {
                ResponsePdu::ReadDiscreteInputs(Response::<ReadDiscreteInputs>::from(value))
            }
            FunctionCode::Public(PublicFunctionCode::ReadHoldingRegisters) => {
                ResponsePdu::ReadHoldingRegisters(Response::<ReadHoldingRegisters>::from(value))
            }
            // FunctionCode::Public(PublicFunctionCode::ReadInputRegisters) => {
            //     ResponsePdu::ReadInputRegisters(Response::<ReadInputRegisters>::from(value))
            // }
            // FunctionCode::Public(PublicFunctionCode::WriteSingleCoil) => {
            //     ResponsePdu::WriteSingleCoil(Response::<WriteSingleCoil>::from(value))
            // }
            FunctionCode::Public(PublicFunctionCode::WriteSingleRegister) => {
                ResponsePdu::WriteSingleRegister(Response::<WriteSingleRegister>::from(value))
            }
            _ => ResponsePdu::UserDefined(Response::<UserDefined>::from(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_data_excp_rsp_new() {
        let rxcep_rsp = ExceptionResponse::new(
            FunctionCode::Public(crate::model::code::PublicFunctionCode::ReadCoils),
            ExceptionCode::IllegalFunction,
        )
        .unwrap();
        assert_eq!(
            rxcep_rsp.function_code(),
            FunctionCode::Public(crate::model::code::PublicFunctionCode::ReadCoils)
        );
        assert_eq!(
            rxcep_rsp.exception_code().unwrap(),
            ExceptionCode::IllegalFunction
        );
    }

    #[test]
    fn test_model_data_req_pdu() {
        let mut pdu = Pdu::new(0x01).unwrap();
        pdu.put_u16(123).unwrap();
        pdu.put_u16(456).unwrap();
        let req_pdu = RequestPdu::from(pdu.clone());
        assert_eq!(
            req_pdu,
            RequestPdu::ReadCoils(Request::<ReadCoils>::from(pdu))
        );

        match req_pdu {
            RequestPdu::ReadCoils(req) => {
                assert_eq!(req.starting_address(), Some(123));
                assert_eq!(req.quantity_of_coils(), Some(456));
            }
            _ => panic!("unexpected request"),
        }
    }

    #[test]
    fn test_model_data_rsp_pdu() {
        let mut pdu = Pdu::new(0x01).unwrap();
        pdu.put_u8(1).unwrap();
        pdu.put_u8(1).unwrap();
        let rsp_pdu = ResponsePdu::from(pdu.clone());
        assert_eq!(
            rsp_pdu,
            ResponsePdu::ReadCoils(Response::<ReadCoils>::from(pdu))
        );

        match rsp_pdu {
            ResponsePdu::ReadCoils(rsp) => {
                assert_eq!(rsp.byte_count(), Some(1));
                assert_eq!(rsp.coil_status(), Some([1].as_ref()));
            }
            _ => panic!("unexpected response"),
        }
    }
}

use super::code::*;
use super::function::*;
use super::*;
use crate::{error::ModbusApplicationError, Result};

/// Read Coils
/// ## Code
/// * Function Code : `0x01`
/// ## Data fields
/// * Starting Address : `u16`
/// * Quantity of Coils : `u16`
pub type ReadCoilsRequest = Request<ReadCoils>;

impl Request<ReadCoils> {
    pub fn new(starting_address: u16, quantity_of_coils: u16) -> Result<Self> {
        if !(1..=2000).contains(&quantity_of_coils) {
            return Err(ModbusApplicationError::OutOfRange.into());
        }

        let mut pdu = Pdu::new(PublicFunctionCode::ReadCoils.into())?;
        pdu.put_u16(starting_address)?;
        pdu.put_u16(quantity_of_coils)?;

        Ok(Self {
            inner: pdu,
            _marker: PhantomData,
        })
    }

    pub fn starting_address(&self) -> Option<u16> {
        self.inner.get_u16(0)
    }

    pub fn quantity_of_coils(&self) -> Option<u16> {
        self.inner.get_u16(2)
    }
}

impl Debug for Request<ReadCoils> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Request<ReadCoils>")
            .field("starting_address", &self.starting_address())
            .field("quantity_of_coils", &self.quantity_of_coils())
            .finish()
    }
}

/// Read Discrete Inputs
/// ## Code
/// * Function Code : `0x02`
/// ## Data fields
/// * Starting Address : `u16`
/// * Quantity of Inputs : `u16`
pub type ReadDiscreteInputsRequest = Request<ReadDiscreteInputs>;

impl Request<ReadDiscreteInputs> {
    pub fn new(starting_address: u16, quantity_of_inputs: u16) -> Result<Self> {
        if !(1..=2000).contains(&quantity_of_inputs) {
            return Err(ModbusApplicationError::OutOfRange.into());
        }

        let mut pdu = Pdu::new(PublicFunctionCode::ReadDiscreteInputs.into())?;
        pdu.put_u16(starting_address)?;
        pdu.put_u16(quantity_of_inputs)?;

        Ok(Self {
            inner: pdu,
            _marker: PhantomData,
        })
    }

    pub fn starting_address(&self) -> Option<u16> {
        self.inner.get_u16(0)
    }

    pub fn quantity_of_inputs(&self) -> Option<u16> {
        self.inner.get_u16(2)
    }
}

impl Debug for Request<ReadDiscreteInputs> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Request<ReadDiscreteInputs>")
            .field("starting_address", &self.starting_address())
            .field("quantity_of_inputs", &self.quantity_of_inputs())
            .finish()
    }
}

/// Read Holding Registers
/// ## Code
/// * Function Code : `0x03`
/// ## Data fields
/// * Starting Address : `u16`
/// * Quantity of Registers : `u16`
pub type ReadHoldingRegistersRequest = Request<ReadHoldingRegisters>;

impl Request<ReadHoldingRegisters> {
    pub fn new(starting_address: u16, quantity_of_registers: u16) -> Result<Self> {
        if !(1..=125).contains(&quantity_of_registers) {
            return Err(ModbusApplicationError::OutOfRange.into());
        }

        let mut pdu = Pdu::new(PublicFunctionCode::ReadHoldingRegisters.into())?;
        pdu.put_u16(starting_address)?;
        pdu.put_u16(quantity_of_registers)?;

        Ok(Self {
            inner: pdu,
            _marker: PhantomData,
        })
    }

    pub fn starting_address(&self) -> Option<u16> {
        self.inner.get_u16(0)
    }

    pub fn quantity_of_registers(&self) -> Option<u16> {
        self.inner.get_u16(2)
    }
}

impl Debug for Request<ReadHoldingRegisters> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Request<ReadHoldingRegisters>")
            .field("starting_address", &self.starting_address())
            .field("quantity_of_registers", &self.quantity_of_registers())
            .finish()
    }
}

/// Read Input Registers
/// ## Code
/// * Function Code : `0x04`
/// ## Data fields
/// * Starting Address : `u16`
/// * Quantity of Registers : `u16`
pub type ReadInputRegistersRequest = Request<ReadInputRegisters>;

impl Request<ReadInputRegisters> {
    pub fn new(starting_address: u16, quantity_of_input_registers: u16) -> Result<Self> {
        if !(1..=125).contains(&quantity_of_input_registers) {
            return Err(ModbusApplicationError::OutOfRange.into());
        }

        let mut pdu = Pdu::new(PublicFunctionCode::ReadInputRegisters.into())?;
        pdu.put_u16(starting_address)?;
        pdu.put_u16(quantity_of_input_registers)?;

        Ok(Self {
            inner: pdu,
            _marker: PhantomData,
        })
    }

    pub fn starting_address(&self) -> Option<u16> {
        self.inner.get_u16(0)
    }

    pub fn quantity_of_input_registers(&self) -> Option<u16> {
        self.inner.get_u16(2)
    }
}

impl Debug for Request<ReadInputRegisters> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Request<ReadInputRegisters>")
            .field("starting_address", &self.starting_address())
            .field(
                "quantity_of_input_registers",
                &self.quantity_of_input_registers(),
            )
            .finish()
    }
}

/// Write Single Coil
/// ## Code
/// * Function Code : `0x05`
/// ## Data fields
/// * Output Address : `u16`
/// * Output Value : `u16`
pub type WriteSingleCoilRequest = Request<WriteSingleCoil>;

impl Request<WriteSingleCoil> {
    pub fn new(output_address: u16, output_value: bool) -> Result<Self> {
        let mut pdu = Pdu::new(PublicFunctionCode::WriteSingleCoil.into())?;
        pdu.put_u16(output_address)?;
        pdu.put_u16(if output_value { 0xFF00 } else { 0x0000 })?;

        Ok(Self {
            inner: pdu,
            _marker: PhantomData,
        })
    }

    pub fn output_address(&self) -> Option<u16> {
        self.inner.get_u16(0)
    }

    pub fn output_value(&self) -> Option<bool> {
        self.inner.get_u16(2).map(|v| v == 0xFF00)
    }
}

impl Debug for Request<WriteSingleCoil> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Request<WriteSingleCoil>")
            .field("output_address", &self.output_address())
            .field("output_value", &self.output_value())
            .finish()
    }
}

/// Write Single Register
/// ## Code
/// * Function Code : `0x06`
/// ## Data fields
/// * Register Address : `u16`
/// * Register Value : `u16`
pub type WriteSingleRegisterRequest = Request<WriteSingleRegister>;

impl Request<WriteSingleRegister> {
    pub fn new(register_address: u16, register_value: u16) -> Result<Self> {
        let mut pdu = Pdu::new(PublicFunctionCode::WriteSingleRegister.into())?;
        pdu.put_u16(register_address)?;
        pdu.put_u16(register_value)?;

        Ok(Self {
            inner: pdu,
            _marker: PhantomData,
        })
    }

    pub fn register_address(&self) -> Option<u16> {
        self.inner.get_u16(0)
    }

    pub fn register_value(&self) -> Option<u16> {
        self.inner.get_u16(2)
    }
}

impl Debug for Request<WriteSingleRegister> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Request<WriteSingleRegister>")
            .field("register_address", &self.register_address())
            .field("register_value", &self.register_value())
            .finish()
    }
}

/// User Defined
/// ## Code
/// * Function Code : `u8`
/// ## Data fields
/// * Data : `[u8; 252]`
pub type UserDefinedRequest = Request<UserDefined>;

impl Request<UserDefined> {
    pub fn new(function_code: u8, data: &[u8]) -> Result<Self> {
        let mut pdu = Pdu::new(function_code)?;
        pdu.extend_from_slice(data)?;

        Ok(Self {
            inner: pdu,
            _marker: PhantomData,
        })
    }

    pub fn function_code(&self) -> u8 {
        self.inner.function_code()
    }

    pub fn data(&self) -> &[u8] {
        self.inner.data()
    }
}

impl Debug for Request<UserDefined> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Request<UserDefined>")
            .field("function_code", &self.function_code())
            .field("data", &self.data())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_model_req_read_coils_valid() {
        let req = ReadCoilsRequest::new(0x0001, 0x0002).unwrap();
        assert_eq!(req.starting_address(), Some(0x0001));
        assert_eq!(req.quantity_of_coils(), Some(0x0002));
    }

    #[test]
    fn test_app_model_req_read_coils_out_of_range() {
        assert!(ReadCoilsRequest::new(0x0001, 0x0000).is_err());
        assert!(ReadCoilsRequest::new(0x0001, 0x07D1).is_err());
    }

    #[test]
    fn test_app_model_req_read_discrete_inputs_vaild() {
        let req = ReadDiscreteInputsRequest::new(0x0001, 0x0002).unwrap();
        assert_eq!(req.starting_address(), Some(0x0001));
        assert_eq!(req.quantity_of_inputs(), Some(0x0002));
    }

    #[test]
    fn test_app_model_req_read_discrete_inputs_out_of_range() {
        assert!(ReadDiscreteInputsRequest::new(0x0001, 0x0000).is_err());
        assert!(ReadDiscreteInputsRequest::new(0x0001, 0x07D1).is_err());
    }

    #[test]
    fn test_app_model_req_read_holding_registers_vaild() {
        let req = ReadHoldingRegistersRequest::new(0x0001, 0x0002).unwrap();
        assert_eq!(req.starting_address(), Some(0x0001));
        assert_eq!(req.quantity_of_registers(), Some(0x0002));
    }

    #[test]
    fn test_app_model_req_read_holding_registers_out_of_range() {
        assert!(ReadHoldingRegistersRequest::new(0x0001, 0x0000).is_err());
        assert!(ReadHoldingRegistersRequest::new(0x0001, 0x007E).is_err());
    }

    #[test]
    fn test_app_model_req_read_input_registers_vaild() {
        let req = ReadInputRegistersRequest::new(0x0001, 0x0002).unwrap();
        assert_eq!(req.starting_address(), Some(0x0001));
        assert_eq!(req.quantity_of_input_registers(), Some(0x0002));
    }

    #[test]
    fn test_app_model_req_read_input_registers_out_of_range() {
        assert!(ReadInputRegistersRequest::new(0x0001, 0x0000).is_err());
        assert!(ReadInputRegistersRequest::new(0x0001, 0x007E).is_err());
    }

    #[test]
    fn test_model_data_req_write_single_coil_valid() {
        let req = WriteSingleCoilRequest::new(0x0001, true).unwrap();
        assert_eq!(req.output_address().unwrap(), 0x0001);
        assert_eq!(req.output_value().unwrap(), true);
    }

    #[test]
    fn test_model_data_req_write_single_register_valid() {
        let req = WriteSingleRegisterRequest::new(0x0001, 0x0002).unwrap();
        assert_eq!(req.register_address().unwrap(), 0x0001);
        assert_eq!(req.register_value().unwrap(), 0x0002);
    }

    #[test]
    fn test_model_data_req_user_defined() {
        let req = UserDefinedRequest::new(0x0A, &[0x01, 0x02]).unwrap();
        assert_eq!(req.function_code(), 0x0A);
        assert_eq!(req.data(), &[0x01, 0x02]);
    }
}

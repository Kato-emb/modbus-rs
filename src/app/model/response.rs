use super::code::*;
use super::function::*;
use super::*;
use crate::app::types::*;
use crate::app::Result;

/// Read Coils
/// ## Code
/// * Function Code : `0x01`
/// ## Data fields
/// * Byte Count : `N / 8 (+ 1)`
/// * Coil Status : `[bool; N]`
pub type ReadCoilsResponse = Response<ReadCoils>;

impl Response<ReadCoils> {
    pub fn new(coil_status: DataVec) -> Result<Self> {
        debug_assert!(coil_status.len() <= 250);

        let mut pdu = Pdu::new(PublicFunctionCode::ReadCoils.into())?;
        pdu.put_u8(coil_status.len() as u8)?;
        pdu.extend_from_slice(&coil_status)?;

        Ok(Self {
            inner: pdu,
            _marker: PhantomData,
        })
    }

    pub fn byte_count(&self) -> Option<u8> {
        self.inner.get_u8(0)
    }

    pub fn coil_status(&self) -> Option<BitSet<'_>> {
        let byte_count = self.byte_count()?.checked_add(1)?;
        Some(BitSet::new(&self.inner.data()[1..byte_count as usize]))
    }
}

impl Display for Response<ReadCoils> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response<ReadCoils>")
            .field("byte_count", &self.byte_count())
            .field("coil_status", &self.coil_status())
            .finish()
    }
}

/// Read Discrete Inputs
/// ## Code
/// * Function Code : `0x02`
/// ## Data fields
/// * Byte Count : `N / 8 (+ 1)`
/// * Input Status : `[bool; N]`
pub type ReadDiscreteInputsResponse = Response<ReadDiscreteInputs>;

impl Response<ReadDiscreteInputs> {
    pub fn new(input_status: DataVec) -> Result<Self> {
        debug_assert!(input_status.len() <= 250);

        let mut pdu = Pdu::new(PublicFunctionCode::ReadDiscreteInputs.into())?;
        pdu.put_u8(input_status.len() as u8)?;
        pdu.extend_from_slice(&input_status)?;

        Ok(Self {
            inner: pdu,
            _marker: PhantomData,
        })
    }

    pub fn byte_count(&self) -> Option<u8> {
        self.inner.get_u8(0)
    }

    pub fn input_status(&self) -> Option<BitSet<'_>> {
        let byte_count = self.byte_count()?.checked_add(1)?;
        Some(BitSet::new(&self.inner.data()[1..byte_count as usize]))
    }
}

impl Display for Response<ReadDiscreteInputs> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response<ReadDiscreteInputs>")
            .field("byte_count", &self.byte_count())
            .field("input_status", &self.input_status())
            .finish()
    }
}

/// Read Holding Registers
/// ## Code
/// * Function Code : `0x03`
/// ## Data fields
/// * Byte Count : `N * 2`
/// * Register Value : `[u16; N]`
pub type ReadHoldingRegistersResponse = Response<ReadHoldingRegisters>;

impl Response<ReadHoldingRegisters> {
    pub fn new(register_value: DataVec) -> Result<Self> {
        debug_assert!(register_value.len() <= 250);

        let mut pdu = Pdu::new(PublicFunctionCode::ReadHoldingRegisters.into())?;
        pdu.put_u8(register_value.len() as u8)?;
        pdu.extend_from_slice(&register_value)?;

        Ok(Self {
            inner: pdu,
            _marker: PhantomData,
        })
    }

    pub fn byte_count(&self) -> Option<u8> {
        self.inner.get_u8(0)
    }

    pub fn register_value(&self) -> Option<RegisterSlice<'_>> {
        let byte_count = self.byte_count()?.checked_add(1)?;
        Some(RegisterSlice::new(
            &self.inner.data()[1..byte_count as usize],
        ))
    }

    pub fn register(&self, index: usize) -> Option<u16> {
        let byte_count = self.byte_count()?;
        let start = 1 + index * 2;

        // Check if the index is within the bounds
        if start < byte_count as usize {
            self.inner.get_u16(start)
        } else {
            None
        }
    }
}

impl Display for Response<ReadHoldingRegisters> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response<ReadHoldingRegisters>")
            .field("byte_count", &self.byte_count())
            .field("register_value", &self.register_value())
            .finish()
    }
}

/// Read Input Registers
/// ## Code
/// * Function Code : `0x04`
/// ## Data fields
/// * Byte Count : `N * 2`
/// * Input Registers : `[u16; N]`
pub type ReadInputRegistersResponse = Response<ReadInputRegisters>;

impl Response<ReadInputRegisters> {
    pub fn new(input_registers: DataVec) -> Result<Self> {
        debug_assert!(input_registers.len() <= 250);

        let mut pdu = Pdu::new(PublicFunctionCode::ReadInputRegisters.into())?;
        pdu.put_u8(input_registers.len() as u8)?;
        pdu.extend_from_slice(&input_registers)?;

        Ok(Self {
            inner: pdu,
            _marker: PhantomData,
        })
    }

    pub fn byte_count(&self) -> Option<u8> {
        self.inner.get_u8(0)
    }

    pub fn input_registers(&self) -> Option<RegisterSlice<'_>> {
        let byte_count = self.byte_count()?.checked_add(1)?;
        Some(RegisterSlice::new(
            &self.inner.data()[1..byte_count as usize],
        ))
    }

    pub fn register(&self, index: usize) -> Option<u16> {
        let byte_count = self.byte_count()?;
        let start = 1 + index * 2;

        // Check if the index is within the bounds
        if start < byte_count as usize {
            self.inner.get_u16(start)
        } else {
            None
        }
    }
}

impl Display for Response<ReadInputRegisters> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response<ReadInputRegisters>")
            .field("byte_count", &self.byte_count())
            .field("input_registers", &self.input_registers())
            .finish()
    }
}

/// Write Single Coil
/// ## Code
/// * Function Code : `0x05`
/// ## Request
/// * Output Address : `u16`
/// * Output Value : `bool`
pub type WriteSingleCoilResponse = Response<WriteSingleCoil>;

impl Response<WriteSingleCoil> {
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
        self.inner.get_u16(2).map(|value| value == 0xFF00)
    }
}

impl Display for Response<WriteSingleCoil> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response<WriteSingleCoil>")
            .field("output_address", &self.output_address())
            .field("output_value", &self.output_value())
            .finish()
    }
}

/// Write Single Register
/// ## Code
/// * Function Code : `0x06`
/// ## Request
/// * Register Address : `u16`
/// * Register Value : `u16`
pub type WriteSingleRegisterResponse = Response<WriteSingleRegister>;

impl Response<WriteSingleRegister> {
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

impl Display for Response<WriteSingleRegister> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response<WriteSingleRegister>")
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
pub type UserDefinedResponse = Response<UserDefined>;

impl Response<UserDefined> {
    pub fn new(function_code: u8, data: DataVec) -> Result<Self> {
        let mut pdu = Pdu::new(function_code)?;
        pdu.extend_from_slice(&data)?;

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

    pub fn from_pdu(pdu: Pdu) -> Self {
        Self {
            inner: pdu,
            _marker: PhantomData,
        }
    }
}

impl Display for Response<UserDefined> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response<UserDefined>")
            .field("function_code", &self.function_code())
            .field("data", &self.data())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_model_rsp_read_coils() {
        let coil_status = DataVec::from_slice(&[0x12, 0x34]).unwrap();
        let rsp = ReadCoilsResponse::new(coil_status).unwrap();
        assert_eq!(rsp.byte_count(), Some(0x02));
        let mut coil_status = rsp.coil_status().unwrap();

        // first byte
        assert_eq!(coil_status.next(), Some(false));
        assert_eq!(coil_status.next(), Some(true));
        assert_eq!(coil_status.next(), Some(false));
        assert_eq!(coil_status.next(), Some(false));
        assert_eq!(coil_status.next(), Some(true));
        assert_eq!(coil_status.next(), Some(false));
        assert_eq!(coil_status.next(), Some(false));
        assert_eq!(coil_status.next(), Some(false));

        // second byte
        assert_eq!(coil_status.next(), Some(false));
        assert_eq!(coil_status.next(), Some(false));
        assert_eq!(coil_status.next(), Some(true));
        assert_eq!(coil_status.next(), Some(false));
        assert_eq!(coil_status.next(), Some(true));
        assert_eq!(coil_status.next(), Some(true));
        assert_eq!(coil_status.next(), Some(false));
        assert_eq!(coil_status.next(), Some(false));

        // eos
        assert_eq!(coil_status.next(), None);
    }

    #[test]
    fn test_app_model_rsp_read_discrete_inputs() {
        let input_status = DataVec::from_slice(&[0x12, 0x34]).unwrap();
        let rsp = ReadDiscreteInputsResponse::new(input_status).unwrap();
        assert_eq!(rsp.byte_count(), Some(0x02));
        let mut input_status = rsp.input_status().unwrap();

        // first byte
        assert_eq!(input_status.next(), Some(false));
        assert_eq!(input_status.next(), Some(true));
        assert_eq!(input_status.next(), Some(false));
        assert_eq!(input_status.next(), Some(false));
        assert_eq!(input_status.next(), Some(true));
        assert_eq!(input_status.next(), Some(false));
        assert_eq!(input_status.next(), Some(false));
        assert_eq!(input_status.next(), Some(false));

        // second byte
        assert_eq!(input_status.next(), Some(false));
        assert_eq!(input_status.next(), Some(false));
        assert_eq!(input_status.next(), Some(true));
        assert_eq!(input_status.next(), Some(false));
        assert_eq!(input_status.next(), Some(true));
        assert_eq!(input_status.next(), Some(true));
        assert_eq!(input_status.next(), Some(false));
        assert_eq!(input_status.next(), Some(false));

        // eos
        assert_eq!(input_status.next(), None);
    }

    #[test]
    fn test_app_model_rsp_read_holding_registers() {
        let register_value = DataVec::from_slice(&[0x12, 0x34, 0x56, 0x78]).unwrap();
        let rsp = ReadHoldingRegistersResponse::new(register_value).unwrap();
        assert_eq!(rsp.byte_count(), Some(0x04));
        let mut register_value = rsp.register_value().unwrap();

        assert_eq!(register_value.next(), Some(0x1234));
        assert_eq!(register_value.next(), Some(0x5678));
        assert_eq!(register_value.next(), None);

        assert_eq!(rsp.register(0), Some(0x1234));
        assert_eq!(rsp.register(1), Some(0x5678));
        assert_eq!(rsp.register(2), None);
    }

    #[test]
    fn test_app_model_rsp_read_input_registers() {
        let input_registers = DataVec::from_slice(&[0x12, 0x34, 0x56, 0x78]).unwrap();
        let rsp = ReadInputRegistersResponse::new(input_registers).unwrap();
        assert_eq!(rsp.byte_count(), Some(0x04));
        let mut input_registers = rsp.input_registers().unwrap();

        assert_eq!(input_registers.next(), Some(0x1234));
        assert_eq!(input_registers.next(), Some(0x5678));
        assert_eq!(input_registers.next(), None);

        assert_eq!(rsp.register(0), Some(0x1234));
        assert_eq!(rsp.register(1), Some(0x5678));
        assert_eq!(rsp.register(2), None);
    }

    #[test]
    fn test_app_model_rsp_wite_single_register() {
        let rsp = WriteSingleRegisterResponse::new(0x0102, 0x0304).unwrap();
        assert_eq!(rsp.register_address(), Some(0x0102));
        assert_eq!(rsp.register_value(), Some(0x0304));
    }

    #[test]
    fn test_app_model_rsp_user_defined() {
        let data = DataVec::from_slice(&[0x01, 0x02]).unwrap();
        let rsp = UserDefinedResponse::new(0x0A, data).unwrap();
        assert_eq!(rsp.function_code(), 0x0A);
        assert_eq!(rsp.data(), &[0x01, 0x02]);
    }
}

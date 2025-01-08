use super::*;

impl Request<ReadCoils> {
    pub fn new(starting_address: u16, quantity_of_coils: u16) -> Result<Self> {
        if quantity_of_coils < 1 || quantity_of_coils > 2000 {
            return Err(ModbusApplicationError::InvalidData.into());
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

impl Request<ReadDiscreteInputs> {
    pub fn new(starting_address: u16, quantity_of_inputs: u16) -> Result<Self> {
        if quantity_of_inputs < 1 || quantity_of_inputs > 2000 {
            return Err(ModbusApplicationError::InvalidData.into());
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

impl Request<ReadHoldingRegisters> {
    pub fn new(starting_address: u16, quantity_of_registers: u16) -> Result<Self> {
        if quantity_of_registers < 1 || quantity_of_registers > 125 {
            return Err(ModbusApplicationError::InvalidData.into());
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
    fn test_model_data_req_read_coils() {
        let req = Request::<ReadCoils>::new(0x0001, 0x0002).unwrap();
        assert_eq!(req.starting_address().unwrap(), 0x0001);
        assert_eq!(req.quantity_of_coils().unwrap(), 0x0002);
    }

    #[test]
    fn test_model_data_req_read_discrete_inputs() {
        let req = Request::<ReadDiscreteInputs>::new(0x0001, 0x0002).unwrap();
        assert_eq!(req.starting_address().unwrap(), 0x0001);
        assert_eq!(req.quantity_of_inputs().unwrap(), 0x0002);
    }

    #[test]
    fn test_model_data_req_read_holding_registers() {
        let req = Request::<ReadHoldingRegisters>::new(0x0001, 0x0002).unwrap();
        assert_eq!(req.starting_address().unwrap(), 0x0001);
        assert_eq!(req.quantity_of_registers().unwrap(), 0x0002);
    }

    #[test]
    fn test_model_data_req_write_single_register() {
        let req = Request::<WriteSingleRegister>::new(0x0001, 0x0002).unwrap();
        assert_eq!(req.register_address().unwrap(), 0x0001);
        assert_eq!(req.register_value().unwrap(), 0x0002);
    }

    #[test]
    fn test_model_data_req_user_defined() {
        let req = Request::<UserDefined>::new(0x0A, &[0x01, 0x02]).unwrap();
        assert_eq!(req.function_code(), 0x0A);
        assert_eq!(req.data(), &[0x01, 0x02]);
    }
}

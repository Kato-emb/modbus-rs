use super::*;

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

    pub fn coil_status(&self) -> Option<&[u8]> {
        let byte_count = self.byte_count()?.checked_add(1)?;
        Some(&self.inner.data()[1..byte_count as usize])
    }
}

impl Debug for Response<ReadCoils> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response<ReadCoils>")
            .field("byte_count", &self.byte_count())
            .field("coil_status", &self.coil_status())
            .finish()
    }
}

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

    pub fn input_status(&self) -> Option<&[u8]> {
        let byte_count = self.byte_count()?.checked_add(1)?;
        Some(&self.inner.data()[1..byte_count as usize])
    }
}

impl Debug for Response<ReadDiscreteInputs> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response<ReadDiscreteInputs>")
            .field("byte_count", &self.byte_count())
            .field("input_status", &self.input_status())
            .finish()
    }
}

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

    pub fn register_value(&self) -> Option<&[u8]> {
        let byte_count = self.byte_count()?.checked_add(1)?;
        Some(&self.inner.data()[1..byte_count as usize])
    }

    pub fn register(&self, index: usize) -> Option<u16> {
        let byte_count = self.byte_count()?;
        let start = 1 + index * 2;

        // Check if the index is within the bounds
        if start + 1 <= byte_count as usize {
            self.inner.get_u16(start)
        } else {
            None
        }
    }
}

impl Debug for Response<ReadHoldingRegisters> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response<ReadHoldingRegisters>")
            .field("byte_count", &self.byte_count())
            .field("register_value", &self.register_value())
            .finish()
    }
}

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

impl Debug for Response<WriteSingleRegister> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response<WriteSingleRegister>")
            .field("register_address", &self.register_address())
            .field("register_value", &self.register_value())
            .finish()
    }
}

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
}

impl Debug for Response<UserDefined> {
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

    fn create_data_vec(data: &[u8]) -> DataVec {
        #[cfg(feature = "std")]
        let data_vec = DataVec::from(data);
        #[cfg(not(feature = "std"))]
        let data_vec = DataVec::from_slice(data).unwrap();
        data_vec
    }

    #[test]
    fn test_model_data_rsp_read_coils() {
        let coil_status = create_data_vec(&[0x02, 0x01, 0x01]);
        let rsp = Response::<ReadCoils>::new(coil_status).unwrap();
        assert_eq!(rsp.byte_count().unwrap(), 0x03);
        assert_eq!(rsp.coil_status().unwrap(), &[0x02, 0x01, 0x01]);
    }

    #[test]
    fn test_model_data_rsp_read_discrete_inputs() {
        let input_status = create_data_vec(&[0x02, 0x01, 0x01]);
        let rsp = Response::<ReadDiscreteInputs>::new(input_status).unwrap();
        assert_eq!(rsp.byte_count().unwrap(), 0x03);
        assert_eq!(rsp.input_status().unwrap(), &[0x02, 0x01, 0x01]);
    }

    #[test]
    fn test_model_data_rsp_read_holding_registers() {
        let register_value = create_data_vec(&[0x01, 0x02, 0x03, 0x04]);
        let rsp = Response::<ReadHoldingRegisters>::new(register_value).unwrap();
        assert_eq!(rsp.byte_count().unwrap(), 0x04);
        assert_eq!(rsp.register_value().unwrap(), &[0x01, 0x02, 0x03, 0x04]);
        assert_eq!(rsp.register(0).unwrap(), 0x0102);
        assert_eq!(rsp.register(1).unwrap(), 0x0304);
        assert!(rsp.register(2).is_none());
    }

    #[test]
    fn test_model_data_rsp_write_single_register() {
        let rsp = Response::<WriteSingleRegister>::new(0x0102, 0x0304).unwrap();
        assert_eq!(rsp.register_address().unwrap(), 0x0102);
        assert_eq!(rsp.register_value().unwrap(), 0x0304);
    }

    #[test]
    fn test_model_data_rsp_user_defined() {
        let data = create_data_vec(&[0x01, 0x02]);
        let rsp = Response::<UserDefined>::new(0x0A, data).unwrap();
        assert_eq!(rsp.function_code(), 0x0A);
        assert_eq!(rsp.data(), &[0x01, 0x02]);
    }
}

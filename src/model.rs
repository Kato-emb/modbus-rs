use crate::{error::ModbusApplicationError, lib::*, Result};

const MAX_PDU_SIZE: usize = 253;

#[cfg(feature = "std")]
type Data<T> = Vec<T>;

#[cfg(not(feature = "std"))]
type Data<T> = Vec<T, MAX_PDU_SIZE>;

#[derive(Clone)]
pub struct Pdu {
    pub(super) data: Data<u8>,
}

impl core::fmt::Debug for Pdu {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("pdu").field("data", &self.data).finish()
    }
}

impl Default for Pdu {
    fn default() -> Self {
        Self::new()
    }
}

impl Pdu {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "std")]
            data: Vec::with_capacity(MAX_PDU_SIZE),
            #[cfg(not(feature = "std"))]
            data: Vec::from_slice(&[0]).unwrap(),
        }
    }

    pub fn function_code(&self) -> u8 {
        self.data[0]
    }

    pub fn set_function_code(&mut self, function_code: u8) {
        self.data[0] = function_code;
    }

    pub fn data(&self) -> &[u8] {
        &self.data[1..]
    }

    pub fn push(&mut self, buf: u8) -> Result<()> {
        if self.data.len() < MAX_PDU_SIZE {
            self.data
                .push(buf)
                .map_err(|_| ModbusApplicationError::BufferOverflow)?;
            Ok(())
        } else {
            Err(ModbusApplicationError::NoSpaceLeft.into())
        }
    }

    pub fn extend_from_slice(&mut self, buf: &[u8]) -> Result<(), ()> {
        if self.data.len() + buf.len() <= MAX_PDU_SIZE {
            self.data.extend_from_slice(buf).map_err(|_| ())?;
            Ok(())
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_pdu_new() {
        let pdu = Pdu::new();
        assert_eq!(pdu.function_code(), 0);
        assert_eq!(pdu.data(), &[]);
    }

    #[test]
    fn test_model_pdu_set_function_code() {
        let mut pdu = Pdu::new();
        pdu.set_function_code(0x03);
        assert_eq!(pdu.function_code(), 0x03);
    }

    #[test]
    fn test_model_pdu_data() {
        let mut pdu = Pdu::new();
        pdu.push(0x01).unwrap();
        pdu.push(0x02).unwrap();
        pdu.push(0x03).unwrap();
        assert_eq!(pdu.data(), &[0x01, 0x02, 0x03]);
    }
}

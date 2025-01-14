use crate::error::ModbusFrameError;
use crate::lib::*;

use super::DataUnit;

pub mod fcode;
pub mod function;
pub mod types;

const MAX_PDU_SIZE: usize = 253;

/// Protocol Data Unit
/// # Structure
/// * Code : `u8`
/// * Data : `[u8; N]` (MAX : 252 bytes)
#[derive(Debug, Clone, PartialEq)]
pub struct Pdu(DataUnit<MAX_PDU_SIZE>);

impl Deref for Pdu {
    type Target = DataUnit<MAX_PDU_SIZE>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Pdu {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Pdu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Code:{:?} {:?}", self.function_code(), self.data())
    }
}

impl Pdu {
    pub fn new(function_code: u8) -> Result<Self, ModbusFrameError> {
        let mut pdu = Pdu(DataUnit::default());

        // Push function code
        pdu.push(function_code)?;

        Ok(pdu)
    }

    pub fn function_code(&self) -> Option<u8> {
        self.get_u8(0)
    }

    pub fn data(&self) -> &[u8] {
        &self.as_slice()[1..]
    }

    pub fn read_u8(&self, index: usize) -> Option<u8> {
        self.get_u8(index + 1)
    }

    pub fn read_u16(&self, index: usize) -> Option<u16> {
        self.get_u16(index + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_pdu_new() {
        let pdu = Pdu::new(1).unwrap();
        assert_eq!(pdu.function_code(), Some(1));
        assert_eq!(pdu.data(), &[]);
    }

    #[test]
    fn test_frame_pdu_put_u8() {
        let mut pdu = Pdu::new(1).unwrap();
        pdu.put_u8(0x01).unwrap();
        pdu.put_u8(0x02).unwrap();
        pdu.put_u8(0x03).unwrap();
        assert_eq!(pdu.data(), &[0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_frame_pdu_put_u16() {
        let mut pdu = Pdu::new(1).unwrap();
        pdu.put_u16(0x0102).unwrap();
        pdu.put_u16(0x0304).unwrap();
        assert_eq!(pdu.data(), &[0x01, 0x02, 0x03, 0x04]);
    }

    #[test]
    fn test_frame_pdu_data_extend_from_slice() {
        let mut pdu = Pdu::new(1).unwrap();
        let buf = &[0x01, 0x02, 0x03];
        assert!(pdu.put_slice(buf).is_ok());
        assert_eq!(pdu.data(), &[0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_frame_pdu_data_extend_from_slice_buffer_overflow() {
        let mut pdu = Pdu::new(1).unwrap();
        let buf = [0; MAX_PDU_SIZE];

        assert!(pdu.put_slice(&buf).is_err());
    }
}

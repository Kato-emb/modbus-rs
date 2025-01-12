use crate::{error::ModbusPduError, lib::*};

const MAX_PDU_SIZE: usize = 253;

type Result<T> = core::result::Result<T, ModbusPduError>;
type PduVec<T> = heapless::Vec<T, MAX_PDU_SIZE>;

/// Protocol Data Unit
/// # Structure
/// * Code : `u8`
/// * Data : `[u8; N]` (MAX : 252 bytes)
#[derive(Clone, PartialEq)]
pub struct Pdu {
    data: PduVec<u8>,
}

impl Debug for Pdu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("pdu").field("data", &self.data).finish()
    }
}

impl Display for Pdu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x} {:?}", self.function_code(), self.data())
    }
}

impl Pdu {
    pub fn new(function_code: u8) -> Result<Self> {
        let mut pdu = Self {
            data: PduVec::new(),
        };

        // Push function code
        pdu.push(function_code)?;
        Ok(pdu)
    }

    pub fn function_code(&self) -> u8 {
        self.data[0]
    }

    pub fn data(&self) -> &[u8] {
        &self.data[1..]
    }

    pub fn put_u8(&mut self, src: u8) -> Result<()> {
        self.push(src)
    }

    pub fn put_u16(&mut self, src: u16) -> Result<()> {
        self.push((src >> 8) as u8)?;
        self.push(src as u8)
    }

    pub fn put_slice(&mut self, src: &[u8]) -> Result<()> {
        if src.len() > MAX_PDU_SIZE - 1 {
            return Err(ModbusPduError::BufferOverflow);
        }

        self.extend_from_slice(src)
    }

    /// Get the value from `data field` at the given index
    pub fn get(&self, index: usize) -> Option<&u8> {
        self.data.get(index + 1)
    }

    pub fn get_u8(&self, index: usize) -> Option<u8> {
        self.get(index).copied()
    }

    pub fn get_u16(&self, h_idx: usize) -> Option<u16> {
        let high = self.get(h_idx)?;
        let low = self.get(h_idx + 1)?;

        Some(u16::from_be_bytes([*high, *low]))
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    fn push(&mut self, src: u8) -> Result<()> {
        self.data.push(src).map_err(|_| ModbusPduError::NoSpaceLeft)
    }

    fn extend_from_slice(&mut self, src: &[u8]) -> Result<()> {
        self.data
            .extend_from_slice(src)
            .map_err(|_| ModbusPduError::NoSpaceLeft)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_pdu_new() {
        let pdu = Pdu::new(1).unwrap();
        assert_eq!(pdu.function_code(), 1);
        assert_eq!(pdu.data(), &[]);
    }

    #[test]
    fn test_model_pdu_data_put_u8() {
        let mut pdu = Pdu::new(1).unwrap();
        pdu.put_u8(0x01).unwrap();
        pdu.put_u8(0x02).unwrap();
        pdu.put_u8(0x03).unwrap();
        assert_eq!(pdu.data(), &[0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_model_pdu_data_put_u16() {
        let mut pdu = Pdu::new(1).unwrap();
        pdu.put_u16(0x0102).unwrap();
        pdu.put_u16(0x0304).unwrap();
        assert_eq!(pdu.data(), &[0x01, 0x02, 0x03, 0x04]);
    }

    #[test]
    fn test_model_pdu_data_extend_from_slice() {
        let mut pdu = Pdu::new(1).unwrap();
        let buf = &[0x01, 0x02, 0x03];
        assert!(pdu.extend_from_slice(buf).is_ok());
        assert_eq!(pdu.data(), &[0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_model_pdu_data_extend_from_slice_buffer_overflow() {
        let mut pdu = Pdu::new(1).unwrap();
        let buf = [0; MAX_PDU_SIZE];

        assert!(pdu.extend_from_slice(&buf).is_err());
    }
}

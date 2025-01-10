use crate::{error::BufferError, lib::*};

const MIN_PDU_SIZE: usize = 1;
const MAX_PDU_SIZE: usize = 253;

type Result<T> = core::result::Result<T, BufferError>;
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

impl TryFrom<&[u8]> for Pdu {
    type Error = BufferError;

    fn try_from(value: &[u8]) -> core::result::Result<Self, Self::Error> {
        if value.len() < MIN_PDU_SIZE {
            return Err(BufferError::BufferUnderflow);
        }

        let data = PduVec::from_slice(value).map_err(|_| BufferError::BufferOverflow)?;
        Ok(Self { data })
    }
}

impl Pdu {
    pub fn new(function_code: u8) -> Result<Self> {
        let mut pdu = Self {
            data: PduVec::new(),
        };

        pdu.put_u8(function_code)?;

        Ok(pdu)
    }

    pub fn function_code(&self) -> u8 {
        self.data[0]
    }

    pub fn data(&self) -> &[u8] {
        &self.data[1..]
    }

    fn push(&mut self, buf: u8) -> Result<()> {
        self.data.push(buf).map_err(|_| BufferError::NoSpaceLeft)
    }

    pub fn put_u8(&mut self, value: u8) -> Result<()> {
        self.push(value)
    }

    pub fn put_u16(&mut self, value: u16) -> Result<()> {
        self.push((value >> 8) as u8)?;
        self.push(value as u8)
    }

    pub fn put_u16_le(&mut self, value: u16) -> Result<()> {
        self.push(value as u8)?;
        self.push((value >> 8) as u8)
    }

    pub fn extend_from_slice(&mut self, buf: &[u8]) -> Result<()> {
        if buf.len() > MAX_PDU_SIZE {
            return Err(BufferError::BufferOverflow);
        }

        self.data
            .extend_from_slice(buf)
            .map_err(|_| BufferError::NoSpaceLeft)
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

    pub fn get_u16_le(&self, l_idx: usize) -> Option<u16> {
        let low = self.get(l_idx)?;
        let high = self.get(l_idx + 1)?;

        Some(u16::from_le_bytes([*high, *low]))
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
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
    fn test_model_pdu_data_put_u16_le() {
        let mut pdu = Pdu::new(1).unwrap();
        pdu.put_u16_le(0x0102).unwrap();
        pdu.put_u16_le(0x0304).unwrap();
        assert_eq!(pdu.data(), &[0x02, 0x01, 0x04, 0x03]);
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

use crate::{error::BufferError, lib::*};

#[cfg(feature = "rtu")]
pub mod rtu;

#[cfg(feature = "tcp")]
pub mod tcp;

pub mod pdu;

#[derive(Clone, PartialEq)]
pub struct DataUnit<const N: usize> {
    data: [u8; N],
    position: usize,
}

impl<const N: usize> Debug for DataUnit<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("data_unit")
            .field("data", &self.data)
            .finish()
    }
}

impl<const N: usize> Default for DataUnit<N> {
    fn default() -> Self {
        Self {
            data: [0; N],
            position: 0,
        }
    }
}

impl<const N: usize> DataUnit<N> {
    pub fn len(&self) -> usize {
        self.position
    }

    pub fn is_empty(&self) -> bool {
        self.position == 0
    }

    pub fn clear(&mut self) {
        self.position = 0;
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data[..self.position]
    }

    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn put_u8(&mut self, src: u8) -> result::Result<(), BufferError> {
        self.push(src)
    }

    pub fn put_u16(&mut self, src: u16) -> result::Result<(), BufferError> {
        self.push((src >> 8) as u8)?;
        self.push(src as u8)
    }

    pub fn put_u16_le(&mut self, src: u16) -> result::Result<(), BufferError> {
        self.push(src as u8)?;
        self.push((src >> 8) as u8)
    }

    pub fn put_slice(&mut self, src: &[u8]) -> result::Result<(), BufferError> {
        self.extend_from_slice(src)
    }

    pub fn get_u8(&self, index: usize) -> Option<u8> {
        self.get(index).copied()
    }

    pub fn get_u16(&self, index: usize) -> Option<u16> {
        let high = self.get(index)?;
        let low = self.get(index + 1)?;

        Some(u16::from_be_bytes([*high, *low]))
    }

    pub fn get_u16_le(&self, index: usize) -> Option<u16> {
        let low = self.get(index)?;
        let high = self.get(index + 1)?;

        Some(u16::from_le_bytes([*low, *high]))
    }

    /// Set the length of the buffer.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check if the length is within the bounds of the buffer.
    pub fn advance(&mut self, count: usize) {
        self.position = (self.position + count).min(N);
    }

    fn push(&mut self, src: u8) -> result::Result<(), BufferError> {
        if self.position >= self.data.len() {
            return Err(BufferError::NoSpaceLeft);
        }

        self.data[self.position] = src;
        self.position += 1;

        Ok(())
    }

    fn extend_from_slice(&mut self, src: &[u8]) -> result::Result<(), BufferError> {
        if src.len() > self.data.len() {
            return Err(BufferError::BufferOverflow);
        }

        if self.position + src.len() > self.data.len() {
            return Err(BufferError::NoSpaceLeft);
        }

        self.data[self.position..self.position + src.len()].copy_from_slice(src);
        self.position += src.len();

        Ok(())
    }

    fn get(&self, index: usize) -> Option<&u8> {
        self.data.get(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_data_unit_new() {
        let pdu = DataUnit {
            data: [0; 10],
            position: 0,
        };
        assert_eq!(pdu.len(), 0);
    }

    #[test]
    fn test_frame_data_unit_put_u8() {
        let mut pdu = DataUnit {
            data: [0; 10],
            position: 0,
        };
        pdu.put_u8(0x01).unwrap();
        pdu.put_u8(0x02).unwrap();
        pdu.put_u8(0x03).unwrap();
        assert_eq!(pdu.len(), 3);
    }

    #[test]
    fn test_frame_data_unit_put_u16() {
        let mut pdu = DataUnit {
            data: [0; 10],
            position: 0,
        };
        pdu.put_u16(0x0102).unwrap();
        pdu.put_u16(0x0304).unwrap();
        assert_eq!(pdu.len(), 4);
    }

    #[test]
    fn test_frame_data_unit_put_u16_le() {
        let mut pdu = DataUnit {
            data: [0; 10],
            position: 0,
        };
        pdu.put_u16_le(0x0102).unwrap();
        pdu.put_u16_le(0x0304).unwrap();
        assert_eq!(pdu.len(), 4);
    }

    #[test]
    fn test_frame_data_unit_data_extend_from_slice() {
        let mut pdu = DataUnit {
            data: [0; 10],
            position: 0,
        };
        let buf = &[0x01, 0x02, 0x03];
        assert!(pdu.put_slice(buf).is_ok());
        assert_eq!(pdu.len(), 3);
    }

    #[test]
    fn test_frame_data_unit_data_extend_from_slice_buffer_overflow() {
        let mut pdu = DataUnit {
            data: [0; 10],
            position: 0,
        };
        pdu.put_u8(0x01).unwrap();
        let buf = [0; 10];

        assert!(pdu.put_slice(&buf).is_err());
    }

    #[test]
    fn test_frame_data_unit_get_u8() {
        let mut pdu = DataUnit {
            data: [0; 10],
            position: 0,
        };
        pdu.put_u8(0x01).unwrap();
        pdu.put_u8(0x02).unwrap();
        pdu.put_u8(0x03).unwrap();
        assert_eq!(pdu.get_u8(0), Some(0x01));
        assert_eq!(pdu.get_u8(1), Some(0x02));
        assert_eq!(pdu.get_u8(2), Some(0x03));
    }

    #[test]
    fn test_frame_data_unit_get_u16() {
        let mut pdu = DataUnit {
            data: [0; 10],
            position: 0,
        };
        pdu.put_u16(0x0102).unwrap();
        pdu.put_u16(0x0304).unwrap();
        assert_eq!(pdu.get_u16(0), Some(0x0102));
        assert_eq!(pdu.get_u16(2), Some(0x0304));
    }

    #[test]
    fn test_frame_data_unit_get_u16_le() {
        let mut pdu = DataUnit {
            data: [0; 10],
            position: 0,
        };
        pdu.put_u16_le(0x0102).unwrap();
        pdu.put_u16_le(0x0304).unwrap();
        assert_eq!(pdu.get_u16_le(0), Some(0x0102));
        assert_eq!(pdu.get_u16_le(2), Some(0x0304));
    }

    #[test]
    fn test_frame_data_unit_as_slice() {
        let mut pdu = DataUnit {
            data: [0; 10],
            position: 0,
        };
        pdu.put_u8(0x01).unwrap();
        pdu.put_u8(0x02).unwrap();
        pdu.put_u8(0x03).unwrap();
        assert_eq!(pdu.as_slice(), &[0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_frame_data_unit_clear() {
        let mut pdu = DataUnit {
            data: [0; 10],
            position: 0,
        };
        pdu.put_u8(0x01).unwrap();
        pdu.put_u8(0x02).unwrap();
        pdu.put_u8(0x03).unwrap();
        pdu.clear();
        assert_eq!(pdu.len(), 0);
    }
}

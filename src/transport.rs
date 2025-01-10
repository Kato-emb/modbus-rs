use crate::{
    common::Pdu,
    error::{BufferError, ModbusTransportError},
};

#[cfg(feature = "rtu")]
pub mod rtu;

#[cfg(feature = "tcp")]
pub mod tcp;

type Result<T> = core::result::Result<T, ModbusTransportError>;

#[cfg(feature = "rtu")]
const MAX_ADU_SIZE: usize = 256;

#[cfg(feature = "tcp")]
const MAX_ADU_SIZE: usize = 260;

type AduVec = [u8; MAX_ADU_SIZE];

#[derive(Debug)]
pub struct Adu {
    data: AduVec,
    position: usize,
}

impl Adu {
    pub fn new() -> Self {
        Self {
            data: [0; MAX_ADU_SIZE],
            position: 0,
        }
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

    pub fn extend_from_slice(&mut self, src: &[u8]) -> Result<()> {
        if src.len() > MAX_ADU_SIZE {
            return Err(BufferError::BufferOverflow.into());
        }

        if src.len() + self.position > MAX_ADU_SIZE {
            return Err(BufferError::NoSpaceLeft.into());
        }

        self.data[self.position..self.position + src.len()].copy_from_slice(src);
        self.position += src.len();

        Ok(())
    }

    fn push(&mut self, buf: u8) -> Result<()> {
        if self.position >= MAX_ADU_SIZE {
            return Err(BufferError::NoSpaceLeft.into());
        }

        self.data[self.position] = buf;
        self.position += 1;

        Ok(())
    }

    pub fn get(&self, index: usize) -> Option<&u8> {
        if index >= self.position {
            return None;
        }

        self.data.get(index)
    }

    pub fn get_u16_le(&self, l_idx: usize) -> Option<u16> {
        let low = self.get(l_idx)?;
        let high = self.get(l_idx + 1)?;

        Some(u16::from_le_bytes([*low, *high]))
    }

    pub fn clear(&mut self) {
        self.position = 0;
    }

    pub fn as_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn as_write_slice(&mut self) -> &mut [u8] {
        &mut self.data[self.position..]
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data[..self.position]
    }

    pub fn is_empty(&self) -> bool {
        self.position == 0
    }

    pub fn len(&self) -> usize {
        self.position
    }

    pub fn set_position(&mut self, pos: usize) {
        self.position = pos;
    }

    pub fn pdu(&self) -> Result<Pdu> {
        let pdu = Pdu::try_from(&self.data[1..self.data.len() - 2])?;
        Ok(pdu)
    }
}

use super::{pdu::Pdu, DataUnit};
use crate::error::{ModbusFrameError, ModbusRtuError};
use crate::lib::*;

const MAX_ADU_SIZE: usize = 256;

/// Modbus RTU Application Data Unit
/// # Structure
/// * Slave Address : `u8`
/// * PDU : `FunctionCode` + `Data` (MAX : 253 bytes)
/// * CRC : `[u8; 2]`
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Adu(DataUnit<MAX_ADU_SIZE>);

impl Deref for Adu {
    type Target = DataUnit<MAX_ADU_SIZE>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Adu {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct RtuFrameHandler;

impl RtuFrameHandler {
    pub fn build_frame(
        adu: &mut Adu,
        slave_address: u8,
        pdu: &Pdu,
    ) -> Result<usize, ModbusFrameError> {
        adu.clear();

        adu.put_u8(slave_address)?;
        adu.put_slice(pdu.as_slice())?;
        let crc = calc_crc(adu.as_slice());
        adu.put_u16_le(crc)?;

        Ok(adu.len())
    }

    pub fn parse_frame(frame: &[u8], expected_address: u8) -> Result<Pdu, ModbusFrameError> {
        check_frame_length(frame)?;
        check_frame_address(frame, expected_address)?;
        check_frame_crc(frame)?;

        let mut pdu = Pdu::new(frame[1])?;
        pdu.put_slice(&frame[2..frame.len() - 2])?;

        Ok(pdu)
    }
}

/// Check the Modbus RTU frame length of the given frame
fn check_frame_length(frame: &[u8]) -> Result<(), ModbusRtuError> {
    if frame.len() < 4 || frame.len() > MAX_ADU_SIZE {
        Err(ModbusRtuError::InvalidFrameLength)
    } else {
        Ok(())
    }
}

/// Check the Modbus RTU slave address of the given frame
fn check_frame_address(frame: &[u8], address: u8) -> Result<(), ModbusRtuError> {
    if address == 0 || frame[0] == address {
        Ok(())
    } else {
        Err(ModbusRtuError::InvalidSlaveAddress(frame[0]))
    }
}

/// Check the Modbus RTU CRC of the given frame
fn check_frame_crc(frame: &[u8]) -> Result<(), ModbusRtuError> {
    let crc = u16::from_le_bytes([frame[frame.len() - 2], frame[frame.len() - 1]]);
    checksum(&frame[..frame.len() - 2], crc)
}

/// Check the Modbus RTU CRC of the given data
fn checksum(data: &[u8], crc: u16) -> Result<(), ModbusRtuError> {
    let expected_crc = calc_crc(data);

    if crc != expected_crc {
        Err(ModbusRtuError::CrcValidationFailure)
    } else {
        Ok(())
    }
}

/// Calculate the Modbus 16-bit CRC for the given data
fn calc_crc(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for byte in data {
        crc = (crc >> 8) ^ MODBUS_16_CRC[((crc ^ (*byte as u16)) & 0xFF) as usize];
    }

    crc
}

/// Lookup table for the Modbus 16-bit CRC algorithm
const MODBUS_16_CRC: [u16; 256] = generate_crc_table();

/// Generate a lookup table for the Modbus 16-bit CRC algorithm
const fn generate_crc_table() -> [u16; 256] {
    let mut table = [0u16; 256];
    let mut i = 0;

    while i < 256 {
        let mut crc = i as u16;
        let mut j = 0;
        while j < 8 {
            if crc & 0x0001 != 0 {
                crc = (crc >> 1) ^ 0xA001;
            } else {
                crc >>= 1;
            }
            j += 1;
        }
        table[i] = crc;
        i += 1;
    }

    table
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_rtu_calc_crc_with_standard_data() {
        let data = b"123456789";
        let expected_crc = 0x4B37;
        assert_eq!(calc_crc(data), expected_crc);
    }

    #[test]
    fn test_frame_rtu_calc_crc_with_empty_data() {
        let data: [u8; 0] = [];
        let expected_crc = 0xFFFF;
        assert_eq!(calc_crc(&data), expected_crc);
    }

    #[test]
    fn test_frame_rtu_calc_crc_with_single_byte() {
        let data = [0x01];
        let expected_crc = 0x807E;
        assert_eq!(calc_crc(&data), expected_crc);
    }

    #[test]
    fn test_frame_rtu_calc_crc_with_multiple_bytes() {
        let data = [0x01, 0x02, 0x03, 0x04];
        let expected_crc = 0x2BA1;
        assert_eq!(calc_crc(&data), expected_crc);
    }

    #[test]
    fn test_frame_rtu_calc_crc_with_edge_values() {
        let data = [0xFF, 0x00, 0xFF, 0x00];
        let expected_crc = 0xC071;
        assert_eq!(calc_crc(&data), expected_crc);
    }
}

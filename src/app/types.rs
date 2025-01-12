use crate::lib::*;

/// Iterator over bits in a byte array
pub struct BitSet<'a> {
    bytes: &'a [u8],
    byte_index: usize,
    bit_index: usize,
}

impl Debug for BitSet<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BitSet")
            .field("bytes", &self.bytes)
            .finish()
    }
}

impl BitSet<'_> {
    pub fn new(bytes: &[u8]) -> BitSet {
        BitSet {
            bytes,
            byte_index: 0,
            bit_index: 0,
        }
    }
}

impl iter::Iterator for BitSet<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.byte_index >= self.bytes.len() {
            return None;
        }

        // pick from LSB
        let bit = (self.bytes[self.byte_index] >> self.bit_index) & 0x01 != 0;
        self.bit_index += 1;

        if self.bit_index >= 8 {
            self.byte_index += 1;
            self.bit_index = 0;
        }

        Some(bit)
    }
}

/// Iterator over 16-bit registers in a byte array
pub struct RegisterSlice<'a> {
    bytes: &'a [u8],
    index: usize,
}

impl Debug for RegisterSlice<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Register")
            .field("bytes", &self.bytes)
            .finish()
    }
}

impl RegisterSlice<'_> {
    pub fn new(bytes: &[u8]) -> RegisterSlice {
        RegisterSlice { bytes, index: 0 }
    }
}

impl Iterator for RegisterSlice<'_> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.bytes.len() {
            return None;
        }

        let value = u16::from_be_bytes([self.bytes[self.index], self.bytes[self.index + 1]]);
        self.index += 2;

        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_types_bitset_next() {
        let bytes = [0b0001_0001, 0b0010_0010];
        let mut bitset = BitSet {
            bytes: &bytes,
            byte_index: 0,
            bit_index: 0,
        };

        // first byte
        assert_eq!(bitset.next(), Some(true));
        assert_eq!(bitset.next(), Some(false));
        assert_eq!(bitset.next(), Some(false));
        assert_eq!(bitset.next(), Some(false));
        assert_eq!(bitset.next(), Some(true));
        assert_eq!(bitset.next(), Some(false));
        assert_eq!(bitset.next(), Some(false));
        assert_eq!(bitset.next(), Some(false));

        // second byte
        assert_eq!(bitset.next(), Some(false));
        assert_eq!(bitset.next(), Some(true));
        assert_eq!(bitset.next(), Some(false));
        assert_eq!(bitset.next(), Some(false));
        assert_eq!(bitset.next(), Some(false));
        assert_eq!(bitset.next(), Some(true));
        assert_eq!(bitset.next(), Some(false));
        assert_eq!(bitset.next(), Some(false));
        assert_eq!(bitset.next(), None);
    }

    #[test]
    fn test_types_register_slice_next() {
        let bytes = [0x01, 0x02, 0x03, 0x04];
        let mut register = RegisterSlice {
            bytes: &bytes,
            index: 0,
        };

        assert_eq!(register.next(), Some(0x0102));
        assert_eq!(register.next(), Some(0x0304));
        assert_eq!(register.next(), None);
    }
}

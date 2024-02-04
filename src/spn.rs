pub mod byte {
    use crate::PDU_NOT_AVAILABLE;

    pub fn enc(value: u8) -> u8 {
        value + 125
    }

    pub fn dec(value: u8) -> Option<u8> {
        if value != PDU_NOT_AVAILABLE {
            Some(value - 125)
        } else {
            None
        }
    }
}

pub mod rpm {
    use crate::PDU_NOT_AVAILABLE;

    pub fn enc(value: u16) -> [u8; 2] {
        (value * 8).to_le_bytes()
    }

    pub fn dec(value: &[u8]) -> Option<u16> {
        if value != [PDU_NOT_AVAILABLE, PDU_NOT_AVAILABLE] {
            Some((u16::from_le_bytes([value[0], value[1]]) as f32 * 0.125) as u16)
        } else {
            None
        }
    }
}

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

pub struct TimeDate {
    /// Year.
    pub year: i32,
    /// Month.
    pub month: u32,
    /// Day.
    pub day: u32,
    /// Hour.
    pub hour: u32,
    /// Minute.
    pub minute: u32,
    /// Second.
    pub second: u32,
}

impl TimeDate {
    pub fn from_pdu(pdu: &[u8]) -> Self {
        Self {
            year: pdu[5] as i32 + 1985,
            month: pdu[3] as u32,
            day: (pdu[4] as f32 * 0.25) as u32,
            hour: pdu[2] as u32,
            minute: pdu[1] as u32,
            second: (pdu[0] as f32 * 0.25) as u32,
        }
    }
}

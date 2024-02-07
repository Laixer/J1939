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
    pub fn from_pdu(pdu: &[u8; 8]) -> Self {
        Self {
            year: crate::decode::spn964(pdu[5]).unwrap_or(0) as i32,
            month: crate::decode::spn963(pdu[3]).unwrap_or(0) as u32,
            day: crate::decode::spn962(pdu[4]).unwrap_or(0) as u32,
            hour: crate::decode::spn961(pdu[2]).unwrap_or(0) as u32,
            minute: crate::decode::spn960(pdu[1]).unwrap_or(0) as u32,
            second: crate::decode::spn959(pdu[0]).unwrap_or(0) as u32,
        }
    }
}

use crate::PDU_NOT_AVAILABLE;

pub fn spn190(value: &[u8; 2]) -> Option<u16> {
    if value != &[PDU_NOT_AVAILABLE, PDU_NOT_AVAILABLE] {
        let rpm = (u16::from_le_bytes(*value) as f32 * 0.125) as u16;
        Some(rpm)
    } else {
        None
    }
}

pub fn spn103(value: &[u8; 2]) -> Option<u16> {
    if value != &[PDU_NOT_AVAILABLE, PDU_NOT_AVAILABLE] {
        Some(u16::from_le_bytes(*value) * 4)
    } else {
        None
    }
}

pub fn spn110(value: u8) -> Option<u8> {
    if value != PDU_NOT_AVAILABLE {
        Some(value - 40)
    } else {
        None
    }
}

pub fn spn174(value: u8) -> Option<u8> {
    if value != PDU_NOT_AVAILABLE {
        Some(value - 40)
    } else {
        None
    }
}

// TODO: This is possibly wrong.
pub fn spn157(value: &[u8; 2]) -> Option<u16> {
    if value != &[PDU_NOT_AVAILABLE, PDU_NOT_AVAILABLE] {
        Some(u16::from_le_bytes(*value))
    } else {
        None
    }
}

pub fn spn512(value: u8) -> Option<u8> {
    if value != PDU_NOT_AVAILABLE {
        Some(value - 125)
    } else {
        None
    }
}

pub fn spn513(value: u8) -> Option<u8> {
    if value != PDU_NOT_AVAILABLE {
        Some(value - 125)
    } else {
        None
    }
}

pub fn spn514(value: u8) -> Option<u8> {
    if value != PDU_NOT_AVAILABLE {
        Some(value - 125)
    } else {
        None
    }
}

pub fn spn515(value: &[u8; 2]) -> Option<u16> {
    if value != &[PDU_NOT_AVAILABLE, PDU_NOT_AVAILABLE] {
        let speed = (u16::from_le_bytes(*value) as f32 * 0.125) as u16;
        Some(speed)
    } else {
        None
    }
}

pub fn spn519(value: u8) -> Option<u8> {
    if value != PDU_NOT_AVAILABLE {
        Some(value)
    } else {
        None
    }
}

pub fn spn975(value: u8) -> Option<f32> {
    if value != PDU_NOT_AVAILABLE {
        Some(value as f32 * 0.4)
    } else {
        None
    }
}

pub fn spn1127(value: &[u8; 2]) -> Option<u16> {
    if value != &[PDU_NOT_AVAILABLE, PDU_NOT_AVAILABLE] {
        let pressure = (u16::from_le_bytes(*value) as f32 * 0.125) as u16;
        Some(pressure)
    } else {
        None
    }
}

pub fn spn1128(value: &[u8; 2]) -> Option<u16> {
    spn1127(value)
}

pub fn spn1129(value: &[u8; 2]) -> Option<u16> {
    spn1127(value)
}

pub fn spn1130(value: &[u8; 2]) -> Option<u16> {
    spn1127(value)
}

pub fn spn1483(value: u8) -> Option<u8> {
    if value != PDU_NOT_AVAILABLE {
        Some(value)
    } else {
        None
    }
}

pub fn spn1601(value: u8) -> Option<u8> {
    if value != PDU_NOT_AVAILABLE {
        Some(value - 125)
    } else {
        None
    }
}

pub fn spn1602(value: u8) -> Option<u8> {
    if value != PDU_NOT_AVAILABLE {
        Some(value - 125)
    } else {
        None
    }
}

use crate::PDU_NOT_AVAILABLE;

pub fn spn1483(value: u8) -> Option<u8> {
    if value != PDU_NOT_AVAILABLE {
        Some(value)
    } else {
        None
    }
}

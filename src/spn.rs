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

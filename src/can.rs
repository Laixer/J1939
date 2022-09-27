pub struct BitMaskFilter {
    pub filter: u32,
    pub mask: u32,
}

pub fn destination_address_filter(address: u8) -> BitMaskFilter {
    BitMaskFilter {
        filter: (address as u32) << 8,
        mask: 0xFF00,
    }
}

pub fn source_address_filter(address: u8) -> BitMaskFilter {
    BitMaskFilter {
        filter: (address as u32),
        mask: 0xFF,
    }
}

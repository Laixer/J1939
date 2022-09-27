/// CAN bus filter.
pub struct BitMaskFilter {
    /// CAN bus filter.
    pub filter: u32,
    /// CAN bus filter mask.
    pub mask: u32,
}

/// Create bitmask filter for destination address.
pub fn destination_address_filter(address: u8) -> BitMaskFilter {
    BitMaskFilter {
        filter: (address as u32) << 8,
        mask: 0xFF00,
    }
}

/// Create bitmask filter for source address.
pub fn source_address_filter(address: u8) -> BitMaskFilter {
    BitMaskFilter {
        filter: (address as u32),
        mask: 0xFF,
    }
}

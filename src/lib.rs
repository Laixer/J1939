#![no_std]

#[derive(Debug, PartialEq)]
pub enum PDUFormat {
    PDU1(u8),
    PDU2(u8),
}

#[derive(Debug, PartialEq)]
pub struct Id(u32);

/// Frame ID
impl Id {
    /// Construct new Frame ID.
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    /// Return ID as raw integer.
    pub fn as_raw(&self) -> u32 {
        self.0
    }

    /// Frame priority
    ///
    /// The priority ranges from 0 to 7, where 0 is the highest priority and 7 the lowest priority.
    ///
    /// Default priority for informational, proprietary, request and acknowledgement frames is 6.
    /// Default priority for control frames (e.g., speeding up or slowing down the vehicle) is 3.
    #[inline]
    pub fn priority(&self) -> u8 {
        (self.0 >> 26).try_into().unwrap()
    }

    // Data page
    pub fn dp(&self) -> u8 {
        ((self.0 >> 24) & 0x1).try_into().unwrap()
    }

    /// Parameter group number
    pub fn pgn(&self) -> u16 {
        match self.pf() {
            PDUFormat::PDU1(_) => (self.0 >> 8) & 0xff00,
            PDUFormat::PDU2(_) => (self.0 >> 8) & 0xffff,
        }
        .try_into()
        .unwrap()
    }

    /// PDU Format
    pub fn pf(&self) -> PDUFormat {
        let format: u8 = ((self.0 >> 16) & 0xff).try_into().unwrap();
        if format < 240 {
            PDUFormat::PDU1(format)
        } else {
            PDUFormat::PDU2(format)
        }
    }

    /// Test if the frame is a broadcast frame
    pub fn is_broadcast(&self) -> bool {
        match self.pf() {
            PDUFormat::PDU2(_) => true,
            _ => false,
        }
    }

    /// Frame destination address
    ///
    /// The destination address is only availale on PDU1 frames.
    pub fn destination_address(&self) -> Option<u8> {
        match self.pf() {
            PDUFormat::PDU1(_) => Some(self.ps()),
            _ => None,
        }
    }

    /// Frame group extension
    ///
    /// The group extension is only availale on PDU2 frames.
    pub fn group_extension(&self) -> Option<u8> {
        match self.pf() {
            PDUFormat::PDU2(_) => Some(self.ps()),
            _ => None,
        }
    }

    /// PDU Specific
    pub fn ps(&self) -> u8 {
        ((self.0 >> 8) & 0xff).try_into().unwrap()
    }

    /// Device source address
    pub fn sa(&self) -> u8 {
        (self.0 & 0xff).try_into().unwrap()
    }
}

impl core::fmt::Display for Id {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(da) = self.destination_address() {
            write!(
                f,
                "[0x{:X?}] Prio: {} PGN: {} DA: {}",
                self.as_raw(),
                self.priority(),
                self.pgn(),
                da
            )
        } else {
            write!(
                f,
                "[0x{:X?}] Pri: {} PGN: {}",
                self.as_raw(),
                self.priority(),
                self.pgn()
            )
        }
    }
}

pub struct IdBuilder {
    priority: u8,
    pgn: u16,
    sa: u8,
    da: u8,
}

impl IdBuilder {
    pub fn from_pgn(pgn: u16) -> Self {
        Self {
            priority: 6,
            pgn,
            sa: 0,
            da: 0,
        }
    }

    /// Set the priority
    pub fn priority(mut self, priority: u8) -> Self {
        self.priority = priority.min(7);
        self
    }

    /// Set the sender address
    pub fn sa(mut self, address: u8) -> Self {
        self.sa = address;
        self
    }

    /// Set the destination address
    pub fn da(mut self, address: u8) -> Self {
        self.da = address;
        self
    }

    pub fn build(self) -> Id {
        let mut id = (self.priority as u32) << 26 | (self.pgn as u32) << 8 | self.sa as u32;

        if let PDUFormat::PDU1(_) = Id::new(id).pf() {
            id |= (self.da as u32) << 8;
        }

        Id::new(id)
    }
}

/// Data frame.
pub struct Frame {
    /// Frame ID.
    id: Id,
    /// PDU.
    pdu: [u8; 8],
}

impl Frame {
    /// Construct new frame.
    pub fn new(id: Id, pdu: [u8; 8]) -> Self {
        Self { id, pdu }
    }

    /// Get frame ID.
    pub fn id(&self) -> &Id {
        &self.id
    }

    /// Get PDU reference.
    pub fn pdu(&self) -> &[u8] {
        &self.pdu[..]
    }
}

impl core::fmt::Display for Frame {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}    {:X?}", self.id, self.pdu)
    }
}

pub struct FrameBuilder {
    id: Id,
    pdu: [u8; 8],
}

impl Default for FrameBuilder {
    fn default() -> Self {
        Self {
            id: Id::new(0),
            pdu: [0; 8],
        }
    }
}

impl FrameBuilder {
    pub fn id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    pub fn pdu_mut_ref(&mut self) -> &mut [u8] {
        &mut self.pdu[..]
    }

    pub fn build(self) -> Frame {
        Frame::new(self.id, self.pdu)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Id, IdBuilder, PDUFormat};

    #[test]
    fn id_decode_1() {
        let id = Id::new(0x18EAFF00);

        assert_eq!(id.as_raw(), 0x18EAFF00);
        assert_eq!(id.priority(), 6);
        assert_eq!(id.dp(), 0);
        assert_eq!(id.pgn(), 59904);
        assert_eq!(id.pf(), PDUFormat::PDU1(234));
        assert_eq!(id.is_broadcast(), false);
        assert_eq!(id.ps(), 255);
        assert_eq!(id.destination_address(), Some(255));
        assert_eq!(id.group_extension(), None);
        assert_eq!(id.sa(), 0);
    }

    #[test]
    fn id_decode_2() {
        let id = Id::new(0xCFE6CEE);

        assert_eq!(id.as_raw(), 0xCFE6CEE);
        assert_eq!(id.priority(), 3);
        assert_eq!(id.dp(), 0);
        assert_eq!(id.pgn(), 65132);
        assert_eq!(id.pf(), PDUFormat::PDU2(254));
        assert_eq!(id.is_broadcast(), true);
        assert_eq!(id.ps(), 108);
        assert_eq!(id.destination_address(), None);
        assert_eq!(id.group_extension(), Some(108));
        assert_eq!(id.sa(), 238);
    }

    #[test]
    fn id_decode_3() {
        let id = Id::new(0xDFE6CEE);

        assert_eq!(id.as_raw(), 0xDFE6CEE);
        assert_eq!(id.priority(), 3);
        assert_eq!(id.dp(), 1);
        assert_eq!(id.pgn(), 65132);
        assert_eq!(id.pf(), PDUFormat::PDU2(254));
        assert_eq!(id.is_broadcast(), true);
        assert_eq!(id.ps(), 108);
        assert_eq!(id.destination_address(), None);
        assert_eq!(id.group_extension(), Some(108));
        assert_eq!(id.sa(), 238);
    }

    #[test]
    fn id_build_1() {
        let id = IdBuilder::from_pgn(51712).priority(3).sa(139).build();

        assert_eq!(id, Id::new(0xCCA008B));
    }

    #[test]
    fn id_build_2() {
        let id = IdBuilder::from_pgn(51712)
            .priority(3)
            .da(0x34)
            .sa(139)
            .build();

        assert_eq!(id, Id::new(0xCCA348B));
    }

    #[test]
    fn id_build_3() {
        let id = IdBuilder::from_pgn(65271).sa(234).build();

        assert_eq!(id, Id::new(0x18FEF7EA));
    }
}

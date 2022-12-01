#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]

// TODO: Make this a feature
pub mod decode;

mod pgn;
pub mod protocol;

pub use pgn::*;

#[cfg(feature = "can")]
pub mod can;

#[derive(Debug, PartialEq, Eq)]
pub enum PDUFormat {
    PDU1(u8),
    PDU2(u8),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Id(u32);

/// Frame ID
impl Id {
    /// Construct new Frame ID.
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    /// Return ID as raw integer.
    pub const fn as_raw(&self) -> u32 {
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

    /// Data page
    pub fn dp(&self) -> u8 {
        ((self.0 >> 24) & 0x1).try_into().unwrap()
    }

    /// Parameter Group Number
    pub fn pgn(&self) -> PGN {
        self.pgn_raw().into()
    }

    // TODO: Should return u32.
    /// Parameter Group Number
    pub fn pgn_raw(&self) -> u16 {
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
        matches!(self.pf(), PDUFormat::PDU2(_))
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
                "[0x{:X?}] Prio: {} PGN: {} DA: 0x{:X?}",
                self.as_raw(),
                self.priority(),
                self.pgn_raw(),
                da
            )
        } else {
            write!(
                f,
                "[0x{:X?}] Prio: {} PGN: {}",
                self.as_raw(),
                self.priority(),
                self.pgn_raw()
            )
        }
    }
}

pub struct IdBuilder {
    /// Message priority.
    priority: u8,
    /// Parameter group number.
    pgn: u16,
    /// Source address.
    sa: u8,
    /// Destination address.
    da: u8,
}

impl IdBuilder {
    /// Construct ID builder from PGN.
    pub fn from_pgn(pgn: PGN) -> Self {
        Self {
            priority: 6,
            pgn: pgn.into(),
            sa: 0,
            da: 0,
        }
    }

    /// Set the priority.
    pub fn priority(mut self, priority: u8) -> Self {
        self.priority = priority.min(7);
        self
    }

    /// Set the sender address.
    pub fn sa(mut self, address: u8) -> Self {
        self.sa = address;
        self
    }

    /// Set the destination address.
    pub fn da(mut self, address: u8) -> Self {
        self.da = address;
        self
    }

    /// Build frame ID.
    pub fn build(self) -> Id {
        let mut id = (self.priority as u32) << 26 | (self.pgn as u32) << 8 | self.sa as u32;

        if let PDUFormat::PDU1(_) = Id::new(id).pf() {
            id |= (self.da as u32) << 8;
        }

        Id::new(id)
    }
}

/// Data frame.
#[derive(Clone, Copy, Debug)]
pub struct Frame {
    /// Frame ID.
    id: Id,
    /// PDU.
    pdu: [u8; 8],
    /// PDU length.
    pdu_length: usize,
}

impl Frame {
    /// Construct new frame.
    pub fn new(id: Id, pdu: [u8; 8]) -> Self {
        Self {
            id,
            pdu,
            pdu_length: 8,
        }
    }

    /// Get frame ID.
    pub fn id(&self) -> &Id {
        &self.id
    }

    /// Get PDU reference.
    pub fn pdu(&self) -> &[u8] {
        &self.pdu[..self.pdu_length]
    }

    /// PDU data length.
    pub fn len(&self) -> usize {
        self.pdu_length
    }

    /// Check if PDU data is empty.
    pub fn is_empty(&self) -> bool {
        self.pdu_length == 0
    }
}

impl core::fmt::Display for Frame {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}    {:02X?}", self.id(), self.pdu())
    }
}

impl AsRef<[u8]> for Frame {
    fn as_ref(&self) -> &[u8] {
        &self.pdu[..self.pdu_length]
    }
}

pub struct FrameBuilder {
    /// Frame ID.
    id: Id,
    /// PDU.
    pdu: [u8; 8],
    /// PDU length.
    pdu_length: usize,
}

impl Default for FrameBuilder {
    fn default() -> Self {
        Self {
            id: Id::new(0),
            pdu: [0xff; 8],
            pdu_length: 0,
        }
    }
}

impl FrameBuilder {
    /// Construct new frame builder.
    pub fn new(id: Id) -> Self {
        Self::default().id(id)
    }

    /// Set the frame ID.
    pub fn id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    /// Copy PDU data from slice.
    ///
    /// A runtime error will occur if the source slice is
    /// larger than the PDU.
    pub fn copy_from_slice(mut self, src: &[u8]) -> Self {
        self.pdu[..src.len()].copy_from_slice(src);
        self.pdu_length = src.len();
        self
    }

    /// Set PDU length.
    pub fn set_len(mut self, len: usize) -> Self {
        self.pdu_length = len;
        self
    }

    /// Construct frame.
    pub fn build(self) -> Frame {
        Frame {
            id: self.id,
            pdu: self.pdu,
            pdu_length: self.pdu_length,
        }
    }
}

impl AsMut<[u8]> for FrameBuilder {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.pdu
    }
}

#[cfg(test)]
mod tests {
    use crate::{FrameBuilder, Id, IdBuilder, PDUFormat, PGN};

    #[test]
    fn id_decode_1() {
        let id = Id::new(0x18EAFF00);

        assert_eq!(id.as_raw(), 0x18EAFF00);
        assert_eq!(id.priority(), 6);
        assert_eq!(id.dp(), 0);
        assert_eq!(id.pgn_raw(), 59904);
        assert_eq!(id.pgn(), PGN::Request);
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
        assert_eq!(id.pgn_raw(), 65132);
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
        assert_eq!(id.pgn_raw(), 65132);
        assert_eq!(id.pf(), PDUFormat::PDU2(254));
        assert_eq!(id.is_broadcast(), true);
        assert_eq!(id.ps(), 108);
        assert_eq!(id.destination_address(), None);
        assert_eq!(id.group_extension(), Some(108));
        assert_eq!(id.sa(), 238);
    }

    #[test]
    fn id_build_1() {
        let id = IdBuilder::from_pgn(PGN::Transfer.into())
            .priority(3)
            .sa(139)
            .build();

        assert_eq!(id, Id::new(0xCCA008B));
    }

    #[test]
    fn id_build_2() {
        let id = IdBuilder::from_pgn(PGN::Transfer.into())
            .priority(3)
            .da(0x34)
            .sa(139)
            .build();

        assert_eq!(id, Id::new(0xCCA348B));
    }

    #[test]
    fn id_build_3() {
        let id = IdBuilder::from_pgn(PGN::ElectronicEngineController2.into())
            .priority(3)
            .da(0)
            .sa(12)
            .build();

        assert_eq!(id, Id::new(0xCF0040C));
    }

    #[test]
    fn id_build_4() {
        let id = IdBuilder::from_pgn(PGN::VehicleElectricalPower1.into())
            .sa(234)
            .build();

        assert_eq!(id, Id::new(0x18FEF7EA));
    }

    // #[test]
    // fn id_build_5() {
    //     let id = IdBuilder::from_pgn(126720)
    //         .sa(234)
    //         .build();

    //     assert_eq!(id, Id::new(0x18FEF7EA));
    // }

    #[test]
    fn frame_build_1() {
        let frame = FrameBuilder::new(
            IdBuilder::from_pgn(PGN::Request.into())
                .da(0x20)
                .sa(0x10)
                .build(),
        )
        .copy_from_slice(&[0x1, 0x2, 0x3])
        .build();

        assert_eq!(frame.id(), &Id::new(0x18EA2010));
        assert_eq!(frame.pdu(), &[0x1, 0x2, 0x3]);
        assert_eq!(frame.len(), 3);
        assert_eq!(frame.is_empty(), false);
    }

    #[test]
    fn frame_build_2() {
        let frame = FrameBuilder::new(
            IdBuilder::from_pgn(PGN::AddressClaimed.into())
                .priority(3)
                .sa(0x10)
                .build(),
        )
        .copy_from_slice(&[0xff; 8])
        .build();

        assert_eq!(frame.id(), &Id::new(0xCEE0010));
        assert_eq!(frame.pdu(), &[0xff; 8]);
        assert_eq!(frame.len(), 8);
        assert_eq!(frame.is_empty(), false);
    }

    #[test]
    fn frame_build_3() {
        let frame = FrameBuilder::new(IdBuilder::from_pgn(PGN::Transfer.into()).build()).build();

        assert_eq!(frame.id(), &Id::new(0x18CA0000));
        assert_eq!(frame.pdu(), &[]);
        assert_eq!(frame.len(), 0);
        assert_eq!(frame.is_empty(), true);
    }

    #[test]
    fn frame_build_4() {
        let mut frame_builder =
            FrameBuilder::default().id(IdBuilder::from_pgn(PGN::Transfer.into()).build());

        frame_builder
            .as_mut()
            .copy_from_slice(&[0x8, 0x7, 0x6, 0x5, 0x4, 0x3, 0x2, 0x1]);

        frame_builder = frame_builder.set_len(8);

        let frame = frame_builder.build();

        assert_eq!(frame.id(), &Id::new(0x18CA0000));
        assert_eq!(frame.pdu(), &[0x8, 0x7, 0x6, 0x5, 0x4, 0x3, 0x2, 0x1]);
        assert_eq!(frame.len(), 8);
        assert_eq!(frame.is_empty(), false);
    }
}

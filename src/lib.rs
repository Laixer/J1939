#![no_std]

#[derive(Debug, PartialEq)]
pub enum PDUFormat {
    PDU1(u8),
    PDU2(u8),
}

#[derive(Debug, PartialEq)]
pub struct Id(u32);

impl Id {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

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

    /// Extended data page
    pub fn edp(&self) -> u8 {
        ((self.0 >> 25) & 0x1).try_into().unwrap()
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

    /// PDU Specific
    pub fn ps(&self) -> u8 {
        ((self.0 >> 8) & 0xff).try_into().unwrap()
    }

    /// Device source address
    pub fn sa(&self) -> u8 {
        (self.0 & 0xff).try_into().unwrap()
    }
}

struct IdBuilder {
    priority: u8,
    pgn: u16,
    sa: u8,
}

impl IdBuilder {
    pub fn from_pgn(pgn: u16) -> Self {
        Self {
            priority: 6,
            pgn,
            sa: 0,
        }
    }

    pub fn priority(mut self, priority: u8) -> IdBuilder {
        self.priority = priority.min(7);
        self
    }

    pub fn build(self) -> Id {
        let id = (self.priority as u32) << 26 | (self.pgn as u32) << 8;

        Id::new(id)
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
        assert_eq!(id.edp(), 0);
        assert_eq!(id.dp(), 0);
        assert_eq!(id.pgn(), 59904);
        assert_eq!(id.pf(), PDUFormat::PDU1(234));
        assert_eq!(id.is_broadcast(), false);
        assert_eq!(id.ps(), 255);
        assert_eq!(id.sa(), 0);
    }

    #[test]
    fn id_decode_2() {
        let id = Id::new(0xCFE6CEE);

        assert_eq!(id.as_raw(), 0xCFE6CEE);
        assert_eq!(id.priority(), 3);
        assert_eq!(id.edp(), 0);
        assert_eq!(id.dp(), 0);
        assert_eq!(id.pgn(), 65132);
        assert_eq!(id.pf(), PDUFormat::PDU2(254));
        assert_eq!(id.is_broadcast(), true);
        assert_eq!(id.ps(), 108);
        assert_eq!(id.sa(), 238);
    }

    #[test]
    fn id_decode_3() {
        let id = Id::new(0xEFE6CEE);

        assert_eq!(id.as_raw(), 0xEFE6CEE);
        assert_eq!(id.priority(), 3);
        assert_eq!(id.edp(), 1);
        assert_eq!(id.dp(), 0);
        assert_eq!(id.pgn(), 65132);
        assert_eq!(id.pf(), PDUFormat::PDU2(254));
        assert_eq!(id.is_broadcast(), true);
        assert_eq!(id.ps(), 108);
        assert_eq!(id.sa(), 238);
    }

    #[test]
    fn id_decode_4() {
        let id = Id::new(0xDFE6CEE);

        assert_eq!(id.as_raw(), 0xDFE6CEE);
        assert_eq!(id.priority(), 3);
        assert_eq!(id.edp(), 0);
        assert_eq!(id.dp(), 1);
        assert_eq!(id.pgn(), 65132);
        assert_eq!(id.pf(), PDUFormat::PDU2(254));
        assert_eq!(id.is_broadcast(), true);
        assert_eq!(id.ps(), 108);
        assert_eq!(id.sa(), 238);
    }

    #[test]
    fn id_build_1() {
        let id = IdBuilder::from_pgn(65247).build();

        assert_eq!(id, Id::new(0x18FEDF00));
    }
}

#![no_std]

#[derive(Debug, PartialEq)]
pub enum PDUFormat {
    PDU1(u8),
    PDU2(u8),
}

pub struct Id(u32);

impl Id {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn as_raw(&self) -> u32 {
        self.0
    }

    pub fn priority(&self) -> u8 {
        (self.0 >> 26).try_into().unwrap()
    }

    pub fn edp(&self) -> u8 {
        ((self.0 >> 25) & 0x1).try_into().unwrap()
    }

    pub fn dp(&self) -> u8 {
        ((self.0 >> 24) & 0x1).try_into().unwrap()
    }

    pub fn pgn(&self) -> u16 {
        match self.pf() {
            PDUFormat::PDU1(_) => (self.0 >> 8) & 0xff00,
            PDUFormat::PDU2(_) => (self.0 >> 8) & 0xffff,
        }
        .try_into()
        .unwrap()
    }

    pub fn pf(&self) -> PDUFormat {
        let format: u8 = ((self.0 >> 16) & 0xff).try_into().unwrap();
        if format < 240 {
            PDUFormat::PDU1(format)
        } else {
            PDUFormat::PDU2(format)
        }
    }

    pub fn is_broadcast(&self) -> bool {
        match self.pf() {
            PDUFormat::PDU2(_) => true,
            _ => false,
        }
    }

    pub fn ps(&self) -> u8 {
        ((self.0 >> 8) & 0xff).try_into().unwrap()
    }

    pub fn sa(&self) -> u8 {
        (self.0 & 0xff).try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::PDUFormat;

    #[test]
    fn id_decode_1() {
        let id = crate::Id::new(0x18EAFF00);

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
        let id = crate::Id::new(0xCFE6CEE);

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
        let id = crate::Id::new(0xEFE6CEE);

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
        let id = crate::Id::new(0xDFE6CEE);

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
}

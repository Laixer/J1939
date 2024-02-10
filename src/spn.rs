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
    pub fn from_pdu(pdu: &[u8]) -> Self {
        Self {
            year: pdu[5] as i32 + 1985,
            month: pdu[3] as u32,
            day: (pdu[4] as f32 * 0.25) as u32,
            hour: pdu[2] as u32,
            minute: pdu[1] as u32,
            second: (pdu[0] as f32 * 0.25) as u32,
        }
    }

    pub fn to_pdu(&self) -> [u8; 8] {
        [
            (self.second * 4) as u8,
            self.minute as u8,
            self.hour as u8,
            self.month as u8,
            (self.day * 4) as u8,
            (self.year - 1985) as u8,
            0xff, // TODO: Add timezone
            0xff, // TODO: Add timezone
        ]
    }
}

pub struct EngineControllerMessage {
    /// Engine Torque Mode.
    pub engine_torque_mode: Option<crate::decode::EngineTorqueMode>,
    /// Driver's Demand Engine - Percent Torque.
    pub driver_demand: Option<u8>,
    /// Actual Engine - Percent Torque.
    pub actual_engine: Option<u8>,
    /// Engine Speed.
    pub rpm: Option<u16>,
    /// Source Address of Controlling Device for Engine Control.
    pub source_addr: Option<u8>,
    /// Engine Starter Mode.
    pub starter_mode: Option<crate::decode::EngineStarterMode>,
}

impl EngineControllerMessage {
    pub fn from_pdu(pdu: &[u8]) -> Self {
        Self {
            engine_torque_mode: crate::decode::spn899(pdu[0]),
            driver_demand: byte::dec(pdu[1]),
            actual_engine: byte::dec(pdu[2]),
            rpm: rpm::dec(&pdu[3..5]),
            source_addr: crate::decode::spn1483(pdu[5]),
            starter_mode: crate::decode::spn1675(pdu[6]),
        }
    }

    pub fn to_pdu(&self) -> [u8; 8] {
        [
            0xff, // TODO: Add engine torque mode
            self.driver_demand.map_or(0xff, byte::enc),
            self.actual_engine.map_or(0xff, byte::enc),
            self.rpm.map_or([0xff, 0xff], rpm::enc)[0],
            self.rpm.map_or([0xff, 0xff], rpm::enc)[1],
            self.source_addr.unwrap_or(0xff),
            0xff,
            0xff,
        ]
    }
}

impl core::fmt::Display for EngineControllerMessage {
    // TODO: Implement Display
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "EngineControllerMessage")
    }
}

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
    /// Engine Torque Mode - SPN 899.
    pub engine_torque_mode: Option<crate::decode::EngineTorqueMode>,
    /// Driver's Demand Engine - Percent Torque.
    pub driver_demand: Option<u8>,
    /// Actual Engine - Percent Torque.
    pub actual_engine: Option<u8>,
    /// Engine Speed.
    pub rpm: Option<u16>,
    /// Source Address of Controlling Device for Engine Control - SPN 1483.
    pub source_addr: Option<u8>,
    /// Engine Starter Mode - SPN 1675.
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
        let engine_torque_mode = match self.engine_torque_mode {
            Some(crate::decode::EngineTorqueMode::NoRequest) => 0b0000,
            Some(crate::decode::EngineTorqueMode::AcceleratorPedal) => 0b0001,
            Some(crate::decode::EngineTorqueMode::CruiseControl) => 0b0010,
            Some(crate::decode::EngineTorqueMode::PTOGovernor) => 0b0011,
            Some(crate::decode::EngineTorqueMode::RoadSpeedGovernor) => 0b0100,
            Some(crate::decode::EngineTorqueMode::ASRControl) => 0b0101,
            Some(crate::decode::EngineTorqueMode::TransmissionControl) => 0b0110,
            Some(crate::decode::EngineTorqueMode::ABSControl) => 0b0111,
            Some(crate::decode::EngineTorqueMode::TorqueLimiting) => 0b1000,
            Some(crate::decode::EngineTorqueMode::HighSpeedGovernor) => 0b1001,
            Some(crate::decode::EngineTorqueMode::BrakingSystem) => 0b1010,
            Some(crate::decode::EngineTorqueMode::RemoteAccelerator) => 0b1011,
            Some(crate::decode::EngineTorqueMode::Other) => 0b1111,
            None => 0b1111,
        };

        let starter_mode = match self.starter_mode {
            Some(crate::decode::EngineStarterMode::StartNotRequested) => 0b0000,
            Some(crate::decode::EngineStarterMode::StarterActiveGearNotEngaged) => 0b0001,
            Some(crate::decode::EngineStarterMode::StarterActiveGearEngaged) => 0b0010,
            Some(crate::decode::EngineStarterMode::StartFinished) => 0b0011,
            Some(crate::decode::EngineStarterMode::StarterInhibitedEngineRunning) => 0b0100,
            Some(crate::decode::EngineStarterMode::StarterInhibitedEngineNotReady) => 0b0101,
            Some(crate::decode::EngineStarterMode::StarterInhibitedTransmissionInhibited) => 0b0110,
            Some(crate::decode::EngineStarterMode::StarterInhibitedActiveImmobilizer) => 0b0111,
            Some(crate::decode::EngineStarterMode::StarterInhibitedOverHeat) => 0b1000,
            Some(crate::decode::EngineStarterMode::StarterInhibitedReasonUnknown) => 0b1100,
            Some(crate::decode::EngineStarterMode::Error) => 0b1101,
            None => 0b1111,
        };

        [
            engine_torque_mode,
            self.driver_demand.map_or(0xff, byte::enc),
            self.actual_engine.map_or(0xff, byte::enc),
            self.rpm.map_or([0xff, 0xff], rpm::enc)[0],
            self.rpm.map_or([0xff, 0xff], rpm::enc)[1],
            self.source_addr.unwrap_or(0xff),
            starter_mode,
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

#[allow(dead_code)]
pub struct TorqueSpeedControlMessage {
    /// Override control mode - SPN 695
    pub override_control_mode: Option<crate::decode::OverrideControlMode>,
    /// Requested speed control conditions - SPN 696
    pub speed_control_condition: Option<crate::decode::RequestedSpeedControlCondition>,
    /// Override control mode priority - SPN 897
    pub control_mode_priority: Option<crate::decode::OverrideControlModePriority>,
    /// Requested speed or speed limit - SPN 898
    pub speed: Option<u16>,
    /// Requested torque or torque limit - SPN 518
    pub torque: Option<u8>,
}

impl TorqueSpeedControlMessage {
    pub fn from_pdu(pdu: &[u8]) -> Self {
        Self {
            override_control_mode: crate::decode::spn695(pdu[0]),
            speed_control_condition: None, // TODO: Add SPN 696
            control_mode_priority: None,   // TODO: Add SPN 897
            speed: rpm::dec(&pdu[1..3]),
            torque: if pdu[3] != 0xff { Some(pdu[3]) } else { None },
        }
    }

    pub fn to_pdu(&self) -> [u8; 8] {
        let override_control_mode = match self.override_control_mode {
            Some(crate::decode::OverrideControlMode::OverrideDisabled) => 0b00,
            Some(crate::decode::OverrideControlMode::SpeedControl) => 0b01,
            Some(crate::decode::OverrideControlMode::TorqueControl) => 0b10,
            Some(crate::decode::OverrideControlMode::SpeedTorqueLimitControl) => 0b11,
            None => 0b00,
        };

        [
            override_control_mode, // TODO: Incomplete, add speed_control_condition, control_mode_priority
            self.speed.map_or(0xff, |speed| rpm::enc(speed)[0]),
            self.speed.map_or(0xff, |speed| rpm::enc(speed)[1]),
            self.torque.unwrap_or(0xff),
            0xff,
            0xff,
            0xff,
            0xff,
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::decode::{EngineStarterMode, EngineTorqueMode};

    use super::*;

    #[test]
    fn engine_controller_message_1() {
        let engine_message =
            EngineControllerMessage::from_pdu(&[0xF0, 0xEA, 0x7D, 0x00, 0x00, 0x00, 0xF0, 0xFF]);
        assert_eq!(
            engine_message.engine_torque_mode,
            Some(EngineTorqueMode::NoRequest)
        );
        assert_eq!(engine_message.driver_demand, Some(109));
        assert_eq!(engine_message.actual_engine, Some(0));
        assert_eq!(engine_message.rpm, Some(0));
        assert_eq!(engine_message.source_addr, Some(0));
        assert_eq!(
            engine_message.starter_mode,
            Some(EngineStarterMode::StartNotRequested)
        );
    }

    #[test]
    fn engine_controller_message_2() {
        let engine_message =
            EngineControllerMessage::from_pdu(&[0xF3, 0x91, 0x91, 0xAA, 0x18, 0x00, 0xF3, 0xFF]);
        assert_eq!(
            engine_message.engine_torque_mode,
            Some(EngineTorqueMode::PTOGovernor)
        );
        assert_eq!(engine_message.driver_demand, Some(20));
        assert_eq!(engine_message.actual_engine, Some(20));
        assert_eq!(engine_message.rpm, Some(789));
        assert_eq!(engine_message.source_addr, Some(0));
        assert_eq!(
            engine_message.starter_mode,
            Some(EngineStarterMode::StartFinished)
        );
    }

    #[test]
    fn engine_controller_message_3() {
        let engine_message_encoded = EngineControllerMessage {
            engine_torque_mode: Some(EngineTorqueMode::HighSpeedGovernor),
            driver_demand: Some(93),
            actual_engine: Some(4),
            rpm: Some(2156),
            source_addr: Some(21),
            starter_mode: Some(EngineStarterMode::StarterInhibitedOverHeat),
        }
        .to_pdu();
        let engine_message_decoded = EngineControllerMessage::from_pdu(&engine_message_encoded);

        assert_eq!(
            engine_message_decoded.engine_torque_mode,
            Some(EngineTorqueMode::HighSpeedGovernor)
        );
        assert_eq!(engine_message_decoded.driver_demand, Some(93));
        assert_eq!(engine_message_decoded.actual_engine, Some(4));
        assert_eq!(engine_message_decoded.rpm, Some(2156));
        assert_eq!(engine_message_decoded.source_addr, Some(21));
        assert_eq!(
            engine_message_decoded.starter_mode,
            Some(EngineStarterMode::StarterInhibitedOverHeat)
        );
    }
}

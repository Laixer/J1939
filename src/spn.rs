use crate::PDU_NOT_AVAILABLE;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EngineTorqueMode {
    NoRequest,
    AcceleratorPedal,
    CruiseControl,
    PTOGovernor,
    RoadSpeedGovernor,
    ASRControl,
    TransmissionControl,
    ABSControl,
    TorqueLimiting,
    HighSpeedGovernor,
    BrakingSystem,
    RemoteAccelerator,
    Other,
}

impl EngineTorqueMode {
    pub fn from_value(value: u8) -> Option<EngineTorqueMode> {
        if value != PDU_NOT_AVAILABLE {
            let mode = match value & 0b1111 {
                0b0000 => EngineTorqueMode::NoRequest,
                0b0001 => EngineTorqueMode::AcceleratorPedal,
                0b0010 => EngineTorqueMode::CruiseControl,
                0b0011 => EngineTorqueMode::PTOGovernor,
                0b0100 => EngineTorqueMode::RoadSpeedGovernor,
                0b0101 => EngineTorqueMode::ASRControl,
                0b0110 => EngineTorqueMode::TransmissionControl,
                0b0111 => EngineTorqueMode::ABSControl,
                0b1000 => EngineTorqueMode::TorqueLimiting,
                0b1001 => EngineTorqueMode::HighSpeedGovernor,
                0b1010 => EngineTorqueMode::BrakingSystem,
                0b1011 => EngineTorqueMode::RemoteAccelerator,
                0b1100..=0b1110 => EngineTorqueMode::Other,
                _ => unreachable!(),
            };

            Some(mode)
        } else {
            None
        }
    }

    pub fn to_value(mode: Self) -> u8 {
        match mode {
            EngineTorqueMode::NoRequest => 0b0000,
            EngineTorqueMode::AcceleratorPedal => 0b0001,
            EngineTorqueMode::CruiseControl => 0b0010,
            EngineTorqueMode::PTOGovernor => 0b0011,
            EngineTorqueMode::RoadSpeedGovernor => 0b0100,
            EngineTorqueMode::ASRControl => 0b0101,
            EngineTorqueMode::TransmissionControl => 0b0110,
            EngineTorqueMode::ABSControl => 0b0111,
            EngineTorqueMode::TorqueLimiting => 0b1000,
            EngineTorqueMode::HighSpeedGovernor => 0b1001,
            EngineTorqueMode::BrakingSystem => 0b1010,
            EngineTorqueMode::RemoteAccelerator => 0b1011,
            EngineTorqueMode::Other => 0b1111,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EngineStarterMode {
    StartNotRequested,
    StarterActiveGearNotEngaged,
    StarterActiveGearEngaged,
    StartFinished,
    StarterInhibitedEngineRunning,
    StarterInhibitedEngineNotReady,
    StarterInhibitedTransmissionInhibited,
    StarterInhibitedActiveImmobilizer,
    StarterInhibitedOverHeat,
    StarterInhibitedReasonUnknown,
    Error,
}

impl EngineStarterMode {
    pub fn from_value(value: u8) -> Option<Self> {
        if value != PDU_NOT_AVAILABLE {
            let mode = match value & 0b1111 {
                0b0000 => EngineStarterMode::StartNotRequested,
                0b0001 => EngineStarterMode::StarterActiveGearNotEngaged,
                0b0010 => EngineStarterMode::StarterActiveGearEngaged,
                0b0011 => EngineStarterMode::StartFinished,
                0b0100 => EngineStarterMode::StarterInhibitedEngineRunning,
                0b0101 => EngineStarterMode::StarterInhibitedEngineNotReady,
                0b0110 => EngineStarterMode::StarterInhibitedTransmissionInhibited,
                0b0111 => EngineStarterMode::StarterInhibitedActiveImmobilizer,
                0b1000 => EngineStarterMode::StarterInhibitedOverHeat,
                0b1100 => EngineStarterMode::StarterInhibitedReasonUnknown,
                0b1101 | 0b1110 => EngineStarterMode::Error,
                _ => unreachable!(),
            };

            Some(mode)
        } else {
            None
        }
    }

    pub fn to_value(mode: Self) -> u8 {
        match mode {
            EngineStarterMode::StartNotRequested => 0b0000,
            EngineStarterMode::StarterActiveGearNotEngaged => 0b0001,
            EngineStarterMode::StarterActiveGearEngaged => 0b0010,
            EngineStarterMode::StartFinished => 0b0011,
            EngineStarterMode::StarterInhibitedEngineRunning => 0b0100,
            EngineStarterMode::StarterInhibitedEngineNotReady => 0b0101,
            EngineStarterMode::StarterInhibitedTransmissionInhibited => 0b0110,
            EngineStarterMode::StarterInhibitedActiveImmobilizer => 0b0111,
            EngineStarterMode::StarterInhibitedOverHeat => 0b1000,
            EngineStarterMode::StarterInhibitedReasonUnknown => 0b1100,
            EngineStarterMode::Error => 0b1101,
        }
    }
}

pub struct ElectronicEngineController1Message {
    /// Engine Torque Mode - SPN 899.
    pub engine_torque_mode: Option<EngineTorqueMode>,
    /// Driver's Demand Engine - Percent Torque.
    pub driver_demand: Option<u8>,
    /// Actual Engine - Percent Torque.
    pub actual_engine: Option<u8>,
    /// Engine Speed.
    pub rpm: Option<u16>,
    /// Source Address of Controlling Device for Engine Control - SPN 1483.
    pub source_addr: Option<u8>,
    /// Engine Starter Mode - SPN 1675.
    pub starter_mode: Option<EngineStarterMode>,
}

impl ElectronicEngineController1Message {
    pub fn from_pdu(pdu: &[u8]) -> Self {
        Self {
            engine_torque_mode: EngineTorqueMode::from_value(pdu[0]),
            driver_demand: byte::dec(pdu[1]),
            actual_engine: byte::dec(pdu[2]),
            rpm: rpm::dec(&pdu[3..5]),
            source_addr: crate::decode::spn1483(pdu[5]),
            starter_mode: EngineStarterMode::from_value(pdu[6]),
        }
    }

    pub fn to_pdu(&self) -> [u8; 8] {
        [
            self.engine_torque_mode
                .map_or(PDU_NOT_AVAILABLE, EngineTorqueMode::to_value),
            self.driver_demand.map_or(PDU_NOT_AVAILABLE, byte::enc),
            self.actual_engine.map_or(PDU_NOT_AVAILABLE, byte::enc),
            self.rpm.map_or([0xff, 0xff], rpm::enc)[0],
            self.rpm.map_or([0xff, 0xff], rpm::enc)[1],
            self.source_addr.unwrap_or(PDU_NOT_AVAILABLE),
            self.starter_mode
                .map_or(PDU_NOT_AVAILABLE, EngineStarterMode::to_value),
            PDU_NOT_AVAILABLE,
        ]
    }
}

impl core::fmt::Display for ElectronicEngineController1Message {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Torque mode: {:?}; Driver demand: {}%; Actual engine: {}%; RPM: {}; Starter mode: {:?}",
            self.engine_torque_mode,
            self.driver_demand.unwrap_or(0),
            self.actual_engine.unwrap_or(0),
            self.rpm.unwrap_or(0),
            self.starter_mode
        )
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
            override_control_mode: crate::decode::spn695(pdu[0] & 0b11),
            speed_control_condition: crate::decode::spn696(pdu[0] >> 2 & 0b11),
            control_mode_priority: crate::decode::spn897(pdu[0] >> 4 & 0b11),
            speed: rpm::dec(&pdu[1..3]),
            torque: if pdu[3] != PDU_NOT_AVAILABLE {
                Some(pdu[3])
            } else {
                None
            },
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

        let speed_control_condition = match self.speed_control_condition {
            Some(crate::decode::RequestedSpeedControlCondition::TransientOptimizedDriveLineDisengaged) => 0b00,
            Some(crate::decode::RequestedSpeedControlCondition::StabilityOptimizedDriveLineDisengaged) => 0b01,
            Some(crate::decode::RequestedSpeedControlCondition::StabilityOptimizedDriveLineEngaged1) => 0b10,
            Some(crate::decode::RequestedSpeedControlCondition::StabilityOptimizedDriveLineEngaged2) => 0b11,
            None => 0b00,
        };

        let control_mode_priority = match self.control_mode_priority {
            Some(crate::decode::OverrideControlModePriority::HighestPriority) => 0b00,
            Some(crate::decode::OverrideControlModePriority::HighPriority) => 0b01,
            Some(crate::decode::OverrideControlModePriority::MediumPriority) => 0b10,
            Some(crate::decode::OverrideControlModePriority::LowPriority) => 0b11,
            None => 0b00,
        };

        [
            override_control_mode | speed_control_condition << 2 | control_mode_priority << 4,
            self.speed
                .map_or(PDU_NOT_AVAILABLE, |speed| rpm::enc(speed)[0]),
            self.speed
                .map_or(PDU_NOT_AVAILABLE, |speed| rpm::enc(speed)[1]),
            self.torque.unwrap_or(PDU_NOT_AVAILABLE),
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
        ]
    }
}

impl core::fmt::Display for TorqueSpeedControlMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Override control mode: {:?}; Speed control condition: {:?}; Control mode priority: {:?}; Speed: {}; Torque: {}",
            self.override_control_mode,
            self.speed_control_condition,
            self.control_mode_priority,
            self.speed.unwrap_or(0),
            self.torque.unwrap_or(0)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_controller_message_1() {
        let engine_message = ElectronicEngineController1Message::from_pdu(&[
            0xF0, 0xEA, 0x7D, 0x00, 0x00, 0x00, 0xF0, 0xFF,
        ]);
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
        let engine_message = ElectronicEngineController1Message::from_pdu(&[
            0xF3, 0x91, 0x91, 0xAA, 0x18, 0x00, 0xF3, 0xFF,
        ]);
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
        let engine_message_encoded = ElectronicEngineController1Message {
            engine_torque_mode: Some(EngineTorqueMode::HighSpeedGovernor),
            driver_demand: Some(93),
            actual_engine: Some(4),
            rpm: Some(2156),
            source_addr: Some(21),
            starter_mode: Some(EngineStarterMode::StarterInhibitedOverHeat),
        }
        .to_pdu();
        let engine_message_decoded =
            ElectronicEngineController1Message::from_pdu(&engine_message_encoded);

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

    #[test]
    fn torque_speed_control_message_1() {
        let torque_speed_encoded = TorqueSpeedControlMessage {
            override_control_mode: Some(crate::decode::OverrideControlMode::SpeedControl),
            speed_control_condition: Some(
                crate::decode::RequestedSpeedControlCondition::StabilityOptimizedDriveLineEngaged1,
            ),
            control_mode_priority: Some(crate::decode::OverrideControlModePriority::MediumPriority),
            speed: Some(1234),
            torque: Some(56),
        }
        .to_pdu();
        let torque_speed_decoded = TorqueSpeedControlMessage::from_pdu(&torque_speed_encoded);

        assert_eq!(
            torque_speed_decoded.override_control_mode,
            Some(crate::decode::OverrideControlMode::SpeedControl)
        );
        assert_eq!(
            torque_speed_decoded.speed_control_condition,
            Some(
                crate::decode::RequestedSpeedControlCondition::StabilityOptimizedDriveLineEngaged1
            )
        );
        assert_eq!(
            torque_speed_decoded.control_mode_priority,
            Some(crate::decode::OverrideControlModePriority::MediumPriority)
        );
        assert_eq!(torque_speed_decoded.speed, Some(1234));
        assert_eq!(torque_speed_decoded.torque, Some(56));
    }
}

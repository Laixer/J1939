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

//
// TimeDate
//

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

//
// ElectronicEngineController1
//

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
    pub fn from_value(value: u8) -> Option<Self> {
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

//
// TorqueSpeedControl1
//

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OverrideControlMode {
    OverrideDisabled,
    SpeedControl,
    TorqueControl,
    SpeedTorqueLimitControl,
}

impl OverrideControlMode {
    pub fn from_value(value: u8) -> Option<Self> {
        if value != PDU_NOT_AVAILABLE {
            let mode = match value & 0b11 {
                0b00 => OverrideControlMode::OverrideDisabled,
                0b01 => OverrideControlMode::SpeedControl,
                0b10 => OverrideControlMode::TorqueControl,
                0b11 => OverrideControlMode::SpeedTorqueLimitControl,
                _ => unreachable!(),
            };

            Some(mode)
        } else {
            None
        }
    }

    pub fn to_value(mode: Self) -> u8 {
        match mode {
            OverrideControlMode::OverrideDisabled => 0b00,
            OverrideControlMode::SpeedControl => 0b01,
            OverrideControlMode::TorqueControl => 0b10,
            OverrideControlMode::SpeedTorqueLimitControl => 0b11,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RequestedSpeedControlCondition {
    TransientOptimizedDriveLineDisengaged,
    StabilityOptimizedDriveLineDisengaged,
    StabilityOptimizedDriveLineEngaged1,
    StabilityOptimizedDriveLineEngaged2,
}

impl RequestedSpeedControlCondition {
    pub fn from_value(value: u8) -> Option<Self> {
        if value != PDU_NOT_AVAILABLE {
            let condition = match value & 0b11 {
                0b00 => RequestedSpeedControlCondition::TransientOptimizedDriveLineDisengaged,
                0b01 => RequestedSpeedControlCondition::StabilityOptimizedDriveLineDisengaged,
                0b10 => RequestedSpeedControlCondition::StabilityOptimizedDriveLineEngaged1,
                0b11 => RequestedSpeedControlCondition::StabilityOptimizedDriveLineEngaged2,
                _ => unreachable!(),
            };

            Some(condition)
        } else {
            None
        }
    }

    pub fn to_value(condition: Self) -> u8 {
        match condition {
            RequestedSpeedControlCondition::TransientOptimizedDriveLineDisengaged => 0b00,
            RequestedSpeedControlCondition::StabilityOptimizedDriveLineDisengaged => 0b01,
            RequestedSpeedControlCondition::StabilityOptimizedDriveLineEngaged1 => 0b10,
            RequestedSpeedControlCondition::StabilityOptimizedDriveLineEngaged2 => 0b11,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OverrideControlModePriority {
    HighestPriority,
    HighPriority,
    MediumPriority,
    LowPriority,
}

impl OverrideControlModePriority {
    pub fn from_value(value: u8) -> Option<Self> {
        if value != PDU_NOT_AVAILABLE {
            let priority = match value & 0b11 {
                0b00 => OverrideControlModePriority::HighestPriority,
                0b01 => OverrideControlModePriority::HighPriority,
                0b10 => OverrideControlModePriority::MediumPriority,
                0b11 => OverrideControlModePriority::LowPriority,
                _ => unreachable!(),
            };

            Some(priority)
        } else {
            None
        }
    }

    pub fn to_value(priority: Self) -> u8 {
        match priority {
            OverrideControlModePriority::HighestPriority => 0b00,
            OverrideControlModePriority::HighPriority => 0b01,
            OverrideControlModePriority::MediumPriority => 0b10,
            OverrideControlModePriority::LowPriority => 0b11,
        }
    }
}

pub struct TorqueSpeedControl1Message {
    /// Override control mode - SPN 695
    pub override_control_mode: Option<OverrideControlMode>,
    /// Requested speed control conditions - SPN 696
    pub speed_control_condition: Option<RequestedSpeedControlCondition>,
    /// Override control mode priority - SPN 897
    pub control_mode_priority: Option<OverrideControlModePriority>,
    /// Requested speed or speed limit - SPN 898
    pub speed: Option<u16>,
    /// Requested torque or torque limit - SPN 518
    pub torque: Option<u8>,
}

impl TorqueSpeedControl1Message {
    pub fn from_pdu(pdu: &[u8]) -> Self {
        Self {
            override_control_mode: OverrideControlMode::from_value(pdu[0] & 0b11),
            speed_control_condition: RequestedSpeedControlCondition::from_value(pdu[0] >> 2 & 0b11),
            control_mode_priority: OverrideControlModePriority::from_value(pdu[0] >> 4 & 0b11),
            speed: rpm::dec(&pdu[1..3]),
            torque: if pdu[3] != PDU_NOT_AVAILABLE {
                Some(pdu[3])
            } else {
                None
            },
        }
    }

    pub fn to_pdu(&self) -> [u8; 8] {
        [
            self.override_control_mode
                .map_or(PDU_NOT_AVAILABLE, OverrideControlMode::to_value)
                | self
                    .speed_control_condition
                    .map_or(PDU_NOT_AVAILABLE, RequestedSpeedControlCondition::to_value)
                    << 2
                | self
                    .control_mode_priority
                    .map_or(PDU_NOT_AVAILABLE, OverrideControlModePriority::to_value)
                    << 4,
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

impl core::fmt::Display for TorqueSpeedControl1Message {
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

//
// AmbientConditions
//

// TODO: Not tested
pub struct AmbientConditionsMessage {
    /// Barometric pressure.
    pub barometric_pressure: Option<u8>,
    /// Cab interior temperature.
    pub cab_interior_temperature: Option<i16>,
    /// Ambient air temperature.
    pub ambient_air_temperature: Option<i16>,
    /// Air inlet temperature.
    pub air_inlet_temperature: Option<i8>,
    /// Road surface temperature.
    pub road_surface_temperature: Option<i16>,
}

impl AmbientConditionsMessage {
    pub fn from_pdu(pdu: &[u8]) -> Self {
        Self {
            barometric_pressure: if pdu[0] != PDU_NOT_AVAILABLE {
                Some((pdu[0] as f32 * 0.5) as u8)
            } else {
                None
            },
            cab_interior_temperature: if pdu[1] != PDU_NOT_AVAILABLE {
                Some(
                    ((i16::from_le_bytes([pdu[1], pdu[2]]) as f32 * 0.03125) as i16 - 273)
                        .clamp(-273, 1735),
                )
            } else {
                None
            },
            ambient_air_temperature: if pdu[3] != PDU_NOT_AVAILABLE {
                Some(
                    ((i16::from_le_bytes([pdu[3], pdu[4]]) as f32 * 0.03125) as i16 - 273)
                        .clamp(-273, 1735),
                )
            } else {
                None
            },
            air_inlet_temperature: if pdu[5] != PDU_NOT_AVAILABLE {
                Some((pdu[5] as i8 - 40).clamp(-40, 127))
            } else {
                None
            },
            road_surface_temperature: if pdu[6] != PDU_NOT_AVAILABLE {
                Some(
                    ((i16::from_le_bytes([pdu[6], pdu[7]]) as f32 * 0.03125) as i16 - 273)
                        .clamp(-273, 1735),
                )
            } else {
                None
            },
        }
    }

    // TODO: Add 2 bytes temperature conversion
    pub fn to_pdu(&self) -> [u8; 8] {
        [
            if let Some(pressure) = self.barometric_pressure {
                (pressure as f32 * 2.0) as u8
            } else {
                PDU_NOT_AVAILABLE
            },
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
            if let Some(temperature) = self.air_inlet_temperature {
                (temperature + 40) as u8
            } else {
                PDU_NOT_AVAILABLE
            },
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
        ]
    }
}

impl core::fmt::Display for AmbientConditionsMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Barometric pressure: {} kPa; Cab interior temperature: {}°C; Ambient air temperature: {}°C; Air inlet temperature: {}°C; Road surface temperature: {}°C",
            self.barometric_pressure.unwrap_or(0),
            self.cab_interior_temperature.unwrap_or(0),
            self.ambient_air_temperature.unwrap_or(0),
            self.air_inlet_temperature.unwrap_or(0),
            self.road_surface_temperature.unwrap_or(0)
        )
    }
}

//
// VehiclePosition
//

// TODO: Not tested
pub struct VehiclePositionMessage {
    /// Latitude.
    pub latitude: Option<f32>,
    /// Longitude.
    pub longitude: Option<f32>,
}

impl VehiclePositionMessage {
    pub fn from_pdu(pdu: &[u8]) -> Self {
        Self {
            latitude: if pdu[0] != PDU_NOT_AVAILABLE {
                Some((i32::from_le_bytes([pdu[0], pdu[1], pdu[2], pdu[3]]) - 210) as f32 * 1e-7)
            } else {
                None
            },
            longitude: if pdu[4] != PDU_NOT_AVAILABLE {
                Some((i32::from_le_bytes([pdu[4], pdu[5], pdu[6], pdu[7]]) - 210) as f32 * 1e-7)
            } else {
                None
            },
        }
    }

    pub fn to_pdu(&self) -> [u8; 8] {
        [
            if let Some(latitude) = self.latitude {
                ((latitude * 1e7) as i32 + 210).to_le_bytes()[0]
            } else {
                PDU_NOT_AVAILABLE
            },
            if let Some(latitude) = self.latitude {
                ((latitude * 1e7) as i32 + 210).to_le_bytes()[1]
            } else {
                PDU_NOT_AVAILABLE
            },
            if let Some(latitude) = self.latitude {
                ((latitude * 1e7) as i32 + 210).to_le_bytes()[2]
            } else {
                PDU_NOT_AVAILABLE
            },
            if let Some(latitude) = self.latitude {
                ((latitude * 1e7) as i32 + 210).to_le_bytes()[3]
            } else {
                PDU_NOT_AVAILABLE
            },
            if let Some(longitude) = self.longitude {
                ((longitude * 1e7) as i32 + 210).to_le_bytes()[0]
            } else {
                PDU_NOT_AVAILABLE
            },
            if let Some(longitude) = self.longitude {
                ((longitude * 1e7) as i32 + 210).to_le_bytes()[1]
            } else {
                PDU_NOT_AVAILABLE
            },
            if let Some(longitude) = self.longitude {
                ((longitude * 1e7) as i32 + 210).to_le_bytes()[2]
            } else {
                PDU_NOT_AVAILABLE
            },
            if let Some(longitude) = self.longitude {
                ((longitude * 1e7) as i32 + 210).to_le_bytes()[3]
            } else {
                PDU_NOT_AVAILABLE
            },
        ]
    }
}

impl core::fmt::Display for VehiclePositionMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Latitude: {:?}; Longitude: {:?}",
            self.latitude.unwrap_or(0.0),
            self.longitude.unwrap_or(0.0)
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
        let torque_speed_encoded = TorqueSpeedControl1Message {
            override_control_mode: Some(OverrideControlMode::SpeedControl),
            speed_control_condition: Some(
                RequestedSpeedControlCondition::StabilityOptimizedDriveLineEngaged1,
            ),
            control_mode_priority: Some(OverrideControlModePriority::MediumPriority),
            speed: Some(1234),
            torque: Some(56),
        }
        .to_pdu();
        let torque_speed_decoded = TorqueSpeedControl1Message::from_pdu(&torque_speed_encoded);

        assert_eq!(
            torque_speed_decoded.override_control_mode,
            Some(OverrideControlMode::SpeedControl)
        );
        assert_eq!(
            torque_speed_decoded.speed_control_condition,
            Some(RequestedSpeedControlCondition::StabilityOptimizedDriveLineEngaged1)
        );
        assert_eq!(
            torque_speed_decoded.control_mode_priority,
            Some(OverrideControlModePriority::MediumPriority)
        );
        assert_eq!(torque_speed_decoded.speed, Some(1234));
        assert_eq!(torque_speed_decoded.torque, Some(56));
    }
}

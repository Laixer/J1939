use crate::PDU_NOT_AVAILABLE;

// TODO: Obsolete, move into slots
// pub mod byte {
//     use crate::PDU_NOT_AVAILABLE;

//     pub fn enc(value: u8) -> u8 {
//         value + 125
//     }

//     pub fn dec(value: u8) -> Option<u8> {
//         if value != PDU_NOT_AVAILABLE {
//             Some(value - 125)
//         } else {
//             None
//         }
//     }
// }

// TODO: Obsolete
// pub mod rpm {
//     use crate::PDU_NOT_AVAILABLE;

//     pub fn enc(value: u16) -> [u8; 2] {
//         (value * 8).to_le_bytes()
//     }

//     pub fn dec(value: &[u8]) -> Option<u16> {
//         if value != [PDU_NOT_AVAILABLE, PDU_NOT_AVAILABLE] {
//             Some((u16::from_le_bytes([value[0], value[1]]) as f32 * 0.125) as u16)
//         } else {
//             None
//         }
//     }
// }

/// Scaling, Limit, Offsets and Transfer Functions
pub mod slots {
    pub mod rotational_velocity {
        use crate::PDU_NOT_AVAILABLE;

        const SCALE: f32 = 0.125;
        const _OFFSET: f32 = 0.0;
        const LIMIT_LOWER: f32 = 0.0;
        const LIMIT_UPPER: f32 = 8031.875;

        pub fn dec(value: [u8; 2]) -> Option<u16> {
            if value != [PDU_NOT_AVAILABLE, PDU_NOT_AVAILABLE] {
                Some(
                    (u16::from_le_bytes(value) as f32 * SCALE).clamp(LIMIT_LOWER, LIMIT_UPPER)
                        as u16,
                )
            } else {
                None
            }
        }

        pub fn enc(value: u16) -> [u8; 2] {
            (((value as f32).clamp(LIMIT_LOWER, LIMIT_UPPER) * 8.0) as u16).to_le_bytes()
        }
    }

    pub mod temperature {
        use crate::PDU_NOT_AVAILABLE;

        const SCALE: f32 = 0.03125;
        const OFFSET: f32 = -273.0;
        const LIMIT_LOWER: f32 = -273.0;
        const LIMIT_UPPER: f32 = 1735.0;

        pub fn dec(value: [u8; 2]) -> Option<i16> {
            if value != [PDU_NOT_AVAILABLE, PDU_NOT_AVAILABLE] {
                Some(
                    ((i16::from_le_bytes(value) as f32 * SCALE + OFFSET)
                        .clamp(LIMIT_LOWER, LIMIT_UPPER)) as i16,
                )
            } else {
                None
            }
        }

        pub fn enc(value: i16) -> [u8; 2] {
            let value = ((value as f32).clamp(LIMIT_LOWER, LIMIT_UPPER) - OFFSET) / SCALE;

            (value as i16).to_le_bytes()
        }
    }

    pub mod position_level {
        use crate::PDU_NOT_AVAILABLE;

        const SCALE: f32 = 0.4;
        const OFFSET: f32 = 0.0;
        const LIMIT_LOWER: f32 = 0.0;
        const LIMIT_UPPER: f32 = 100.0;

        pub fn dec(value: u8) -> Option<u8> {
            if value != PDU_NOT_AVAILABLE {
                Some((value as f32 * SCALE + OFFSET).clamp(LIMIT_LOWER, LIMIT_UPPER) as u8)
            } else {
                None
            }
        }

        pub fn enc(value: u8) -> u8 {
            (((value as f32).clamp(LIMIT_LOWER, LIMIT_UPPER) - OFFSET) / SCALE) as u8
        }
    }

    pub mod position_level2 {
        use crate::PDU_NOT_AVAILABLE;

        const SCALE: f32 = 1.0;
        const OFFSET: f32 = -125.0;
        const LIMIT_LOWER: f32 = -125.0;
        const LIMIT_UPPER: f32 = 125.0;

        pub fn dec(value: u8) -> Option<u8> {
            if value != PDU_NOT_AVAILABLE {
                Some((value as f32 * SCALE + OFFSET).clamp(LIMIT_LOWER, LIMIT_UPPER) as u8)
            } else {
                None
            }
        }

        pub fn enc(value: u8) -> u8 {
            (((value as f32).clamp(LIMIT_LOWER, LIMIT_UPPER) - OFFSET) / SCALE) as u8
        }
    }

    // TODO: Upper limit might be wrong
    pub mod pressure {
        use crate::PDU_NOT_AVAILABLE;

        const SCALE: f32 = 4.0;
        const OFFSET: f32 = 0.0;
        const LIMIT_LOWER: f32 = 0.0;
        const LIMIT_UPPER: f32 = 1000.0;

        pub fn dec(value: u8) -> Option<u8> {
            if value != PDU_NOT_AVAILABLE {
                Some((value as f32 * SCALE + OFFSET).clamp(LIMIT_LOWER, LIMIT_UPPER) as u8)
            } else {
                None
            }
        }

        pub fn enc(value: u8) -> u8 {
            (((value as f32).clamp(LIMIT_LOWER, LIMIT_UPPER) - OFFSET) / SCALE) as u8
        }
    }

    pub mod pressure2 {
        use crate::PDU_NOT_AVAILABLE;

        const SCALE: f32 = 0.05;
        const OFFSET: f32 = 0.0;
        const LIMIT_LOWER: f32 = 0.0;
        const LIMIT_UPPER: f32 = 12.5;

        pub fn dec(value: u8) -> Option<u8> {
            if value != PDU_NOT_AVAILABLE {
                Some((value as f32 * SCALE + OFFSET).clamp(LIMIT_LOWER, LIMIT_UPPER) as u8)
            } else {
                None
            }
        }

        pub fn enc(value: u8) -> u8 {
            (((value as f32).clamp(LIMIT_LOWER, LIMIT_UPPER) - OFFSET) / SCALE) as u8
        }
    }

    // TODO: Upper limit might be wrong
    pub mod pressure3 {
        use crate::PDU_NOT_AVAILABLE;

        const SCALE: f32 = 2.0;
        const OFFSET: f32 = 0.0;
        const LIMIT_LOWER: f32 = 0.0;
        const LIMIT_UPPER: f32 = 500.0;

        pub fn dec(value: u8) -> Option<u8> {
            if value != PDU_NOT_AVAILABLE {
                Some((value as f32 * SCALE + OFFSET).clamp(LIMIT_LOWER, LIMIT_UPPER) as u8)
            } else {
                None
            }
        }

        pub fn enc(value: u8) -> u8 {
            (((value as f32).clamp(LIMIT_LOWER, LIMIT_UPPER) - OFFSET) / SCALE) as u8
        }
    }

    pub mod pressure4 {
        use crate::PDU_NOT_AVAILABLE;

        const SCALE: f32 = 1.0 / 128.0;
        const OFFSET: f32 = -250.0;
        const LIMIT_LOWER: f32 = -250.0;
        const LIMIT_UPPER: f32 = 251.99;

        pub fn dec(value: [u8; 2]) -> Option<i16> {
            if value != [PDU_NOT_AVAILABLE, PDU_NOT_AVAILABLE] {
                Some(
                    ((i16::from_le_bytes(value) as f32 * SCALE + OFFSET)
                        .clamp(LIMIT_LOWER, LIMIT_UPPER)) as i16,
                )
            } else {
                None
            }
        }

        pub fn enc(value: i16) -> [u8; 2] {
            let value = ((value as f32).clamp(LIMIT_LOWER, LIMIT_UPPER) - OFFSET) / SCALE;

            (value as i16).to_le_bytes()
        }
    }
}

//
// Time/Date
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
// Electronic Engine Controller 1
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
            driver_demand: slots::position_level2::dec(pdu[1]),
            actual_engine: slots::position_level2::dec(pdu[2]),
            rpm: slots::rotational_velocity::dec([pdu[3], pdu[4]]),
            source_addr: crate::decode::spn1483(pdu[5]),
            starter_mode: EngineStarterMode::from_value(pdu[6]),
        }
    }

    pub fn to_pdu(&self) -> [u8; 8] {
        [
            self.engine_torque_mode
                .map_or(PDU_NOT_AVAILABLE, EngineTorqueMode::to_value),
            self.driver_demand
                .map_or(PDU_NOT_AVAILABLE, slots::position_level2::enc),
            self.actual_engine
                .map_or(PDU_NOT_AVAILABLE, slots::position_level2::enc),
            self.rpm
                .map_or([0xff, 0xff], slots::rotational_velocity::enc)[0],
            self.rpm
                .map_or([0xff, 0xff], slots::rotational_velocity::enc)[1],
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
// Torque Speed Control 1
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
            speed: slots::rotational_velocity::dec([pdu[1], pdu[2]]),
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
            self.speed.map_or(PDU_NOT_AVAILABLE, |speed| {
                slots::rotational_velocity::enc(speed)[0]
            }),
            self.speed.map_or(PDU_NOT_AVAILABLE, |speed| {
                slots::rotational_velocity::enc(speed)[1]
            }),
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
// Ambient Conditions
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
            cab_interior_temperature: slots::temperature::dec([pdu[1], pdu[2]]),
            ambient_air_temperature: slots::temperature::dec([pdu[3], pdu[4]]),
            air_inlet_temperature: if pdu[5] != PDU_NOT_AVAILABLE {
                Some((pdu[5] as i8 - 40).clamp(-40, 127))
            } else {
                None
            },
            road_surface_temperature: slots::temperature::dec([pdu[6], pdu[7]]),
        }
    }

    pub fn to_pdu(&self) -> [u8; 8] {
        [
            if let Some(pressure) = self.barometric_pressure {
                (pressure as f32 * 2.0) as u8
            } else {
                PDU_NOT_AVAILABLE
            },
            self.cab_interior_temperature.map_or(
                [PDU_NOT_AVAILABLE, PDU_NOT_AVAILABLE],
                slots::temperature::enc,
            )[0],
            self.cab_interior_temperature.map_or(
                [PDU_NOT_AVAILABLE, PDU_NOT_AVAILABLE],
                slots::temperature::enc,
            )[1],
            self.ambient_air_temperature.map_or(
                [PDU_NOT_AVAILABLE, PDU_NOT_AVAILABLE],
                slots::temperature::enc,
            )[0],
            self.ambient_air_temperature.map_or(
                [PDU_NOT_AVAILABLE, PDU_NOT_AVAILABLE],
                slots::temperature::enc,
            )[1],
            if let Some(temperature) = self.air_inlet_temperature {
                (temperature + 40) as u8
            } else {
                PDU_NOT_AVAILABLE
            },
            self.road_surface_temperature.map_or(
                [PDU_NOT_AVAILABLE, PDU_NOT_AVAILABLE],
                slots::temperature::enc,
            )[0],
            self.road_surface_temperature.map_or(
                [PDU_NOT_AVAILABLE, PDU_NOT_AVAILABLE],
                slots::temperature::enc,
            )[1],
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
// Vehicle Position
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

//
// Fuel Economy
//

// TODO: Not tested
pub struct FuelEconomyMessage {
    /// Amount of fuel consumed by engine per unit of time.
    pub fuel_rate: Option<f32>,
    /// Current fuel economy at current vehicle velocity.
    pub instantaneous_fuel_economy: Option<f32>,
    /// Average of instantaneous fuel economy for that segment of vehicle operation of interest.
    pub average_fuel_economy: Option<f32>,
    /// The position of the valve used to regulate the supply of a fluid, usually air or fuel/air
    /// mixture, to an engine. 0% represents no supply and 100% is full supply.
    pub throttle_position: Option<u8>,
}

impl FuelEconomyMessage {
    pub fn from_pdu(pdu: &[u8]) -> Self {
        Self {
            fuel_rate: if pdu[0] != PDU_NOT_AVAILABLE {
                Some((u16::from_le_bytes([pdu[0], pdu[1]]) as f32 * 0.05).clamp(0.0, 3212.75))
            } else {
                None
            },
            instantaneous_fuel_economy: if pdu[2] != PDU_NOT_AVAILABLE {
                Some(
                    (u16::from_le_bytes([pdu[2], pdu[3]]) as f32 * (1.0 / 512.0)).clamp(0.0, 125.5),
                )
            } else {
                None
            },
            average_fuel_economy: if pdu[4] != PDU_NOT_AVAILABLE {
                Some(
                    (u16::from_le_bytes([pdu[4], pdu[5]]) as f32 * (1.0 / 512.0)).clamp(0.0, 125.5),
                )
            } else {
                None
            },
            throttle_position: slots::position_level::dec(pdu[6]),
        }
    }

    pub fn to_pdu(&self) -> [u8; 8] {
        [
            if let Some(fuel_rate) = self.fuel_rate {
                ((fuel_rate * 20.0) as u16).to_le_bytes()[0]
            } else {
                PDU_NOT_AVAILABLE
            },
            if let Some(fuel_rate) = self.fuel_rate {
                ((fuel_rate * 20.0) as u16).to_le_bytes()[1]
            } else {
                PDU_NOT_AVAILABLE
            },
            if let Some(instantaneous_fuel_economy) = self.instantaneous_fuel_economy {
                ((instantaneous_fuel_economy * 512.0) as u16).to_le_bytes()[0]
            } else {
                PDU_NOT_AVAILABLE
            },
            if let Some(instantaneous_fuel_economy) = self.instantaneous_fuel_economy {
                ((instantaneous_fuel_economy * 512.0) as u16).to_le_bytes()[1]
            } else {
                PDU_NOT_AVAILABLE
            },
            if let Some(average_fuel_economy) = self.average_fuel_economy {
                ((average_fuel_economy * 512.0) as u16).to_le_bytes()[0]
            } else {
                PDU_NOT_AVAILABLE
            },
            if let Some(average_fuel_economy) = self.average_fuel_economy {
                ((average_fuel_economy * 512.0) as u16).to_le_bytes()[1]
            } else {
                PDU_NOT_AVAILABLE
            },
            self.throttle_position.unwrap_or(PDU_NOT_AVAILABLE),
            PDU_NOT_AVAILABLE,
        ]
    }
}

impl core::fmt::Display for FuelEconomyMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Fuel rate: {} L/h; Instantaneous fuel economy: {} km/kg; Average fuel economy: {} km/kg; Throttle position: {}%",
            self.fuel_rate.unwrap_or(0.0),
            self.instantaneous_fuel_economy.unwrap_or(0.0),
            self.average_fuel_economy.unwrap_or(0.0),
            self.throttle_position.unwrap_or(0)
        )
    }
}

//
// Engine Fluid Level/Pressure 1
//

// TODO: Not tested
pub struct EngineFluidLevelPressure1Message {
    /// Gage pressure of fuel in system as delivered from supply pump to the injection pump.
    pub fuel_delivery_pressure: Option<u8>,
    /// Differential crankcase blow-by pressure as measured through a tube with a venturi.
    pub extended_crankcase_blow_by_pressure: Option<u8>,
    /// Ratio of current volume of engine sump oil to maximum required volume.
    pub engine_oil_level: Option<u8>,
    /// Gage pressure of oil in engine lubrication system as provided by oil pump.
    pub engine_oil_pressure: Option<u8>,
    /// Gage pressure inside engine crankcase.
    pub crankcase_pressure: Option<i16>,
    /// Gage pressure of liquid found in engine cooling system.
    pub coolant_pressure: Option<u8>,
    /// Ratio of volume of liquid found in engine cooling system to total cooling system volume. Typical
    /// monitoring location is in the coolant expansion tank.
    pub coolant_level: Option<u8>,
}

impl EngineFluidLevelPressure1Message {
    pub fn from_pdu(pdu: &[u8]) -> Self {
        Self {
            fuel_delivery_pressure: slots::pressure::dec(pdu[0]),
            extended_crankcase_blow_by_pressure: slots::pressure2::dec(pdu[1]),
            engine_oil_level: slots::position_level::dec(pdu[2]),
            engine_oil_pressure: slots::pressure::dec(pdu[3]),
            crankcase_pressure: slots::pressure4::dec([pdu[4], pdu[5]]),
            coolant_pressure: slots::pressure3::dec(pdu[6]),
            coolant_level: slots::position_level::dec(pdu[7]),
        }
    }

    pub fn to_pdu(&self) -> [u8; 8] {
        [
            self.fuel_delivery_pressure
                .map_or(PDU_NOT_AVAILABLE, slots::pressure::enc),
            self.extended_crankcase_blow_by_pressure
                .map_or(PDU_NOT_AVAILABLE, slots::pressure2::enc),
            self.engine_oil_level
                .map_or(PDU_NOT_AVAILABLE, slots::position_level::enc),
            self.engine_oil_pressure
                .map_or(PDU_NOT_AVAILABLE, slots::pressure::enc),
            self.crankcase_pressure.map_or(
                [PDU_NOT_AVAILABLE, PDU_NOT_AVAILABLE],
                slots::pressure4::enc,
            )[0],
            self.crankcase_pressure.map_or(
                [PDU_NOT_AVAILABLE, PDU_NOT_AVAILABLE],
                slots::pressure4::enc,
            )[1],
            self.coolant_pressure
                .map_or(PDU_NOT_AVAILABLE, slots::pressure3::enc),
            self.coolant_level
                .map_or(PDU_NOT_AVAILABLE, slots::position_level::enc),
        ]
    }
}

impl core::fmt::Display for EngineFluidLevelPressure1Message {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Fuel delivery pressure: {} kPa; Extended crankcase blow-by pressure: {} kPa; Engine oil level: {}%; Engine oil pressure: {} kPa; Crankcase pressure: {} kPa; Coolant pressure: {} kPa; Coolant level: {}%",
            self.fuel_delivery_pressure.unwrap_or(0),
            self.extended_crankcase_blow_by_pressure.unwrap_or(0),
            self.engine_oil_level.unwrap_or(0),
            self.engine_oil_pressure.unwrap_or(0),
            self.crankcase_pressure.unwrap_or(0),
            self.coolant_pressure.unwrap_or(0),
            self.coolant_level.unwrap_or(0)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotational_velocity_test_1() {
        let value = 900;
        let encoded = slots::rotational_velocity::enc(value);
        let decoded = slots::rotational_velocity::dec(encoded);
        assert_eq!(decoded, Some(900));
    }

    #[test]
    fn temperature_test_1() {
        let value = 25;
        let encoded = slots::temperature::enc(value);
        let decoded = slots::temperature::dec(encoded);
        assert_eq!(decoded, Some(25));
    }

    #[test]
    fn position_level_test_1() {
        let value = 50;
        let encoded = slots::position_level::enc(value);
        let decoded = slots::position_level::dec(encoded);
        assert_eq!(decoded, Some(50));
    }

    #[test]
    fn position_level_test_2() {
        let value = 100;
        let encoded = slots::position_level2::enc(value);
        let decoded = slots::position_level2::dec(encoded);
        assert_eq!(decoded, Some(100));
    }

    // #[test]
    // fn pressure_test_1() {
    //     let value = 33;
    //     let encoded = slots::pressure::enc(value);
    //     let decoded = slots::pressure::dec(encoded);
    //     assert_eq!(decoded, Some(33));
    // }

    #[test]
    fn pressure_test_2() {
        let value = 7;
        let encoded = slots::pressure2::enc(value);
        let decoded = slots::pressure2::dec(encoded);
        assert_eq!(decoded, Some(7));
    }

    #[test]
    fn pressure_test_3() {
        let value = 120;
        let encoded = slots::pressure3::enc(value);
        let decoded = slots::pressure3::dec(encoded);
        assert_eq!(decoded, Some(120));
    }

    #[test]
    fn pressure_test_4() {
        let value = -178;
        let encoded = slots::pressure4::enc(value);
        let decoded = slots::pressure4::dec(encoded);
        assert_eq!(decoded, Some(-178));
    }

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

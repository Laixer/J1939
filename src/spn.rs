use crate::{slots, PDU_NOT_AVAILABLE};

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
            slots::position_level2::enc(self.driver_demand),
            slots::position_level2::enc(self.actual_engine),
            slots::rotational_velocity::enc(self.rpm)[0],
            slots::rotational_velocity::enc(self.rpm)[1],
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
// Electronic Engine Controller 2
//

pub struct ElectronicEngineController2Message {
    /// Switch signal which indicates the state of the accelerator pedal 1 low
    /// idle switch.
    pub accelerator_pedal1_low_idle_switch: Option<bool>,
    /// Switch signal which indicates whether the accelerator pedal kickdown
    /// switch is opened or closed.
    pub accelerator_pedal_kickdown_switch: Option<bool>,
    /// Status (active or not active) of the system used to limit maximum vehicle velocity.
    pub road_speed_limit_status: Option<bool>,
    /// The ratio of actual position of the analog engine speed/torque request input device
    /// (such as an accelerator pedal or throttle lever) to the maximum position of the input device.
    pub accelerator_pedal_position1: Option<u8>,
    /// The ratio of actual engine percent torque (indicated) to maximum indicated
    // torque available at the current engine speed, clipped to zero torque during engine braking.
    pub percent_load_at_current_speed: Option<u8>,
    /// The ratio of actual position of the remote analog engine speed/torque
    // request input device (such as an accelerator pedal or throttle lever) to the maximum position of the input device.
    pub remote_accelerator_pedal_position: Option<u8>,
}

impl ElectronicEngineController2Message {
    pub fn from_pdu(pdu: &[u8]) -> Self {
        Self {
            accelerator_pedal1_low_idle_switch: slots::bool_from_value(pdu[0]),
            accelerator_pedal_kickdown_switch: slots::bool_from_value(pdu[0] >> 2),
            road_speed_limit_status: slots::bool_from_value(pdu[0] >> 4),
            accelerator_pedal_position1: slots::position_level2::dec(pdu[1]),
            percent_load_at_current_speed: slots::position_level3::dec(pdu[2]),
            remote_accelerator_pedal_position: slots::position_level::dec(pdu[3]),
        }
    }

    pub fn to_pdu(&self) -> [u8; 8] {
        [
            slots::bool_to_value(self.accelerator_pedal1_low_idle_switch)
                | slots::bool_to_value(self.accelerator_pedal_kickdown_switch) << 2
                | slots::bool_to_value(self.road_speed_limit_status) << 4,
            slots::position_level2::enc(self.accelerator_pedal_position1),
            slots::position_level3::enc(self.percent_load_at_current_speed),
            slots::position_level::enc(self.remote_accelerator_pedal_position),
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
        ]
    }
}

impl core::fmt::Display for ElectronicEngineController2Message {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Accelerator pedal 1 low idle switch: {:?}; Accelerator pedal kickdown switch: {:?}; Road speed limit status: {:?}; Accelerator pedal position 1: {}%; Percent load at current speed: {}%; Remote accelerator pedal position: {}%",
            self.accelerator_pedal1_low_idle_switch,
            self.accelerator_pedal_kickdown_switch,
            self.road_speed_limit_status,
            self.accelerator_pedal_position1.unwrap_or(0),
            self.percent_load_at_current_speed.unwrap_or(0),
            self.remote_accelerator_pedal_position.unwrap_or(0)
        )
    }
}

//
// Electronic Engine Controller 3
//

pub struct ElectronicEngineController3Message {
    /// The calculated torque that indicates the amount of torque required by
    /// the basic engine itself added by the loss torque of accessories.
    pub nominal_friction_percent_torque: Option<u8>,
    /// An indication by the engine of the optimal operating speed of the engine
    /// for the current existing conditions. These conditions may include the torque generated to accommodate powertrain demands from the
    /// operator (via the accelerator pedal), cruise control, road speed limit governors, or ASR. Dynamic commands from functions such as
    /// smoke control or shift control are excluded from this calculation.
    pub engines_desired_operating_speed: Option<u16>,
    /// This byte is utilized in transmission gear
    /// selection routines and indicates the engine's preference of lower versus higher engine speeds should its desired speed not be achievable.
    pub engines_desired_operating_speed_asymmetry_adjustment: Option<u8>,
}

impl ElectronicEngineController3Message {
    pub fn from_pdu(pdu: &[u8]) -> Self {
        Self {
            nominal_friction_percent_torque: slots::position_level2::dec(pdu[0]),
            engines_desired_operating_speed: slots::rotational_velocity::dec([pdu[1], pdu[2]]),
            engines_desired_operating_speed_asymmetry_adjustment: slots::count::dec(pdu[3]),
        }
    }

    pub fn to_pdu(&self) -> [u8; 8] {
        [
            slots::position_level2::enc(self.nominal_friction_percent_torque),
            slots::rotational_velocity::enc(self.engines_desired_operating_speed)[0],
            slots::rotational_velocity::enc(self.engines_desired_operating_speed)[1],
            slots::count::enc(self.engines_desired_operating_speed_asymmetry_adjustment),
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
        ]
    }
}

impl core::fmt::Display for ElectronicEngineController3Message {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Nominal friction percent torque: {}%; Engines desired operating speed: {} RPM; Engines desired operating speed asymmetry adjustment: {}",
            self.nominal_friction_percent_torque.unwrap_or(0),
            self.engines_desired_operating_speed.unwrap_or(0),
            self.engines_desired_operating_speed_asymmetry_adjustment.unwrap_or(0)
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
            slots::rotational_velocity::enc(self.speed)[0],
            slots::rotational_velocity::enc(self.speed)[1],
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
            air_inlet_temperature: slots::temperature2::dec(pdu[5]),
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
            slots::temperature::enc(self.cab_interior_temperature)[0],
            slots::temperature::enc(self.cab_interior_temperature)[1],
            slots::temperature::enc(self.ambient_air_temperature)[0],
            slots::temperature::enc(self.ambient_air_temperature)[1],
            slots::temperature2::enc(self.air_inlet_temperature),
            slots::temperature::enc(self.road_surface_temperature)[0],
            slots::temperature::enc(self.road_surface_temperature)[1],
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
            slots::pressure::enc(self.fuel_delivery_pressure),
            slots::pressure2::enc(self.extended_crankcase_blow_by_pressure),
            slots::position_level::enc(self.engine_oil_level),
            slots::pressure::enc(self.engine_oil_pressure),
            slots::pressure4::enc(self.crankcase_pressure)[0],
            slots::pressure4::enc(self.crankcase_pressure)[1],
            slots::pressure3::enc(self.coolant_pressure),
            slots::position_level::enc(self.coolant_level),
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

//
// Fuel Consumption (Liquid)
//

pub struct FuelConsumptionMessage {
    /// Fuel consumed during all or part of a journey.
    pub trip_fuel: Option<u32>,
    /// Accumulated amount of fuel used during vehicle operation.
    pub total_fuel_used: Option<u32>,
}

impl FuelConsumptionMessage {
    pub fn from_pdu(pdu: &[u8]) -> Self {
        Self {
            trip_fuel: slots::liquid_fuel_usage::dec([pdu[0], pdu[1], pdu[2], pdu[3]]),
            total_fuel_used: slots::liquid_fuel_usage::dec([pdu[4], pdu[5], pdu[6], pdu[7]]),
        }
    }

    pub fn to_pdu(&self) -> [u8; 8] {
        [
            slots::liquid_fuel_usage::enc(self.trip_fuel)[0],
            slots::liquid_fuel_usage::enc(self.trip_fuel)[1],
            slots::liquid_fuel_usage::enc(self.trip_fuel)[2],
            slots::liquid_fuel_usage::enc(self.trip_fuel)[3],
            slots::liquid_fuel_usage::enc(self.total_fuel_used)[0],
            slots::liquid_fuel_usage::enc(self.total_fuel_used)[1],
            slots::liquid_fuel_usage::enc(self.total_fuel_used)[2],
            slots::liquid_fuel_usage::enc(self.total_fuel_used)[3],
        ]
    }
}

impl core::fmt::Display for FuelConsumptionMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Trip fuel: {} L; Total fuel used: {} L",
            self.trip_fuel.unwrap_or(0),
            self.total_fuel_used.unwrap_or(0)
        )
    }
}

//
// Vehicle Distance
//

pub struct VehicleDistanceMessage {
    /// Distance traveled during all or part of a journey.
    pub trip_distance: Option<u32>,
    /// Accumulated distance traveled by vehicle during its operation.
    pub total_vehicle_distance: Option<u32>,
}

impl VehicleDistanceMessage {
    pub fn from_pdu(pdu: &[u8]) -> Self {
        Self {
            trip_distance: slots::distance::dec([pdu[0], pdu[1], pdu[2], pdu[3]]),
            total_vehicle_distance: slots::distance::dec([pdu[4], pdu[5], pdu[6], pdu[7]]),
        }
    }

    pub fn to_pdu(&self) -> [u8; 8] {
        [
            slots::distance::enc(self.trip_distance)[0],
            slots::distance::enc(self.trip_distance)[1],
            slots::distance::enc(self.trip_distance)[2],
            slots::distance::enc(self.trip_distance)[3],
            slots::distance::enc(self.total_vehicle_distance)[0],
            slots::distance::enc(self.total_vehicle_distance)[1],
            slots::distance::enc(self.total_vehicle_distance)[2],
            slots::distance::enc(self.total_vehicle_distance)[3],
        ]
    }
}

impl core::fmt::Display for VehicleDistanceMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Trip distance: {} km; Total vehicle distance: {} km",
            self.trip_distance.unwrap_or(0),
            self.total_vehicle_distance.unwrap_or(0)
        )
    }
}

//
// ECU History
//

pub struct ECUHistoryMessage {
    /// Total distance accumulated over the life of the ECU. When the ECU is replaced this value
    /// shall be reset.
    pub total_ecu_distance: Option<u32>,
    /// Total time accumulated over the life of the ECU, from ignition switch ON to ignition
    /// switch OFF. When the ECU is replaced this value shall be reset.
    pub total_ecu_run_time: Option<u32>,
}

impl ECUHistoryMessage {
    pub fn from_pdu(pdu: &[u8]) -> Self {
        Self {
            total_ecu_distance: slots::distance::dec([pdu[0], pdu[1], pdu[2], pdu[3]]),
            total_ecu_run_time: slots::time::dec([pdu[4], pdu[5], pdu[6], pdu[7]]),
        }
    }

    pub fn to_pdu(&self) -> [u8; 8] {
        [
            slots::distance::enc(self.total_ecu_distance)[0],
            slots::distance::enc(self.total_ecu_distance)[1],
            slots::distance::enc(self.total_ecu_distance)[2],
            slots::distance::enc(self.total_ecu_distance)[3],
            slots::time::enc(self.total_ecu_run_time)[0],
            slots::time::enc(self.total_ecu_run_time)[1],
            slots::time::enc(self.total_ecu_run_time)[2],
            slots::time::enc(self.total_ecu_run_time)[3],
        ]
    }
}

impl core::fmt::Display for ECUHistoryMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Total ECU distance: {} km; Total ECU run time: {} s",
            self.total_ecu_distance.unwrap_or(0),
            self.total_ecu_run_time.unwrap_or(0)
        )
    }
}

//
// Cab Illumination Message
//

pub struct CabIlluminationMessage {
    /// Commanded backlight brightness level for all cab displays.
    pub illumination_brightness_percent: Option<u8>,
}

impl CabIlluminationMessage {
    pub fn from_pdu(pdu: &[u8]) -> Self {
        Self {
            illumination_brightness_percent: slots::position_level::dec(pdu[0]),
        }
    }

    pub fn to_pdu(&self) -> [u8; 8] {
        [
            slots::position_level::enc(self.illumination_brightness_percent),
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
        ]
    }
}

impl core::fmt::Display for CabIlluminationMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Illumination brightness percent: {}%",
            self.illumination_brightness_percent.unwrap_or(0)
        )
    }
}

//
// Fan Drive
//

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FanDriveState {
    FanOff,
    EngineSystemGeneral,
    ExcessiveEngineAirTemperature,
    ExcessiveEngineOilTemperature,
    ExcessiveEngineCoolantTemperature,
    ExcessiveTransmissionOilTemperature,
    ExcessiveHydraulicOilTemperature,
    DefaultOperation,
    NotDefined,
    ManualControl,
    TransmissionRetarder,
    ACSystem,
    Timer,
    EngineBrake,
    Other,
}

impl FanDriveState {
    pub fn from_value(value: u8) -> Option<Self> {
        if value != 0b1111 {
            let mode = match value & 0b1111 {
                0b0000 => Self::FanOff,
                0b0001 => Self::EngineSystemGeneral,
                0b0010 => Self::ExcessiveEngineAirTemperature,
                0b0011 => Self::ExcessiveEngineOilTemperature,
                0b0100 => Self::ExcessiveEngineCoolantTemperature,
                0b0101 => Self::ExcessiveTransmissionOilTemperature,
                0b0110 => Self::ExcessiveHydraulicOilTemperature,
                0b0111 => Self::DefaultOperation,
                0b1000 => Self::NotDefined,
                0b1001 => Self::ManualControl,
                0b1010 => Self::TransmissionRetarder,
                0b1011 => Self::ACSystem,
                0b1100 => Self::Timer,
                0b1101 => Self::EngineBrake,
                0b1110 => Self::Other,
                _ => unreachable!(),
            };

            Some(mode)
        } else {
            None
        }
    }

    pub fn to_value(mode: Self) -> u8 {
        match mode {
            Self::FanOff => 0b0000,
            Self::EngineSystemGeneral => 0b0001,
            Self::ExcessiveEngineAirTemperature => 0b0010,
            Self::ExcessiveEngineOilTemperature => 0b0011,
            Self::ExcessiveEngineCoolantTemperature => 0b0100,
            Self::ExcessiveTransmissionOilTemperature => 0b0101,
            Self::ExcessiveHydraulicOilTemperature => 0b0110,
            Self::DefaultOperation => 0b0111,
            Self::NotDefined => 0b1000,
            Self::ManualControl => 0b1001,
            Self::TransmissionRetarder => 0b1010,
            Self::ACSystem => 0b1011,
            Self::Timer => 0b1100,
            Self::EngineBrake => 0b1101,
            Self::Other => 0b1110,
        }
    }
}

pub struct FanDriveMessage {
    /// Estimated fan speed as a ratio of the fan drive (current speed) to the fully
    /// engaged fan drive (maximum fan speed). A two state fan (off/on) will use 0% and 100% respectively.
    pub estimated_percent_fan_speed: Option<u8>,
    /// This parameter is used to indicate the current state or mode of operation by the fan drive.
    pub fan_drive_state: Option<FanDriveState>,
    /// The speed of the fan associated with engine coolant system.
    pub fan_speed: Option<u16>,
}

impl FanDriveMessage {
    pub fn from_pdu(pdu: &[u8]) -> Self {
        Self {
            estimated_percent_fan_speed: slots::position_level::dec(pdu[0]),
            fan_drive_state: FanDriveState::from_value(pdu[1]),
            fan_speed: slots::rotational_velocity::dec([pdu[2], pdu[3]]),
        }
    }

    pub fn to_pdu(&self) -> [u8; 8] {
        [
            slots::position_level::enc(self.estimated_percent_fan_speed),
            FanDriveState::to_value(self.fan_drive_state.unwrap_or(FanDriveState::FanOff)),
            slots::rotational_velocity::enc(self.fan_speed)[0],
            slots::rotational_velocity::enc(self.fan_speed)[1],
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
        ]
    }
}

impl core::fmt::Display for FanDriveMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Estimated percent fan speed: {}%; Fan drive state: {:?}; Fan speed: {} RPM",
            self.estimated_percent_fan_speed.unwrap_or(0),
            self.fan_drive_state.unwrap_or(FanDriveState::FanOff),
            self.fan_speed.unwrap_or(0)
        )
    }
}

//
// Shutdown
//

pub struct ShutdownMessage {
    pub idle_shutdown_has_shutdown_engine: Option<bool>,
    pub idle_shutdown_driver_alert_mode: Option<bool>,
    pub idle_shutdown_timer_override: Option<bool>,
    pub idle_shutdown_timer_state: Option<bool>,
    pub idle_shutdown_timer_function: Option<bool>,
    pub ac_high_pressure_fan_switch: Option<bool>,
    pub refrigerant_low_pressure_switch: Option<bool>,
    pub refrigerant_high_pressure_switch: Option<bool>,
    pub wait_to_start_lamp: Option<bool>,
    pub engine_protection_system_has_shutdown_engine: Option<bool>,
    pub engine_protection_system_approaching_shutdown: Option<bool>,
    pub engine_protection_system_timer_override: Option<bool>,
    pub engine_protection_system_timer_state: Option<bool>,
    pub engine_protection_system_configuration: Option<bool>,
}

impl ShutdownMessage {
    pub fn from_pdu(pdu: &[u8]) -> Self {
        Self {
            idle_shutdown_has_shutdown_engine: slots::bool_from_value(pdu[0]),
            idle_shutdown_driver_alert_mode: slots::bool_from_value(pdu[0] >> 2),
            idle_shutdown_timer_override: slots::bool_from_value(pdu[0] >> 4),
            idle_shutdown_timer_state: slots::bool_from_value(pdu[0] >> 6),
            idle_shutdown_timer_function: slots::bool_from_value(pdu[1] >> 6),
            ac_high_pressure_fan_switch: slots::bool_from_value(pdu[2]),
            refrigerant_low_pressure_switch: slots::bool_from_value(pdu[2] >> 2),
            refrigerant_high_pressure_switch: slots::bool_from_value(pdu[2] >> 4),
            wait_to_start_lamp: slots::bool_from_value(pdu[3]),
            engine_protection_system_has_shutdown_engine: slots::bool_from_value(pdu[4]),
            engine_protection_system_approaching_shutdown: slots::bool_from_value(pdu[4] >> 2),
            engine_protection_system_timer_override: slots::bool_from_value(pdu[4] >> 4),
            engine_protection_system_timer_state: slots::bool_from_value(pdu[4] >> 6),
            engine_protection_system_configuration: slots::bool_from_value(pdu[5] >> 6),
        }
    }

    pub fn to_pdu(&self) -> [u8; 8] {
        [
            slots::bool_to_value(self.idle_shutdown_has_shutdown_engine)
                | slots::bool_to_value(self.idle_shutdown_driver_alert_mode) << 2
                | slots::bool_to_value(self.idle_shutdown_timer_override) << 4
                | slots::bool_to_value(self.idle_shutdown_timer_state) << 6,
            slots::bool_to_value(self.idle_shutdown_timer_function) << 6,
            slots::bool_to_value(self.ac_high_pressure_fan_switch)
                | slots::bool_to_value(self.refrigerant_low_pressure_switch) << 2
                | slots::bool_to_value(self.refrigerant_high_pressure_switch) << 4,
            slots::bool_to_value(self.wait_to_start_lamp),
            slots::bool_to_value(self.engine_protection_system_has_shutdown_engine)
                | slots::bool_to_value(self.engine_protection_system_approaching_shutdown) << 2
                | slots::bool_to_value(self.engine_protection_system_timer_override) << 4
                | slots::bool_to_value(self.engine_protection_system_timer_state) << 6,
            slots::bool_to_value(self.engine_protection_system_configuration) << 6,
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
        ]
    }
}

impl core::fmt::Display for ShutdownMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Idle shutdown has shutdown engine: {:?}; Idle shutdown driver alert mode: {:?}; Idle shutdown timer override: {:?}; Idle shutdown timer state: {:?}; Idle shutdown timer function: {:?}; AC high pressure fan switch: {:?}; Refrigerant low pressure switch: {:?}; Refrigerant high pressure switch: {:?}; Wait to start lamp: {:?}; Engine protection system has shutdown engine: {:?}; Engine protection system approaching shutdown: {:?}; Engine protection system timer override: {:?}; Engine protection system timer state: {:?}; Engine protection system configuration: {:?}",
            self.idle_shutdown_has_shutdown_engine,
            self.idle_shutdown_driver_alert_mode,
            self.idle_shutdown_timer_override,
            self.idle_shutdown_timer_state,
            self.idle_shutdown_timer_function,
            self.ac_high_pressure_fan_switch,
            self.refrigerant_low_pressure_switch,
            self.refrigerant_high_pressure_switch,
            self.wait_to_start_lamp,
            self.engine_protection_system_has_shutdown_engine,
            self.engine_protection_system_approaching_shutdown,
            self.engine_protection_system_timer_override,
            self.engine_protection_system_timer_state,
            self.engine_protection_system_configuration
        )
    }
}

//
// Power Takeoff Information
//

pub struct PowerTakeoffInformationMessage {
    /// Temperature of lubricant in device used to transmit engine power to auxiliary equipment.
    pub power_takeoff_oil_temperature: Option<i8>,
    /// Rotational velocity of device used to transmit engine power to auxiliary equipment.
    pub power_takeoff_speed: Option<u16>,
    /// Rotational velocity selected by operator for device used to transmit engine power to
    /// auxiliary equipment.
    pub power_takeoff_set_speed: Option<u16>,
    /// Switch signal which indicates that the PTO toggle switch is in the enabled (ON) position and
    /// therefore it is possible to manage the PTO control function.
    pub pto_enable_switch: Option<bool>,
    /// Switch signal which indicates that the remote
    /// PTO toggle switch is in the enabled (ON) position. If the toggle switch is enabled and other conditions are satisfied then the remote
    /// PTO control feature is activated and the PTO will control at the preprogrammed speed.
    pub remote_pto_preprogrammed_speed_control_switch: Option<bool>,
    /// Switch signal which indicates that the remote PTO toggle
    /// switch is in the enabled (ON) position. If the toggle switch is enabled and other conditions are satisfied then the remote PTO control
    /// feature is activated and the PTO will control at a variable speed.
    pub remote_pto_variable_speed_control_switch: Option<bool>,
    /// Switch signal of the PTO control activator which indicates that the activator is in the position "set".
    pub pto_set_switch: Option<bool>,
    /// Switch signal of the PTO control activator which indicates that the activator is in the position "coast/decelerate".
    pub pto_coast_decelerate_switch: Option<bool>,
    /// Switch signal of the PTO control activator which indicates that the activator is in the position "resume".
    pub pto_resume_switch: Option<bool>,
    /// Switch signal of the PTO control activator which indicates that the activator is in the position "accelerate".
    pub pto_accelerate_switch: Option<bool>,
}

impl PowerTakeoffInformationMessage {
    pub fn from_pdu(pdu: &[u8]) -> Self {
        Self {
            power_takeoff_oil_temperature: slots::temperature2::dec(pdu[0]),
            power_takeoff_speed: slots::rotational_velocity::dec([pdu[1], pdu[2]]),
            power_takeoff_set_speed: slots::rotational_velocity::dec([pdu[3], pdu[4]]),
            pto_enable_switch: slots::bool_from_value(pdu[5]),
            remote_pto_preprogrammed_speed_control_switch: slots::bool_from_value(pdu[5] >> 2),
            remote_pto_variable_speed_control_switch: slots::bool_from_value(pdu[5] >> 4),
            pto_set_switch: slots::bool_from_value(pdu[6]),
            pto_coast_decelerate_switch: slots::bool_from_value(pdu[6] >> 2),
            pto_resume_switch: slots::bool_from_value(pdu[6] >> 4),
            pto_accelerate_switch: slots::bool_from_value(pdu[6] >> 6),
        }
    }

    pub fn to_pdu(&self) -> [u8; 8] {
        [
            slots::temperature2::enc(self.power_takeoff_oil_temperature),
            slots::rotational_velocity::enc(self.power_takeoff_speed)[0],
            slots::rotational_velocity::enc(self.power_takeoff_speed)[1],
            slots::rotational_velocity::enc(self.power_takeoff_set_speed)[0],
            slots::rotational_velocity::enc(self.power_takeoff_set_speed)[1],
            slots::bool_to_value(self.pto_enable_switch)
                | slots::bool_to_value(self.remote_pto_preprogrammed_speed_control_switch) << 2
                | slots::bool_to_value(self.remote_pto_variable_speed_control_switch) << 4,
            slots::bool_to_value(self.pto_set_switch)
                | slots::bool_to_value(self.pto_coast_decelerate_switch) << 2
                | slots::bool_to_value(self.pto_resume_switch) << 4
                | slots::bool_to_value(self.pto_accelerate_switch) << 6,
            PDU_NOT_AVAILABLE,
        ]
    }
}

impl core::fmt::Display for PowerTakeoffInformationMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Power takeoff oil temperature: {}°C; Power takeoff speed: {} RPM; Power takeoff set speed: {} RPM; PTO enable switch: {:?}; Remote PTO preprogrammed speed control switch: {:?}; Remote PTO variable speed control switch: {:?}; PTO set switch: {:?}; PTO coast/decelerate switch: {:?}; PTO resume switch: {:?}; PTO accelerate switch: {:?}",
            self.power_takeoff_oil_temperature.unwrap_or(0),
            self.power_takeoff_speed.unwrap_or(0),
            self.power_takeoff_set_speed.unwrap_or(0),
            self.pto_enable_switch,
            self.remote_pto_preprogrammed_speed_control_switch,
            self.remote_pto_variable_speed_control_switch,
            self.pto_set_switch,
            self.pto_coast_decelerate_switch,
            self.pto_resume_switch,
            self.pto_accelerate_switch
        )
    }
}

//
// Engine Temperature 1
//

pub struct EngineTemperature1Message {
    /// Temperature of liquid found in engine cooling system.
    pub engine_coolant_temperature: Option<i8>,
    /// Temperature of fuel entering injectors.
    pub fuel_temperature: Option<i8>,
    /// Temperature of the engine lubricant.
    pub engine_oil_temperature: Option<i16>,
    /// Temperature of the turbocharger lubricant.
    pub turbo_oil_temperature: Option<i16>,
    /// Temperature of liquid found in the intercooler located after the turbocharger.
    pub engine_intercooler_temperature: Option<i8>,
    /// The current position of the thermostat used to regulate the
    /// temperature of the engine intercooler. A value of 0% represents the thermostat being completely closed and 100% represents the
    /// thermostat being completely open.
    pub engine_intercooler_thermostat_opening: Option<u8>,
}

impl EngineTemperature1Message {
    pub fn from_pdu(pdu: &[u8]) -> Self {
        Self {
            engine_coolant_temperature: slots::temperature2::dec(pdu[0]),
            fuel_temperature: slots::temperature2::dec(pdu[1]),
            engine_oil_temperature: slots::temperature::dec([pdu[2], pdu[3]]),
            turbo_oil_temperature: slots::temperature::dec([pdu[4], pdu[5]]),
            engine_intercooler_temperature: slots::temperature2::dec(pdu[6]),
            engine_intercooler_thermostat_opening: slots::position_level::dec(pdu[7]),
        }
    }

    pub fn to_pdu(&self) -> [u8; 8] {
        [
            slots::temperature2::enc(self.engine_coolant_temperature),
            slots::temperature2::enc(self.fuel_temperature),
            slots::temperature::enc(self.engine_oil_temperature)[0],
            slots::temperature::enc(self.engine_oil_temperature)[1],
            slots::temperature::enc(self.turbo_oil_temperature)[0],
            slots::temperature::enc(self.turbo_oil_temperature)[1],
            slots::temperature2::enc(self.engine_intercooler_temperature),
            slots::position_level::enc(self.engine_intercooler_thermostat_opening),
        ]
    }
}

impl core::fmt::Display for EngineTemperature1Message {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Engine coolant temperature: {}°C; Fuel temperature: {}°C; Engine oil temperature: {}°C; Turbo oil temperature: {}°C; Engine intercooler temperature: {}°C; Engine intercooler thermostat opening: {}%",
            self.engine_coolant_temperature.unwrap_or(0),
            self.fuel_temperature.unwrap_or(0),
            self.engine_oil_temperature.unwrap_or(0),
            self.turbo_oil_temperature.unwrap_or(0),
            self.engine_intercooler_temperature.unwrap_or(0),
            self.engine_intercooler_thermostat_opening.unwrap_or(0)
        )
    }
}

//
// Inlet/Exhaust Conditions 1
//

pub struct InletExhaustConditions1Message {
    /// Exhaust back pressure as a result of particle accumulation on filter media placed in the exhaust stream.
    pub particulate_trap_inlet_pressure: Option<u8>,
    /// Gage pressure of air measured downstream on the compressor discharge side of the turbocharger.
    /// See also SPNs 1127-1130 for alternate range and resolution. If there is one boost pressure to report and this range and resolution is
    /// adequate, this parameter should be used.
    pub boost_pressure: Option<u8>,
    /// Temperature of pre-combustion air found in intake manifold of engine air supply system.
    pub intake_manifold_temperature: Option<i8>,
    /// Absolute air pressure at inlet to intake manifold or air box.
    pub air_inlet_pressure: Option<u8>,
    /// Change in engine air system pressure, measured across the filter, due to the
    /// filter and any accumulation of solid foreign matter on or in the filter. This is the measurement of the first filter in a multiple air filter
    /// system. In a single air filter application, this is the only SPN used. Filter numbering follows the guidelines noted in section, Naming
    /// Convention For Engine Parameters.
    pub air_filter_differential_pressure: Option<u8>,
    /// Temperature of combustion byproducts leaving the engine. See SPNs 2433 and
    /// 2434 for engines with more than one exhause gas temperature measurement.
    pub exhaust_gas_temperature: Option<i16>,
    /// Change in coolant pressure, measured across the filter, due to the filter
    /// and any accumulation of solid or semisolid matter on or in the filter.
    pub coolant_filter_differential_pressure: Option<u8>,
}

impl InletExhaustConditions1Message {
    pub fn from_pdu(pdu: &[u8]) -> Self {
        Self {
            particulate_trap_inlet_pressure: None,
            boost_pressure: slots::pressure3::dec(pdu[1]),
            intake_manifold_temperature: slots::temperature2::dec(pdu[2]),
            air_inlet_pressure: slots::pressure3::dec(pdu[3]),
            air_filter_differential_pressure: slots::pressure2::dec(pdu[4]),
            exhaust_gas_temperature: slots::temperature::dec([pdu[5], pdu[6]]),
            coolant_filter_differential_pressure: None,
        }
    }

    pub fn to_pdu(&self) -> [u8; 8] {
        [
            PDU_NOT_AVAILABLE,
            slots::pressure3::enc(self.boost_pressure),
            slots::temperature2::enc(self.intake_manifold_temperature),
            slots::pressure3::enc(self.air_inlet_pressure),
            slots::pressure2::enc(self.air_filter_differential_pressure),
            slots::temperature::enc(self.exhaust_gas_temperature)[0],
            slots::temperature::enc(self.exhaust_gas_temperature)[1],
            PDU_NOT_AVAILABLE,
        ]
    }
}

impl core::fmt::Display for InletExhaustConditions1Message {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Particulate trap inlet pressure: {} kPa; Boost pressure: {} kPa; Intake manifold temperature: {}°C; Air inlet pressure: {} kPa; Air filter differential pressure: {} kPa; Exhaust gas temperature: {}°C; Coolant filter differential pressure: {} kPa",
            self.particulate_trap_inlet_pressure.unwrap_or(0),
            self.boost_pressure.unwrap_or(0),
            self.intake_manifold_temperature.unwrap_or(0),
            self.air_inlet_pressure.unwrap_or(0),
            self.air_filter_differential_pressure.unwrap_or(0),
            self.exhaust_gas_temperature.unwrap_or(0),
            self.coolant_filter_differential_pressure.unwrap_or(0)
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

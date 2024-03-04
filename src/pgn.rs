use crate::PGN_MAX_LENGTH;

/// Parameter group number.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PGN {
    /// TSC1 - Torque/Speed Control 1.
    TorqueSpeedControl1,
    /// PCM1 - Proprietarily Configurable Message 1.
    ProprietarilyConfigurableMessage1,
    /// PCM2 - Proprietarily Configurable Message 2.
    ProprietarilyConfigurableMessage2,
    /// PCM3 - Proprietarily Configurable Message 3.
    ProprietarilyConfigurableMessage3,
    /// PCM4 - Proprietarily Configurable Message 4.
    ProprietarilyConfigurableMessage4,
    /// PCM5 - Proprietarily Configurable Message 5.
    ProprietarilyConfigurableMessage5,
    /// PCM6 - Proprietarily Configurable Message 6.
    ProprietarilyConfigurableMessage6,
    /// PCM7 - Proprietarily Configurable Message 7.
    ProprietarilyConfigurableMessage7,
    /// PCM8 - Proprietarily Configurable Message 8.
    ProprietarilyConfigurableMessage8,
    /// PCM9 - Proprietarily Configurable Message 9.
    ProprietarilyConfigurableMessage9,
    /// PCM10 - Proprietarily Configurable Message 10.
    ProprietarilyConfigurableMessage10,
    /// PCM11 - Proprietarily Configurable Message 11.
    ProprietarilyConfigurableMessage11,
    /// PCM12 - Proprietarily Configurable Message 12.
    ProprietarilyConfigurableMessage12,
    /// PCM13 - Proprietarily Configurable Message 13.
    ProprietarilyConfigurableMessage13,
    /// PCM14 - Proprietarily Configurable Message 14.
    ProprietarilyConfigurableMessage14,
    /// PCM15 - Proprietarily Configurable Message 15.
    ProprietarilyConfigurableMessage15,
    /// PCM16 - Proprietarily Configurable Message 16.
    ProprietarilyConfigurableMessage16,
    /// XFER - Transfer.
    Transfer,
    /// EEC2 - Electronic Engine Controller 2.
    ElectronicEngineController2,
    /// EEC1 - Electronic Engine Controller 1.
    ElectronicEngineController1,
    /// ETC2 Electronic Transmission Controller 2
    ElectronicTransmissionController2,
    /// TI1 - TANK Information 1.
    TANKInformation1,
    /// TCO1 - Tachoraph.
    Tachoraph,
    /// EH - ECU History.
    ECUHistory,
    /// FD - Fan Drive.
    FanDrive,
    /// EEC4 - Electronic Engine Controller 4.
    ElectronicEngineController4,
    /// SOFT - Software Identification.
    SoftwareIdentification,
    /// IO - Idle Operation.
    IdleOperation,
    /// RQST - Request.
    Request,
    /// RQST2 - Request 2.
    Request2,
    /// TP.DT - Transport Protocol Data Transfer.
    TransportProtocolDataTransfer,
    /// TP.CM - Transport Protocol Connection Management.
    TransportProtocolConnectionManagement,
    /// AC - Address Claimed.
    AddressClaimed,
    /// PropA - Proprietary A.
    ProprietaryA,
    /// EBC1 - Electronic Brake Controller 1.
    ElectronicBrakeController1,
    /// ETC1 - Electronic Transmission Controller 1.
    ElectronicTransmissionController1,
    /// CA - Commanded Address.
    CommandedAddress,
    /// AUXIO - Auxiliary Input/Output Status.
    AuxiliaryInputOutputStatus,
    /// EEC3 - Electronic Engine Controller 3
    ElectronicEngineController3,
    /// VD - Vehicle Distance.
    VehicleDistance,
    /// EC - Engine Configuration.
    EngineConfiguration,
    /// SHUTDOWN - Shutdown.
    Shutdown,
    /// HOURS - Engine Hours, Revolutions.
    EngineHoursRevolutions,
    /// TD - Time / Date.
    TimeDate,
    /// VH - Vehicle Hours.
    VehicleHours,
    /// VDS - Vehicle Direction/Speed.
    VehicleDirectionSpeed,
    /// LFC - Fuel Consumption (Liquid).
    FuelConsumption,
    /// VW - Vehicle Weight.
    VehicleWeight,
    /// ET1 - Engine Temperature 1.
    EngineTemperature1,
    /// EFL/P1 - Engine Fluid Level/Pressure 1.
    EngineFluidLevelPressure1,
    /// PTO - Power Takeoff Information.
    PowerTakeoffInformation,
    /// CCVS - Cruise Control/Vehicle Speed.
    CruiseControlVehicleSpeed,
    /// LFE - Fuel Economy (Liquid).
    FuelEconomy,
    /// VP - Vehicle Position.
    VehiclePosition,
    /// AMB - Ambient Conditions.
    AmbientConditions,
    /// IC1 - Inlet/Exhaust Conditions 1.
    InletExhaustConditions1,
    /// VEP1 - Vehicle Electrical Power 1.
    VehicleElectricalPower1,
    /// TF - Transmission Fluids.
    TransmissionFluids,
    /// AI - Axle Information.
    AxleInformation,
    /// B - Brakes.
    Brakes,
    /// RF - Retarder fluids.
    RetarderFluids,
    /// DD - Dash Display.
    DashDisplay,
    /// A1 - Alternate Fuel 1.
    AlternateFuel1,
    /// AWPP - Auxiliary Water Pump Pressure.
    AuxiliaryWaterPumpPressure,
    /// WFI - Water in Fuel Indicator.
    WaterInFuelIndicator,
    /// ACKM - Acknowledgment Message.
    AcknowledgmentMessage,
    /// CI - Component Identification.
    ComponentIdentification,
    /// VI - Vehicle Identification.
    VehicleIdentification,
    /// PropB - Proprietary B.
    ProprietaryB(u32),
    /// Other PGN.
    Other(u32),
}

impl PGN {
    /// Converts the PGN to a little-endian byte array.
    ///
    /// Returns a byte array of length `PGN_MAX_LENGTH` representing the PGN in little-endian format.
    pub fn to_le_bytes(self) -> [u8; PGN_MAX_LENGTH] {
        let byte_array = u32::to_be_bytes(self.into());

        [byte_array[3], byte_array[2], byte_array[1]]
    }

    /// Creates a PGN from a little-endian byte array.
    ///
    /// Returns the PGN created from the byte array.
    pub fn from_le_bytes(bytes: [u8; PGN_MAX_LENGTH]) -> Self {
        let pgn = u32::from_be_bytes([0x0, bytes[2], bytes[1], bytes[0]]);

        PGN::from(pgn & 0x3ffff)
    }
}

impl From<u32> for PGN {
    fn from(value: u32) -> Self {
        match value & 0x3ffff {
            0 => PGN::TorqueSpeedControl1,
            45_312 => PGN::ProprietarilyConfigurableMessage1,
            45_568 => PGN::ProprietarilyConfigurableMessage2,
            45_824 => PGN::ProprietarilyConfigurableMessage3,
            46_080 => PGN::ProprietarilyConfigurableMessage4,
            46_336 => PGN::ProprietarilyConfigurableMessage5,
            46_592 => PGN::ProprietarilyConfigurableMessage6,
            46_848 => PGN::ProprietarilyConfigurableMessage7,
            47_104 => PGN::ProprietarilyConfigurableMessage8,
            47_360 => PGN::ProprietarilyConfigurableMessage9,
            47_616 => PGN::ProprietarilyConfigurableMessage10,
            47_872 => PGN::ProprietarilyConfigurableMessage11,
            48_128 => PGN::ProprietarilyConfigurableMessage12,
            48_384 => PGN::ProprietarilyConfigurableMessage13,
            48_640 => PGN::ProprietarilyConfigurableMessage14,
            48_896 => PGN::ProprietarilyConfigurableMessage15,
            49_152 => PGN::ProprietarilyConfigurableMessage16,
            51_456 => PGN::Request2,
            51_712 => PGN::Transfer,
            59_392 => PGN::AcknowledgmentMessage,
            59_904 => PGN::Request,
            60_160 => PGN::TransportProtocolDataTransfer,
            60_416 => PGN::TransportProtocolConnectionManagement,
            60_928 => PGN::AddressClaimed,
            61_184 => PGN::ProprietaryA,
            61_441 => PGN::ElectronicBrakeController1,
            61_442 => PGN::ElectronicTransmissionController1,
            61_443 => PGN::ElectronicEngineController2,
            61_444 => PGN::ElectronicEngineController1,
            61_445 => PGN::ElectronicTransmissionController2,
            65_110 => PGN::TANKInformation1,
            65_132 => PGN::Tachoraph,
            65_201 => PGN::ECUHistory,
            65_213 => PGN::FanDrive,
            65_214 => PGN::ElectronicEngineController4,
            65_240 => PGN::CommandedAddress,
            65_241 => PGN::AuxiliaryInputOutputStatus,
            65_242 => PGN::SoftwareIdentification,
            65_244 => PGN::IdleOperation,
            65_247 => PGN::ElectronicEngineController3,
            65_248 => PGN::VehicleDistance,
            65_251 => PGN::EngineConfiguration,
            65_252 => PGN::Shutdown,
            65_253 => PGN::EngineHoursRevolutions,
            65_254 => PGN::TimeDate,
            65_255 => PGN::VehicleHours,
            65_256 => PGN::VehicleDirectionSpeed,
            65_257 => PGN::FuelConsumption,
            65_258 => PGN::VehicleWeight,
            65_259 => PGN::ComponentIdentification,
            65_260 => PGN::VehicleIdentification,
            65_262 => PGN::EngineTemperature1,
            65_263 => PGN::EngineFluidLevelPressure1,
            65_264 => PGN::PowerTakeoffInformation,
            65_265 => PGN::CruiseControlVehicleSpeed,
            65_266 => PGN::FuelEconomy,
            65_267 => PGN::VehiclePosition,
            65_269 => PGN::AmbientConditions,
            65_270 => PGN::InletExhaustConditions1,
            65_271 => PGN::VehicleElectricalPower1,
            65_272 => PGN::TransmissionFluids,
            65_273 => PGN::AxleInformation,
            65_274 => PGN::Brakes,
            65_275 => PGN::RetarderFluids,
            65_276 => PGN::DashDisplay,
            65_277 => PGN::AlternateFuel1,
            65_278 => PGN::AuxiliaryWaterPumpPressure,
            65_279 => PGN::WaterInFuelIndicator,
            65_280..=65_535 => PGN::ProprietaryB(value & 0x3ffff),
            _ => PGN::Other(value & 0x3ffff),
        }
    }
}

impl From<PGN> for u32 {
    fn from(value: PGN) -> Self {
        match value {
            PGN::TorqueSpeedControl1 => 0,
            PGN::ProprietarilyConfigurableMessage1 => 45_312,
            PGN::ProprietarilyConfigurableMessage2 => 45_568,
            PGN::ProprietarilyConfigurableMessage3 => 45_824,
            PGN::ProprietarilyConfigurableMessage4 => 46_080,
            PGN::ProprietarilyConfigurableMessage5 => 46_336,
            PGN::ProprietarilyConfigurableMessage6 => 46_592,
            PGN::ProprietarilyConfigurableMessage7 => 46_848,
            PGN::ProprietarilyConfigurableMessage8 => 47_104,
            PGN::ProprietarilyConfigurableMessage9 => 47_360,
            PGN::ProprietarilyConfigurableMessage10 => 47_616,
            PGN::ProprietarilyConfigurableMessage11 => 47_872,
            PGN::ProprietarilyConfigurableMessage12 => 48_128,
            PGN::ProprietarilyConfigurableMessage13 => 48_384,
            PGN::ProprietarilyConfigurableMessage14 => 48_640,
            PGN::ProprietarilyConfigurableMessage15 => 48_896,
            PGN::ProprietarilyConfigurableMessage16 => 49_152,
            PGN::Request2 => 51_456,
            PGN::Transfer => 51_712,
            PGN::AcknowledgmentMessage => 59_392,
            PGN::Request => 59_904,
            PGN::TransportProtocolDataTransfer => 60_160,
            PGN::TransportProtocolConnectionManagement => 60_416,
            PGN::AddressClaimed => 60_928,
            PGN::ProprietaryA => 61_184,
            PGN::ElectronicBrakeController1 => 61_441,
            PGN::ElectronicTransmissionController1 => 61_442,
            PGN::ElectronicEngineController1 => 61_444,
            PGN::ElectronicEngineController2 => 61_443,
            PGN::ElectronicTransmissionController2 => 61_445,
            PGN::TANKInformation1 => 65_110,
            PGN::Tachoraph => 65_132,
            PGN::ECUHistory => 65_201,
            PGN::FanDrive => 65_213,
            PGN::ElectronicEngineController4 => 65_214,
            PGN::CommandedAddress => 65_240,
            PGN::AuxiliaryInputOutputStatus => 65_241,
            PGN::SoftwareIdentification => 65_242,
            PGN::IdleOperation => 65_244,
            PGN::ElectronicEngineController3 => 65_247,
            PGN::VehicleDistance => 65_248,
            PGN::EngineConfiguration => 65_251,
            PGN::Shutdown => 65_252,
            PGN::EngineHoursRevolutions => 65_253,
            PGN::TimeDate => 65_254,
            PGN::VehicleHours => 65_255,
            PGN::VehicleDirectionSpeed => 65_256,
            PGN::FuelConsumption => 65_257,
            PGN::VehicleWeight => 65_258,
            PGN::ComponentIdentification => 65_259,
            PGN::VehicleIdentification => 65_260,
            PGN::EngineTemperature1 => 65_262,
            PGN::EngineFluidLevelPressure1 => 65_263,
            PGN::PowerTakeoffInformation => 65_264,
            PGN::CruiseControlVehicleSpeed => 65_265,
            PGN::FuelEconomy => 65_266,
            PGN::VehiclePosition => 65_267,
            PGN::AmbientConditions => 65_269,
            PGN::InletExhaustConditions1 => 65_270,
            PGN::VehicleElectricalPower1 => 65_271,
            PGN::TransmissionFluids => 65_272,
            PGN::AxleInformation => 65_273,
            PGN::Brakes => 65_274,
            PGN::RetarderFluids => 65_275,
            PGN::DashDisplay => 65_276,
            PGN::AlternateFuel1 => 65_277,
            PGN::AuxiliaryWaterPumpPressure => 65_278,
            PGN::WaterInFuelIndicator => 65_279,
            PGN::ProprietaryB(value_u32) => value_u32 & 0x3ffff,
            PGN::Other(value_u32) => value_u32 & 0x3ffff,
        }
    }
}

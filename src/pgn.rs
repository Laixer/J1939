/// Parameter group number.
#[derive(PartialEq, Eq, Debug)]
pub enum PGN {
    /// TSC1 - Torque/Speed Control 1.
    TorqueSpeedControl1,
    /// PCM1 - Proprietarily Configurable Message 1.
    ProprietarilyConfigurableMessage1,
    /// PCM2 - Proprietarily Configurable Message 2.
    ProprietarilyConfigurableMessage2,
    /// PCM3 - Proprietarily Configurable Message 3.
    ProprietarilyConfigurableMessage3,
    /// XFER - Transfer.
    Transfer,
    /// EEC1 - Electronic Engine Controller 1.
    ElectronicEngineController1,
    /// EEC2 - Electronic Engine Controller 2.
    ElectronicEngineController2,
    /// SOFT - Software Identification.
    SoftwareIdentification,
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
    /// CA - Commanded Address.
    CommandedAddress,
    /// TD - Time / Date.
    TimeDate,
    /// ET1 - Engine Temperature 1.
    EngineTemperature1,
    /// VEP1 - Vehicle Electrical Power 1.
    VehicleElectricalPower1,
    /// PropB - Proprietary B.
    ProprietaryB(u16),
    /// Other PGN.
    Other(u16),
}

impl From<u16> for PGN {
    fn from(value: u16) -> Self {
        match value {
            0 => PGN::TorqueSpeedControl1,
            45_312 => PGN::ProprietarilyConfigurableMessage1,
            45_568 => PGN::ProprietarilyConfigurableMessage2,
            45_824 => PGN::ProprietarilyConfigurableMessage3,
            51_456 => PGN::Request2,
            51_712 => PGN::Transfer,
            59_904 => PGN::Request,
            60_160 => PGN::TransportProtocolDataTransfer,
            60_416 => PGN::TransportProtocolConnectionManagement,
            60_928 => PGN::AddressClaimed,
            61_184 => PGN::ProprietaryA,
            61_443 => PGN::ElectronicEngineController1,
            61_444 => PGN::ElectronicEngineController2,
            65_240 => PGN::CommandedAddress,
            65_242 => PGN::SoftwareIdentification,
            65_254 => PGN::TimeDate,
            65_262 => PGN::EngineTemperature1,
            65_271 => PGN::VehicleElectricalPower1,
            65_280..=65_535 => PGN::ProprietaryB(value),
            _ => PGN::Other(value),
        }
    }
}

impl From<PGN> for u16 {
    fn from(value: PGN) -> Self {
        match value {
            PGN::TorqueSpeedControl1 => 0,
            PGN::ProprietarilyConfigurableMessage1 => 45_312,
            PGN::ProprietarilyConfigurableMessage2 => 45_568,
            PGN::ProprietarilyConfigurableMessage3 => 45_824,
            PGN::Request2 => 51_456,
            PGN::Transfer => 51_712,
            PGN::Request => 59_904,
            PGN::TransportProtocolDataTransfer => 60_160,
            PGN::TransportProtocolConnectionManagement => 60_416,
            PGN::AddressClaimed => 60_928,
            PGN::ProprietaryA => 61_184,
            PGN::ElectronicEngineController1 => 61_443,
            PGN::ElectronicEngineController2 => 61_444,
            PGN::CommandedAddress => 65_240,
            PGN::SoftwareIdentification => 65_242,
            PGN::TimeDate => 65_254,
            PGN::EngineTemperature1 => 65_262,
            PGN::VehicleElectricalPower1 => 65_271,
            PGN::ProprietaryB(value_u16) => value_u16,
            PGN::Other(value_u16) => value_u16,
        }
    }
}

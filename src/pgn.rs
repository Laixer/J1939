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
    /// EEC1 - Electronic Engine Controller 1.
    ElectronicEngineController1,
    /// EEC2 - Electronic Engine Controller 2.
    ElectronicEngineController2,
    /// SOFT - Software Identification.
    SoftwareIdentification,
    /// RQST - Request Message.
    RequestMessage,
    AddressClaimed,
    /// ET1 - Engine Temperature 1.
    EngineTemperature1,
    /// Other PGN.
    Other(u16),
}

impl From<u16> for PGN {
    fn from(value: u16) -> Self {
        match value {
            898 => PGN::TorqueSpeedControl1,
            45_312 => PGN::ProprietarilyConfigurableMessage1,
            45_568 => PGN::ProprietarilyConfigurableMessage2,
            45_824 => PGN::ProprietarilyConfigurableMessage3,
            61_443 => PGN::ElectronicEngineController1,
            61_444 => PGN::ElectronicEngineController2,
            65_242 => PGN::SoftwareIdentification,
            59_904 => PGN::RequestMessage,
            60_928 => PGN::AddressClaimed,
            65_262 => PGN::EngineTemperature1,
            _ => PGN::Other(value),
        }
    }
}

impl From<PGN> for u16 {
    fn from(value: PGN) -> Self {
        match value {
            PGN::TorqueSpeedControl1 => 898,
            PGN::ProprietarilyConfigurableMessage1 => 45_312,
            PGN::ProprietarilyConfigurableMessage2 => 45_568,
            PGN::ProprietarilyConfigurableMessage3 => 45_824,
            PGN::ElectronicEngineController1 => 61_443,
            PGN::ElectronicEngineController2 => 61_444,
            PGN::SoftwareIdentification => 65_242,
            PGN::RequestMessage => 59_904,
            PGN::AddressClaimed => 60_928,
            PGN::EngineTemperature1 => 65_262,
            PGN::Other(value_u16) => value_u16,
        }
    }
}

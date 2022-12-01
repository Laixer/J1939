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
    /// ACKM - Acknowledgment Message.
    AcknowledgmentMessage,
    /// CI - Component Identification.
    ComponentIdentification,
    /// VI - Vehicle Identification.
    VehicleIdentification,
    /// PropB - Proprietary B.
    ProprietaryB(u16),
    /// Other PGN.
    Other(u16),
}

impl PGN {
    pub fn to_le_bytes(self) -> [u8; 3] {
        let byte_array = u32::to_be_bytes(u16::from(self) as u32);

        [byte_array[3], byte_array[2], byte_array[1]]
    }

    pub fn from_le_bytes(bytes: [u8; 3]) -> Self {
        let pgn = u32::from_be_bytes([0x0, bytes[2], bytes[1], bytes[0]]);
        let pgn: u16 = pgn.try_into().unwrap();

        PGN::from(pgn)
    }
}

impl From<u16> for PGN {
    fn from(value: u16) -> Self {
        match value {
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
            61_443 => PGN::ElectronicEngineController1,
            61_444 => PGN::ElectronicEngineController2,
            65_240 => PGN::CommandedAddress,
            65_242 => PGN::SoftwareIdentification,
            65_254 => PGN::TimeDate,
            65_259 => PGN::ComponentIdentification,
            65_260 => PGN::VehicleIdentification,
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
            PGN::ElectronicEngineController1 => 61_443,
            PGN::ElectronicEngineController2 => 61_444,
            PGN::CommandedAddress => 65_240,
            PGN::SoftwareIdentification => 65_242,
            PGN::TimeDate => 65_254,
            PGN::ComponentIdentification => 65_259,
            PGN::VehicleIdentification => 65_260,
            PGN::EngineTemperature1 => 65_262,
            PGN::VehicleElectricalPower1 => 65_271,
            PGN::ProprietaryB(value_u16) => value_u16,
            PGN::Other(value_u16) => value_u16,
        }
    }
}

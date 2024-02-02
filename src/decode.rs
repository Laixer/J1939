use crate::PDU_NOT_AVAILABLE;

pub fn spn190(value: &[u8; 2]) -> Option<u16> {
    if value != &[PDU_NOT_AVAILABLE, PDU_NOT_AVAILABLE] {
        let rpm = (u16::from_le_bytes(*value) as f32 * 0.125) as u16;
        Some(rpm)
    } else {
        None
    }
}

pub fn spn103(value: &[u8; 2]) -> Option<u16> {
    if value != &[PDU_NOT_AVAILABLE, PDU_NOT_AVAILABLE] {
        Some(u16::from_le_bytes(*value) * 4)
    } else {
        None
    }
}

pub fn spn110(value: u8) -> Option<u8> {
    if value != PDU_NOT_AVAILABLE {
        Some(value - 40)
    } else {
        None
    }
}

pub fn spn174(value: u8) -> Option<u8> {
    if value != PDU_NOT_AVAILABLE {
        Some(value - 40)
    } else {
        None
    }
}

// TODO: This is possibly wrong.
pub fn spn157(value: &[u8; 2]) -> Option<u16> {
    if value != &[PDU_NOT_AVAILABLE, PDU_NOT_AVAILABLE] {
        Some(u16::from_le_bytes(*value))
    } else {
        None
    }
}

#[derive(Debug, PartialEq, Eq)]
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

pub fn spn1675(value: u8) -> Option<EngineStarterMode> {
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

pub fn spn512(value: u8) -> Option<u8> {
    if value != PDU_NOT_AVAILABLE {
        Some(value - 125)
    } else {
        None
    }
}

pub fn spn513(value: u8) -> Option<u8> {
    if value != PDU_NOT_AVAILABLE {
        Some(value - 125)
    } else {
        None
    }
}

pub fn spn514(value: u8) -> Option<u8> {
    if value != PDU_NOT_AVAILABLE {
        Some(value - 125)
    } else {
        None
    }
}

pub fn spn515(value: &[u8; 2]) -> Option<u16> {
    if value != &[PDU_NOT_AVAILABLE, PDU_NOT_AVAILABLE] {
        let speed = (u16::from_le_bytes(*value) as f32 * 0.125) as u16;
        Some(speed)
    } else {
        None
    }
}

pub fn spn519(value: u8) -> Option<u8> {
    if value != PDU_NOT_AVAILABLE {
        Some(value)
    } else {
        None
    }
}

pub fn spn975(value: u8) -> Option<f32> {
    if value != PDU_NOT_AVAILABLE {
        Some(value as f32 * 0.4)
    } else {
        None
    }
}

pub fn spn1127(value: &[u8; 2]) -> Option<u16> {
    if value != &[PDU_NOT_AVAILABLE, PDU_NOT_AVAILABLE] {
        let pressure = (u16::from_le_bytes(*value) as f32 * 0.125) as u16;
        Some(pressure)
    } else {
        None
    }
}

pub fn spn1128(value: &[u8; 2]) -> Option<u16> {
    spn1127(value)
}

pub fn spn1129(value: &[u8; 2]) -> Option<u16> {
    spn1127(value)
}

pub fn spn1130(value: &[u8; 2]) -> Option<u16> {
    spn1127(value)
}

pub fn spn1483(value: u8) -> Option<u8> {
    if value != PDU_NOT_AVAILABLE {
        Some(value)
    } else {
        None
    }
}

#[derive(Debug, PartialEq, Eq)]
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

pub fn spn899(value: u8) -> Option<EngineTorqueMode> {
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

pub fn spn959(value: u8) -> Option<u8> {
    if value != PDU_NOT_AVAILABLE {
        Some((value as f32 * 0.25) as u8)
    } else {
        None
    }
}

pub fn spn960(value: u8) -> Option<u8> {
    if value != PDU_NOT_AVAILABLE {
        Some(value)
    } else {
        None
    }
}

pub fn spn961(value: u8) -> Option<u8> {
    if value != PDU_NOT_AVAILABLE {
        Some(value)
    } else {
        None
    }
}

pub fn spn963(value: u8) -> Option<u8> {
    if value != PDU_NOT_AVAILABLE {
        Some(value)
    } else {
        None
    }
}

pub fn spn962(value: u8) -> Option<u8> {
    if value != PDU_NOT_AVAILABLE {
        Some((value as f32 * 0.25) as u8)
    } else {
        None
    }
}

pub fn spn964(value: u8) -> Option<u16> {
    if value != PDU_NOT_AVAILABLE {
        Some(value as u16 + 1985)
    } else {
        None
    }
}

pub fn spn1601(value: u8) -> Option<u8> {
    if value != PDU_NOT_AVAILABLE {
        Some(value - 125)
    } else {
        None
    }
}

pub fn spn1602(value: u8) -> Option<u8> {
    if value != PDU_NOT_AVAILABLE {
        Some(value - 125)
    } else {
        None
    }
}

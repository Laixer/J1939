pub fn bool_from_value(value: u8) -> Option<bool> {
    match value & 0b11 {
        0b00 => Some(false),
        0b01 => Some(true),
        _ => None,
    }
}

pub fn bool_to_value(value: Option<bool>) -> u8 {
    match value {
        Some(false) => 0b00,
        Some(true) => 0b01,
        None => 0b11,
    }
}

struct Param {
    scale: f32,
    offset: f32,
    limit_lower: f32,
    limit_upper: f32,
}

impl Param {
    #[inline]
    fn dec(&self, v: f32) -> f32 {
        (v * self.scale + self.offset).clamp(self.limit_lower, self.limit_upper)
    }

    #[inline]
    fn enc(&self, v: f32) -> f32 {
        (v.clamp(self.limit_lower, self.limit_upper) - self.offset) / self.scale
    }
}

pub mod source_address {
    const RESOLUTION: super::Param = super::Param {
        scale: 1.0,
        offset: 0.0,
        limit_lower: 0.0,
        limit_upper: 255.0,
    };

    pub fn dec(value: u8) -> Option<u8> {
        if value == crate::PDU_NOT_AVAILABLE {
            return None;
        }

        Some(RESOLUTION.dec(value as f32) as u8)
    }

    pub fn enc(value: Option<u8>) -> u8 {
        value.map_or(crate::PDU_NOT_AVAILABLE, |v| RESOLUTION.enc(v as f32) as u8)
    }
}

pub mod count {
    const RESOLUTION: super::Param = super::Param {
        scale: 1.0,
        offset: 0.0,
        limit_lower: 0.0,
        limit_upper: 250.0,
    };

    pub fn dec(value: u8) -> Option<u8> {
        if value == crate::PDU_NOT_AVAILABLE {
            return None;
        }

        Some(RESOLUTION.dec(value as f32) as u8)
    }

    pub fn enc(value: Option<u8>) -> u8 {
        value.map_or(crate::PDU_NOT_AVAILABLE, |v| RESOLUTION.enc(v as f32) as u8)
    }
}

pub mod rotational_velocity {
    const RESOLUTION: super::Param = super::Param {
        scale: 0.125,
        offset: 0.0,
        limit_lower: 0.0,
        limit_upper: 8031.875,
    };

    pub fn dec(value: [u8; 2]) -> Option<u16> {
        if value == [crate::PDU_NOT_AVAILABLE; 2] {
            return None;
        }

        Some(RESOLUTION.dec(u16::from_le_bytes(value) as f32) as u16)
    }

    pub fn enc(value: Option<u16>) -> [u8; 2] {
        value.map_or([crate::PDU_NOT_AVAILABLE; 2], |v| {
            (RESOLUTION.enc(v as f32) as u16).to_le_bytes()
        })
    }
}

pub mod temperature {
    const RESOLUTION: super::Param = super::Param {
        scale: 0.03125,
        offset: -273.0,
        limit_lower: -273.0,
        limit_upper: 1735.0,
    };

    pub fn dec(value: [u8; 2]) -> Option<i16> {
        if value == [crate::PDU_NOT_AVAILABLE; 2] {
            return None;
        }

        Some(RESOLUTION.dec(i16::from_le_bytes(value) as f32) as i16)
    }

    pub fn enc(value: Option<i16>) -> [u8; 2] {
        value.map_or([crate::PDU_NOT_AVAILABLE; 2], |v| {
            (RESOLUTION.enc(v as f32) as i16).to_le_bytes()
        })
    }
}

pub mod temperature2 {
    const RESOLUTION: super::Param = super::Param {
        scale: 1.0,
        offset: -40.0,
        limit_lower: -40.0,
        limit_upper: 127.5,
    };

    pub fn dec(value: u8) -> Option<i8> {
        if value == crate::PDU_NOT_AVAILABLE {
            return None;
        }

        Some(RESOLUTION.dec(value as f32) as i8)
    }

    pub fn enc(value: Option<i8>) -> u8 {
        value.map_or(crate::PDU_NOT_AVAILABLE, |v| RESOLUTION.enc(v as f32) as u8)
    }
}

pub mod electrical_current {
    const RESOLUTION: super::Param = super::Param {
        scale: 1.0,
        offset: -125.0,
        limit_lower: -125.0,
        limit_upper: 125.0,
    };

    pub fn dec(value: u8) -> Option<i8> {
        if value == crate::PDU_NOT_AVAILABLE {
            return None;
        }

        Some(RESOLUTION.dec(value as f32) as i8)
    }

    pub fn enc(value: Option<i8>) -> u8 {
        value.map_or(crate::PDU_NOT_AVAILABLE, |v| RESOLUTION.enc(v as f32) as u8)
    }
}

pub mod electrical_current2 {
    const RESOLUTION: super::Param = super::Param {
        scale: 1.0,
        offset: 0.0,
        limit_lower: 0.0,
        limit_upper: 250.0,
    };

    pub fn dec(value: u8) -> Option<u8> {
        if value == crate::PDU_NOT_AVAILABLE {
            return None;
        }

        Some(RESOLUTION.dec(value as f32) as u8)
    }

    pub fn enc(value: Option<u8>) -> u8 {
        value.map_or(crate::PDU_NOT_AVAILABLE, |v| RESOLUTION.enc(v as f32) as u8)
    }
}

pub mod electrical_voltage {
    const RESOLUTION: super::Param = super::Param {
        scale: 0.05,
        offset: 0.0,
        limit_lower: 0.0,
        limit_upper: 3212.75,
    };

    pub fn dec(value: [u8; 2]) -> Option<u16> {
        if value == [crate::PDU_NOT_AVAILABLE; 2] {
            return None;
        }

        Some(RESOLUTION.dec(u16::from_le_bytes(value) as f32) as u16)
    }

    pub fn enc(value: Option<u16>) -> [u8; 2] {
        value.map_or([crate::PDU_NOT_AVAILABLE; 2], |v| {
            (RESOLUTION.enc(v as f32) as u16).to_le_bytes()
        })
    }
}

pub mod position_level {
    const RESOLUTION: super::Param = super::Param {
        scale: 0.4,
        offset: 0.0,
        limit_lower: 0.0,
        limit_upper: 100.5,
    };

    pub fn dec(value: u8) -> Option<u8> {
        if value == crate::PDU_NOT_AVAILABLE {
            return None;
        }

        Some(RESOLUTION.dec(value as f32) as u8)
    }

    pub fn enc(value: Option<u8>) -> u8 {
        value.map_or(crate::PDU_NOT_AVAILABLE, |v| RESOLUTION.enc(v as f32) as u8)
    }
}

// TODO: Return i8 ?
pub mod position_level2 {
    const RESOLUTION: super::Param = super::Param {
        scale: 1.0,
        offset: -125.0,
        limit_lower: -125.0,
        limit_upper: 125.5,
    };

    pub fn dec(value: u8) -> Option<u8> {
        if value == crate::PDU_NOT_AVAILABLE {
            return None;
        }

        Some(RESOLUTION.dec(value as f32) as u8)
    }

    pub fn enc(value: Option<u8>) -> u8 {
        value.map_or(crate::PDU_NOT_AVAILABLE, |v| RESOLUTION.enc(v as f32) as u8)
    }
}

pub mod position_level3 {
    const RESOLUTION: super::Param = super::Param {
        scale: 1.0,
        offset: 0.0,
        limit_lower: 0.0,
        limit_upper: 125.0,
    };

    pub fn dec(value: u8) -> Option<u8> {
        if value == crate::PDU_NOT_AVAILABLE {
            return None;
        }

        Some(RESOLUTION.dec(value as f32) as u8)
    }

    pub fn enc(value: Option<u8>) -> u8 {
        value.map_or(crate::PDU_NOT_AVAILABLE, |v| RESOLUTION.enc(v as f32) as u8)
    }
}

// TODO: Upper limit might be wrong
pub mod pressure {
    const RESOLUTION: super::Param = super::Param {
        scale: 4.0,
        offset: 0.0,
        limit_lower: 0.0,
        limit_upper: 1000.5,
    };

    pub fn dec(value: u8) -> Option<u8> {
        if value == crate::PDU_NOT_AVAILABLE {
            return None;
        }

        Some(RESOLUTION.dec(value as f32) as u8)
    }

    pub fn enc(value: Option<u8>) -> u8 {
        value.map_or(crate::PDU_NOT_AVAILABLE, |v| RESOLUTION.enc(v as f32) as u8)
    }
}

pub mod pressure2 {
    const RESOLUTION: super::Param = super::Param {
        scale: 0.05,
        offset: 0.0,
        limit_lower: 0.0,
        limit_upper: 12.5,
    };

    pub fn dec(value: u8) -> Option<u8> {
        if value == crate::PDU_NOT_AVAILABLE {
            return None;
        }

        Some(RESOLUTION.dec(value as f32) as u8)
    }

    pub fn enc(value: Option<u8>) -> u8 {
        value.map_or(crate::PDU_NOT_AVAILABLE, |v| RESOLUTION.enc(v as f32) as u8)
    }
}

// TODO: Upper limit might be wrong
pub mod pressure3 {
    const RESOLUTION: super::Param = super::Param {
        scale: 2.0,
        offset: 0.0,
        limit_lower: 0.0,
        limit_upper: 500.99,
    };

    pub fn dec(value: u8) -> Option<u8> {
        if value == crate::PDU_NOT_AVAILABLE {
            return None;
        }

        Some(RESOLUTION.dec(value as f32) as u8)
    }

    pub fn enc(value: Option<u8>) -> u8 {
        value.map_or(crate::PDU_NOT_AVAILABLE, |v| RESOLUTION.enc(v as f32) as u8)
    }
}

pub mod pressure4 {
    const RESOLUTION: super::Param = super::Param {
        scale: 1.0 / 128.0,
        offset: -250.0,
        limit_lower: -250.0,
        limit_upper: 251.99,
    };

    pub fn dec(value: [u8; 2]) -> Option<i16> {
        if value == [crate::PDU_NOT_AVAILABLE; 2] {
            return None;
        }

        Some(RESOLUTION.dec(i16::from_le_bytes(value) as f32) as i16)
    }

    pub fn enc(value: Option<i16>) -> [u8; 2] {
        value.map_or([crate::PDU_NOT_AVAILABLE; 2], |v| {
            (RESOLUTION.enc(v as f32) as i16).to_le_bytes()
        })
    }
}

pub mod liquid_fuel_usage {
    const RESOLUTION: super::Param = super::Param {
        scale: 0.5,
        offset: 0.0,
        limit_lower: 0.0,
        limit_upper: 2105540607.5,
    };

    pub fn dec(value: [u8; 4]) -> Option<u32> {
        if value == [crate::PDU_NOT_AVAILABLE; 4] {
            return None;
        }

        Some(RESOLUTION.dec(u32::from_le_bytes(value) as f32) as u32)
    }

    pub fn enc(value: Option<u32>) -> [u8; 4] {
        value.map_or([crate::PDU_NOT_AVAILABLE; 4], |v| {
            (RESOLUTION.enc(v as f32) as u32).to_le_bytes()
        })
    }
}

pub mod distance {
    const RESOLUTION: super::Param = super::Param {
        scale: 0.125,
        offset: 0.0,
        limit_lower: 0.0,
        limit_upper: 526385151.9,
    };

    pub fn dec(value: [u8; 4]) -> Option<u32> {
        if value == [crate::PDU_NOT_AVAILABLE; 4] {
            return None;
        }

        Some(RESOLUTION.dec(u32::from_le_bytes(value) as f32) as u32)
    }

    pub fn enc(value: Option<u32>) -> [u8; 4] {
        value.map_or([crate::PDU_NOT_AVAILABLE; 4], |v| {
            (RESOLUTION.enc(v as f32) as u32).to_le_bytes()
        })
    }
}

pub mod time {
    const RESOLUTION: super::Param = super::Param {
        scale: 0.05,
        offset: 0.0,
        limit_lower: 0.0,
        limit_upper: 210554060.75,
    };

    pub fn dec(value: [u8; 4]) -> Option<u32> {
        if value == [crate::PDU_NOT_AVAILABLE; 4] {
            return None;
        }

        Some(RESOLUTION.dec(u32::from_le_bytes(value) as f32) as u32)
    }

    pub fn enc(value: Option<u32>) -> [u8; 4] {
        value.map_or([crate::PDU_NOT_AVAILABLE; 4], |v| {
            (RESOLUTION.enc(v as f32) as u32).to_le_bytes()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_test_1() {
        let value = Some(94);
        let encoded = count::enc(value);
        let decoded = count::dec(encoded);
        assert_eq!(decoded, Some(94));
    }

    #[test]
    fn rotational_velocity_test_1() {
        let value = Some(900);
        let encoded = rotational_velocity::enc(value);
        let decoded = rotational_velocity::dec(encoded);
        assert_eq!(decoded, Some(900));
    }

    #[test]
    fn temperature_test_1() {
        let value = Some(25);
        let encoded = temperature::enc(value);
        let decoded = temperature::dec(encoded);
        assert_eq!(decoded, Some(25));
    }

    #[test]
    fn temperature_test_2() {
        let value = Some(-13);
        let encoded = temperature2::enc(value);
        let decoded = temperature2::dec(encoded);
        assert_eq!(decoded, Some(-13));
    }

    #[test]
    fn position_level_test_1() {
        let value = Some(50);
        let encoded = position_level::enc(value);
        let decoded = position_level::dec(encoded);
        assert_eq!(decoded, Some(50));
    }

    #[test]
    fn position_level_test_2() {
        let value = Some(100);
        let encoded = position_level2::enc(value);
        let decoded = position_level2::dec(encoded);
        assert_eq!(decoded, Some(100));
    }

    // #[test]
    // fn pressure_test_1() {
    //     let value = 33;
    //     let encoded = pressure::enc(value);
    //     let decoded = pressure::dec(encoded);
    //     assert_eq!(decoded, Some(33));
    // }

    #[test]
    fn pressure_test_2() {
        let value = Some(7);
        let encoded = pressure2::enc(value);
        let decoded = pressure2::dec(encoded);
        assert_eq!(decoded, Some(7));
    }

    #[test]
    fn pressure_test_3() {
        let value = Some(120);
        let encoded = pressure3::enc(value);
        let decoded = pressure3::dec(encoded);
        assert_eq!(decoded, Some(120));
    }

    #[test]
    fn pressure_test_4() {
        let value = Some(-178);
        let encoded = pressure4::enc(value);
        let decoded = pressure4::dec(encoded);
        assert_eq!(decoded, Some(-178));
    }

    #[test]
    fn liquid_fuel_usage_test_1() {
        let value = Some(7863247);
        let encoded = liquid_fuel_usage::enc(value);
        let decoded = liquid_fuel_usage::dec(encoded);
        assert_eq!(decoded, Some(7863247));
    }

    #[test]
    fn distance_test_1() {
        let value = Some(123456);
        let encoded = distance::enc(value);
        let decoded = distance::dec(encoded);
        assert_eq!(decoded, Some(123456));
    }

    #[test]
    fn time_test_1() {
        let value = Some(123456);
        let encoded = time::enc(value);
        let decoded = time::dec(encoded);
        assert_eq!(decoded, Some(123456));
    }
}

use crate::PDU_MAX_LENGTH;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Name {
    /// Identity number.
    pub identity_number: u32,
    /// Manufacturer code.
    pub manufacturer_code: u16,
    /// Function instance.
    pub function_instance: u8,
    /// ECU instance.
    pub ecu_instance: u8,
    /// Function.
    pub function: u8,
    /// Vehicle system.
    pub vehicle_system: u8,
    /// Vehicle system instance.
    pub vehicle_system_instance: u8,
    /// Industry group.
    pub industry_group: u8,
    /// Arbitrary address.
    pub arbitrary_address: bool,
}

impl Name {
    pub fn to_bytes(self) -> [u8; PDU_MAX_LENGTH] {
        let mut bytes = [0; PDU_MAX_LENGTH];

        bytes[0] = self.identity_number as u8;
        bytes[1] = (self.identity_number >> 8) as u8;
        bytes[2] = ((self.identity_number >> 16) as u8) | ((self.manufacturer_code << 5) as u8);
        bytes[3] = (self.manufacturer_code >> 3) as u8;
        bytes[4] = (self.function_instance << 3) | self.ecu_instance;
        bytes[5] = self.function;
        bytes[6] = self.vehicle_system;
        bytes[7] = self.vehicle_system_instance
            | self.industry_group << 4
            | ((self.arbitrary_address as u8) << 7);

        bytes
    }

    pub fn from_bytes(bytes: [u8; PDU_MAX_LENGTH]) -> Self {
        let identity_number =
            bytes[0] as u32 | ((bytes[1] as u32) << 8) | (((bytes[2] & 0x1f) as u32) << 16);
        let manufacturer_code = (bytes[2] >> 5) as u16 | ((bytes[3] as u16) << 3);
        let function_instance = bytes[4] >> 3;
        let ecu_instance = bytes[4] & 0x7;
        let function = bytes[5];
        let vehicle_system = bytes[6] & 0x7f;
        let vehicle_system_instance = bytes[7] & 0xf;
        let industry_group = (bytes[7] >> 4) & 0x7;
        let arbitrary_address = bytes[7] >> 7;

        Name {
            identity_number,
            manufacturer_code,
            function_instance,
            ecu_instance,
            function,
            vehicle_system,
            vehicle_system_instance,
            industry_group,
            arbitrary_address: arbitrary_address != 0,
        }
    }
}

#[derive(Default)]
pub struct NameBuilder {
    identity_number: u32,
    manufacturer_code: u16,
    function_instance: u8,
    ecu_instance: u8,
    function: u8,
    vehicle_system: u8,
    vehicle_system_instance: u8,
    industry_group: u8,
    arbitrary_address: bool,
}

impl NameBuilder {
    /// Set the identity number.
    #[inline]
    pub fn identity_number(mut self, identity_number: u32) -> Self {
        self.identity_number = identity_number & 0x1fffff;
        self
    }

    /// Set the manufacturer code.
    #[inline]
    pub fn manufacturer_code(mut self, manufacturer_code: u16) -> Self {
        self.manufacturer_code = manufacturer_code & 0x7ff;
        self
    }

    /// Set the function instance.
    #[inline]
    pub fn function_instance(mut self, function_instance: u8) -> Self {
        self.function_instance = function_instance & 0x1f;
        self
    }

    /// Set the ECU instance.
    #[inline]
    pub fn ecu_instance(mut self, ecu_instance: u8) -> Self {
        self.ecu_instance = ecu_instance & 0x7;
        self
    }

    /// Set the function.
    #[inline]
    pub fn function(mut self, function: u8) -> Self {
        self.function = function;
        self
    }

    /// Set the vehicle system.
    #[inline]
    pub fn vehicle_system(mut self, vehicle_system: u8) -> Self {
        self.vehicle_system = vehicle_system & 0x7f;
        self
    }

    /// Set the vehicle system instance.
    #[inline]
    pub fn vehicle_system_instance(mut self, vehicle_system_instance: u8) -> Self {
        self.vehicle_system_instance = vehicle_system_instance & 0xf;
        self
    }

    /// Set the industry group.
    #[inline]
    pub fn industry_group(mut self, industry_group: u8) -> Self {
        self.industry_group = industry_group & 0x7;
        self
    }

    /// Set the arbitrary address.
    #[inline]
    pub fn arbitrary_address(mut self, arbitrary_address: bool) -> Self {
        self.arbitrary_address = arbitrary_address;
        self
    }

    /// Construct name.
    pub fn build(self) -> Name {
        Name {
            identity_number: self.identity_number,
            manufacturer_code: self.manufacturer_code,
            function_instance: self.function_instance,
            ecu_instance: self.ecu_instance,
            function: self.function,
            vehicle_system: self.vehicle_system,
            vehicle_system_instance: self.vehicle_system_instance,
            industry_group: self.industry_group,
            arbitrary_address: self.arbitrary_address,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_to_name() {
        let name = NameBuilder::default()
            .identity_number(0xB5D15)
            .manufacturer_code(0x623)
            .function_instance(30)
            .ecu_instance(0x7)
            .function(0xE3)
            .vehicle_system(126)
            .vehicle_system_instance(15)
            .industry_group(5)
            .arbitrary_address(true)
            .build();

        let name2 = Name::from_bytes(name.to_bytes());

        assert_eq!(name, name2);
        assert_eq!(name2.identity_number, 0xB5D15);
        assert_eq!(name2.manufacturer_code, 0x623);
        assert_eq!(name2.function_instance, 30);
        assert_eq!(name2.ecu_instance, 0x7);
        assert_eq!(name2.function, 0xE3);
        assert_eq!(name2.vehicle_system, 126);
        assert_eq!(name2.vehicle_system_instance, 15);
        assert_eq!(name2.industry_group, 5);
        assert!(name2.arbitrary_address);
    }

    #[test]
    fn test_to_bytes() {
        let name = Name {
            identity_number: 0xB0309,
            manufacturer_code: 0x122,
            function_instance: 0x2,
            ecu_instance: 0x1,
            function: 0x5,
            vehicle_system: 0x6,
            vehicle_system_instance: 0x5,
            industry_group: 0x0,
            arbitrary_address: true,
        };

        let bytes = name.to_bytes();

        assert_eq!(bytes, [0x09, 0x03, 0x4B, 0x24, 0x11, 0x05, 0x06, 0x85]);
    }

    #[test]
    fn test_from_bytes() {
        let bytes = [0x19, 0xA4, 0x49, 0x24, 0x11, 0x05, 0x06, 0x85];

        let name = Name::from_bytes(bytes);

        assert_eq!(
            name,
            Name {
                identity_number: 0x9A419,
                manufacturer_code: 0x122,
                function_instance: 0x2,
                ecu_instance: 0x1,
                function: 0x5,
                vehicle_system: 0x6,
                vehicle_system_instance: 0x5,
                industry_group: 0,
                arbitrary_address: true,
            }
        );
    }
}

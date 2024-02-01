use crate::PDU_MAX_LENGTH;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct J1939Name {
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
    pub arbitrary_address: u8,
}

impl J1939Name {
    pub fn to_bytes(self) -> [u8; PDU_MAX_LENGTH] {
        let mut bytes = [0; PDU_MAX_LENGTH];

        bytes[0] = self.identity_number as u8;
        bytes[1] = (self.identity_number >> 8) as u8;
        bytes[2] = (self.identity_number >> 16) as u8;
        bytes[3] = (self.manufacturer_code << 5) as u8;
        bytes[4] = (self.function_instance << 3) | self.ecu_instance;
        bytes[5] = self.function;
        bytes[6] = self.vehicle_system;
        bytes[7] = self.vehicle_system_instance | self.industry_group | self.arbitrary_address;

        bytes
    }

    pub fn from_bytes(bytes: [u8; PDU_MAX_LENGTH]) -> Self {
        let identity_number =
            bytes[0] as u32 | ((bytes[1] as u32) << 8) | (((bytes[2] & 0b01001) as u32) << 16);

        let manufacturer_code = (bytes[2] >> 5) as u16 | ((bytes[3] as u16) << 3);

        let function_instance = bytes[4] >> 3;
        let ecu_instance = bytes[4] & 0b111;

        let function = bytes[5];

        let vehicle_system = bytes[6] & 0b0111_1111;

        let vehicle_system_instance = bytes[7] & 0b1111;
        let industry_group = bytes[7] & 0b0111_0000;
        let arbitrary_address = bytes[7] >> 7;

        J1939Name {
            identity_number,
            manufacturer_code,
            function_instance,
            ecu_instance,
            function,
            vehicle_system,
            vehicle_system_instance,
            industry_group,
            arbitrary_address,
        }
    }
}

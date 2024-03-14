use crate::PDU_NOT_AVAILABLE;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LampStatus {
    Off,
    On,
    Error,
}

impl LampStatus {
    pub fn from_value(value: u8) -> Option<Self> {
        match value & 0b11 {
            0b00 => Some(Self::Off),
            0b01 => Some(Self::On),
            0b10 => Some(Self::Error),
            _ => None,
        }
    }

    pub fn to_value(mode: Option<Self>) -> u8 {
        match mode {
            Some(Self::Off) => 0b00,
            Some(Self::On) => 0b01,
            Some(Self::Error) => 0b10,
            None => 0b11,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FlashStatus {
    Slow,
    Fast,
    Reserved,
}

impl FlashStatus {
    pub fn from_value(value: u8) -> Option<Self> {
        match value & 0b11 {
            0b00 => Some(Self::Slow),
            0b01 => Some(Self::Fast),
            0b10 => Some(Self::Reserved),
            _ => None,
        }
    }

    pub fn to_value(mode: Option<Self>) -> u8 {
        match mode {
            Some(Self::Slow) => 0b00,
            Some(Self::Fast) => 0b01,
            Some(Self::Reserved) => 0b10,
            None => 0b11,
        }
    }
}

pub struct Diagnostic1Message {
    pub protect_lamp: Option<LampStatus>,
    pub amber_warning_lamp: Option<LampStatus>,
    pub red_stop_lamp: Option<LampStatus>,
    pub malfunction_indicator_lamp: Option<LampStatus>,
    pub protect_lamp_flash: Option<FlashStatus>,
    pub amber_warning_lamp_flash: Option<FlashStatus>,
    pub red_stop_lamp_flash: Option<FlashStatus>,
    pub malfunction_indicator_lamp_flash: Option<FlashStatus>,
    pub suspect_parameter_number: u32,
    pub failure_mode_identifier: u8,
    pub spn_conversion_method: u8,
    pub occurrence_count: u8,
}

impl Diagnostic1Message {
    pub fn from_pdu(pdu: &[u8]) -> Self {
        Self {
            protect_lamp: LampStatus::from_value(pdu[0]),
            amber_warning_lamp: LampStatus::from_value(pdu[0] >> 2),
            red_stop_lamp: LampStatus::from_value(pdu[0] >> 4),
            malfunction_indicator_lamp: LampStatus::from_value(pdu[0] >> 6),
            protect_lamp_flash: FlashStatus::from_value(pdu[1]),
            amber_warning_lamp_flash: FlashStatus::from_value(pdu[1] >> 2),
            red_stop_lamp_flash: FlashStatus::from_value(pdu[1] >> 4),
            malfunction_indicator_lamp_flash: FlashStatus::from_value(pdu[1] >> 6),
            suspect_parameter_number: u32::from_le_bytes([pdu[2], pdu[3], pdu[4] >> 6, 0]),
            failure_mode_identifier: pdu[4] & 0x1F,
            spn_conversion_method: pdu[5] >> 7,
            occurrence_count: pdu[5] & 0x7F,
        }
    }

    pub fn to_pdu(&self) -> [u8; 8] {
        [
            LampStatus::to_value(self.protect_lamp)
                | LampStatus::to_value(self.amber_warning_lamp) << 2
                | LampStatus::to_value(self.red_stop_lamp) << 4
                | LampStatus::to_value(self.malfunction_indicator_lamp) << 6,
            FlashStatus::to_value(self.protect_lamp_flash)
                | FlashStatus::to_value(self.amber_warning_lamp_flash) << 2
                | FlashStatus::to_value(self.red_stop_lamp_flash) << 4
                | FlashStatus::to_value(self.malfunction_indicator_lamp_flash) << 6,
            (self.suspect_parameter_number & 0xFF) as u8,
            ((self.suspect_parameter_number >> 8) & 0xFF) as u8,
            ((self.suspect_parameter_number >> 16) & 0x3F) as u8
                | (self.failure_mode_identifier & 0x1F),
            (self.spn_conversion_method & 0x01) << 7 | (self.occurrence_count & 0x7F),
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
        ]
    }
}

impl core::fmt::Display for Diagnostic1Message {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "Protect Lamp: {:?}, Amber Warning Lamp: {:?}, Red Stop Lamp: {:?}, Malfunction Indicator Lamp: {:?}, \
            Protect Lamp Flash: {:?}, Amber Warning Lamp Flash: {:?}, Red Stop Lamp Flash: {:?}, Malfunction Indicator Lamp Flash: {:?}, \
            Suspect Parameter Number: {}, Failure Mode Identifier: {}, SPN Conversion Method: {}, Occurrence Count: {}",
            self.protect_lamp,
            self.amber_warning_lamp,
            self.red_stop_lamp,
            self.malfunction_indicator_lamp,
            self.protect_lamp_flash,
            self.amber_warning_lamp_flash,
            self.red_stop_lamp_flash,
            self.malfunction_indicator_lamp_flash,
            self.suspect_parameter_number,
            self.failure_mode_identifier,
            self.spn_conversion_method,
            self.occurrence_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostic_1_message_1() {
        let diagnostic_message =
            Diagnostic1Message::from_pdu(&[0x57, 0xFF, 0x9F, 0x00, 0x03, 0x01]);

        assert_eq!(diagnostic_message.protect_lamp, None);
        assert_eq!(diagnostic_message.amber_warning_lamp, Some(LampStatus::On));
        assert_eq!(diagnostic_message.red_stop_lamp, Some(LampStatus::On));
        assert_eq!(
            diagnostic_message.malfunction_indicator_lamp,
            Some(LampStatus::On)
        );
        assert_eq!(diagnostic_message.protect_lamp_flash, None);
        assert_eq!(diagnostic_message.amber_warning_lamp_flash, None);
        assert_eq!(diagnostic_message.red_stop_lamp_flash, None);
        assert_eq!(diagnostic_message.malfunction_indicator_lamp_flash, None);
        assert_eq!(diagnostic_message.suspect_parameter_number, 159);
        assert_eq!(diagnostic_message.failure_mode_identifier, 3);
        assert_eq!(diagnostic_message.spn_conversion_method, 0);
        assert_eq!(diagnostic_message.occurrence_count, 1);
    }

    #[test]
    fn diagnostic_1_message_2() {
        let diagnostic_message =
            Diagnostic1Message::from_pdu(&[0x57, 0xFF, 0xFB, 0x06, 0x0B, 0x32]);

        assert_eq!(diagnostic_message.protect_lamp, None);
        assert_eq!(diagnostic_message.amber_warning_lamp, Some(LampStatus::On));
        assert_eq!(diagnostic_message.red_stop_lamp, Some(LampStatus::On));
        assert_eq!(
            diagnostic_message.malfunction_indicator_lamp,
            Some(LampStatus::On)
        );
        assert_eq!(diagnostic_message.protect_lamp_flash, None);
        assert_eq!(diagnostic_message.amber_warning_lamp_flash, None);
        assert_eq!(diagnostic_message.red_stop_lamp_flash, None);
        assert_eq!(diagnostic_message.malfunction_indicator_lamp_flash, None);
        assert_eq!(diagnostic_message.suspect_parameter_number, 1787);
        assert_eq!(diagnostic_message.failure_mode_identifier, 11);
        assert_eq!(diagnostic_message.spn_conversion_method, 0);
        assert_eq!(diagnostic_message.occurrence_count, 50);
    }

    #[test]
    fn diagnostic_1_message_3() {
        let diagnostic_message =
            Diagnostic1Message::from_pdu(&[0x40, 0xFF, 0x7F, 0x02, 0x02, 0x00]);

        assert_eq!(diagnostic_message.protect_lamp, Some(LampStatus::Off));
        assert_eq!(diagnostic_message.amber_warning_lamp, Some(LampStatus::Off));
        assert_eq!(diagnostic_message.red_stop_lamp, Some(LampStatus::Off));
        assert_eq!(
            diagnostic_message.malfunction_indicator_lamp,
            Some(LampStatus::On)
        );
        assert_eq!(diagnostic_message.protect_lamp_flash, None);
        assert_eq!(diagnostic_message.amber_warning_lamp_flash, None);
        assert_eq!(diagnostic_message.red_stop_lamp_flash, None);
        assert_eq!(diagnostic_message.malfunction_indicator_lamp_flash, None);
        assert_eq!(diagnostic_message.suspect_parameter_number, 639);
        assert_eq!(diagnostic_message.failure_mode_identifier, 2);
        assert_eq!(diagnostic_message.spn_conversion_method, 0);
        assert_eq!(diagnostic_message.occurrence_count, 0);
    }

    #[test]
    fn diagnostic_1_message_4() {
        let diagnostic_message =
            Diagnostic1Message::from_pdu(&[0x00, 0xFF, 0x00, 0x00, 0x00, 0x00]);

        assert_eq!(diagnostic_message.protect_lamp, Some(LampStatus::Off));
        assert_eq!(diagnostic_message.amber_warning_lamp, Some(LampStatus::Off));
        assert_eq!(diagnostic_message.red_stop_lamp, Some(LampStatus::Off));
        assert_eq!(
            diagnostic_message.malfunction_indicator_lamp,
            Some(LampStatus::Off)
        );
        assert_eq!(diagnostic_message.protect_lamp_flash, None);
        assert_eq!(diagnostic_message.amber_warning_lamp_flash, None);
        assert_eq!(diagnostic_message.red_stop_lamp_flash, None);
        assert_eq!(diagnostic_message.malfunction_indicator_lamp_flash, None);
        assert_eq!(diagnostic_message.suspect_parameter_number, 0);
        assert_eq!(diagnostic_message.failure_mode_identifier, 0);
        assert_eq!(diagnostic_message.spn_conversion_method, 0);
        assert_eq!(diagnostic_message.occurrence_count, 0);
    }

    #[test]
    fn diagnostic_1_message_5() {
        let diagnostic_message_encoded = Diagnostic1Message {
            protect_lamp: Some(LampStatus::Off),
            amber_warning_lamp: Some(LampStatus::Off),
            red_stop_lamp: Some(LampStatus::Off),
            malfunction_indicator_lamp: Some(LampStatus::On),
            protect_lamp_flash: None,
            amber_warning_lamp_flash: None,
            red_stop_lamp_flash: None,
            malfunction_indicator_lamp_flash: None,
            suspect_parameter_number: 639,
            failure_mode_identifier: 2,
            spn_conversion_method: 0,
            occurrence_count: 0,
        }
        .to_pdu();

        assert_eq!(
            diagnostic_message_encoded,
            [0x40, 0xFF, 0x7F, 0x02, 0x02, 0x00, 0xFF, 0xFF]
        );
    }

    #[test]
    fn diagnostic_1_message_6() {
        let diagnostic_message_encoded = Diagnostic1Message {
            protect_lamp: Some(LampStatus::On),
            amber_warning_lamp: Some(LampStatus::Off),
            red_stop_lamp: Some(LampStatus::On),
            malfunction_indicator_lamp: Some(LampStatus::On),
            protect_lamp_flash: Some(FlashStatus::Slow),
            amber_warning_lamp_flash: None,
            red_stop_lamp_flash: Some(FlashStatus::Slow),
            malfunction_indicator_lamp_flash: Some(FlashStatus::Fast),
            suspect_parameter_number: 639,
            failure_mode_identifier: 2,
            spn_conversion_method: 0,
            occurrence_count: 0,
        }
        .to_pdu();

        let diagnostic_message_decoded = Diagnostic1Message::from_pdu(&diagnostic_message_encoded);

        assert_eq!(
            diagnostic_message_decoded.protect_lamp,
            Some(LampStatus::On)
        );
        assert_eq!(
            diagnostic_message_decoded.amber_warning_lamp,
            Some(LampStatus::Off)
        );
        assert_eq!(
            diagnostic_message_decoded.red_stop_lamp,
            Some(LampStatus::On)
        );
        assert_eq!(
            diagnostic_message_decoded.malfunction_indicator_lamp,
            Some(LampStatus::On)
        );

        assert_eq!(
            diagnostic_message_decoded.protect_lamp_flash,
            Some(FlashStatus::Slow)
        );
        assert_eq!(diagnostic_message_decoded.amber_warning_lamp_flash, None);
        assert_eq!(
            diagnostic_message_decoded.red_stop_lamp_flash,
            Some(FlashStatus::Slow)
        );
        assert_eq!(
            diagnostic_message_decoded.malfunction_indicator_lamp_flash,
            Some(FlashStatus::Fast)
        );
    }
}

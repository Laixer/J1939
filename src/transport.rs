use crate::{Frame, FrameBuilder, IdBuilder, PGN};

pub enum ConnectionManagement {
    RequestToSend = 0x10,
    ClearToSend = 0x11,
    EndOfMessageAcknowledgment = 0x13,
    BroadcastAnnounceMessage = 0x20,
    Abort = 0xff,
}

pub enum BroadcastTransportState {
    ConnectionManagement,
    DataTransfer(u8),
}

pub struct BroadcastTransport {
    sa: u8,
    pgn: PGN,
    data: [u8; 1785],
    length: usize,
    state: BroadcastTransportState,
}

impl BroadcastTransport {
    pub fn new(sa: u8, pgn: PGN) -> Self {
        Self {
            sa,
            pgn,
            data: [0xFF; 1785],
            length: 0,
            state: BroadcastTransportState::ConnectionManagement,
        }
    }

    pub fn with_data(mut self, data: &[u8]) -> Self {
        self.data[..data.len()].copy_from_slice(data);
        self.length = data.len();
        self
    }

    pub fn packet_count(&self) -> usize {
        let quotient = self.length / 7;
        let remainder = self.length % 7;

        if remainder > 0 {
            quotient + 1
        } else {
            quotient
        }
    }

    pub fn next_frame(&mut self) -> Frame {
        match self.state {
            BroadcastTransportState::ConnectionManagement => {
                let data_length = (self.length as u16).to_le_bytes();
                let packets = self.packet_count() as u8;
                let byte_array = self.pgn.to_le_bytes();

                let frame = FrameBuilder::new(
                    IdBuilder::from_pgn(PGN::TransportProtocolConnectionManagement)
                        .priority(7)
                        .sa(self.sa)
                        .da(0xff)
                        .build(),
                )
                .copy_from_slice(&[
                    ConnectionManagement::BroadcastAnnounceMessage as u8,
                    data_length[0],
                    data_length[1],
                    packets,
                    0xff,
                    byte_array[0],
                    byte_array[1],
                    byte_array[2],
                ])
                .build();

                self.state = BroadcastTransportState::DataTransfer(0);

                frame
            }
            // TODO: Return error frame if packet is out of bounds.
            BroadcastTransportState::DataTransfer(packet) => {
                let sequence = packet + 1;

                let mut frame_builder = FrameBuilder::new(
                    IdBuilder::from_pgn(PGN::TransportProtocolDataTransfer)
                        .priority(7)
                        .sa(self.sa)
                        .da(0xff)
                        .build(),
                );

                let payload = frame_builder.as_mut();
                payload[0] = sequence;

                let data_chunk = &self.data[packet as usize * 7..(packet as usize + 1) * 7];

                if data_chunk.len() == 7 {
                    payload[1..8].copy_from_slice(data_chunk);
                } else {
                    payload[1..(data_chunk.len() + 1)].copy_from_slice(data_chunk);
                }

                let frame = frame_builder.set_len(8).build();

                self.state = BroadcastTransportState::DataTransfer(packet + 1);

                frame
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_broadcast_transport() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];

        let mut transport = BroadcastTransport::new(0x01, PGN::AddressClaimed).with_data(&data);

        let frame = transport.next_frame();
        assert_eq!(frame.id().as_raw(), 0x1CECFF01);
        assert_eq!(frame.len(), 8);
        assert_eq!(
            frame.as_ref(),
            &[0x20, 0x09, 0x00, 0x02, 0xFF, 0x00, 0xEE, 0x00]
        );

        let frame = transport.next_frame();
        assert_eq!(frame.id().as_raw(), 0x1CEBFF01);
        assert_eq!(frame.len(), 8);
        assert_eq!(
            frame.as_ref(),
            &[0x01, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]
        );

        let frame = transport.next_frame();
        assert_eq!(frame.id().as_raw(), 0x1CEBFF01);
        assert_eq!(frame.len(), 8);
        assert_eq!(
            frame.as_ref(),
            &[0x02, 0x08, 0x09, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
        );
    }
}

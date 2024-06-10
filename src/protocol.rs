use crate::{
    transport::BroadcastTransport, Frame, FrameBuilder, IdBuilder, Name, PDU_NOT_AVAILABLE, PGN,
};

/// Create PGN request frame.
pub fn request(da: u8, sa: u8, pgn: PGN) -> Frame {
    let pgn_bytes = pgn.to_le_bytes();

    let id = IdBuilder::from_pgn(PGN::Request).sa(sa).da(da).build();

    FrameBuilder::new(id)
        .copy_from_slice(&[pgn_bytes[0], pgn_bytes[1], pgn_bytes[2]])
        .build()
}

/// Extract PGN from PDU.
pub fn request_from_pdu(pdu: &[u8]) -> PGN {
    PGN::from_le_bytes([pdu[0], pdu[1], pdu[2]])
}

/// Create address claimed frame.
pub fn address_claimed(sa: u8, name: &Name) -> Frame {
    let id = IdBuilder::from_pgn(PGN::AddressClaimed)
        .sa(sa)
        .da(PDU_NOT_AVAILABLE)
        .build();

    FrameBuilder::new(id)
        .copy_from_slice(&name.to_bytes()[..])
        .build()
}

/// Create acknowledgment frame.
pub fn acknowledgement(sa: u8, pgn: PGN) -> Frame {
    let pgn_bytes = pgn.to_le_bytes();

    let id = IdBuilder::from_pgn(PGN::AcknowledgmentMessage)
        .sa(sa)
        .da(PDU_NOT_AVAILABLE)
        .build();

    FrameBuilder::new(id)
        .copy_from_slice(&[
            0x01,
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
            PDU_NOT_AVAILABLE,
            pgn_bytes[0],
            pgn_bytes[1],
            pgn_bytes[2],
        ])
        .build()
}

/// Assign address to node.
pub fn commanded_address(sa: u8, name: &Name, address: u8) -> [Frame; 3] {
    let name_bytes = name.to_bytes();

    let data = [
        name_bytes[0],
        name_bytes[1],
        name_bytes[2],
        name_bytes[3],
        name_bytes[4],
        name_bytes[5],
        name_bytes[6],
        name_bytes[7],
        address,
    ];

    let mut transport = BroadcastTransport::new(sa, PGN::CommandedAddress).with_data(&data);

    let frame0 = transport.next_frame();
    let frame1 = transport.next_frame();
    let frame2 = transport.next_frame();

    [frame0, frame1, frame2]
}

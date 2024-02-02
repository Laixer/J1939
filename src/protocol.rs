use crate::{Frame, FrameBuilder, IdBuilder, Name, PGN};

/// Create PGN request frame.
pub fn request(da: u8, pgn: PGN) -> Frame {
    let pgn_bytes = pgn.to_le_bytes();

    let id = IdBuilder::from_pgn(PGN::Request).da(da).build();

    FrameBuilder::new(id)
        .copy_from_slice(&[pgn_bytes[0], pgn_bytes[1], pgn_bytes[2]])
        .build()
}

/// Create address claimed frame.
pub fn address_claimed(sa: u8, name: Name) -> Frame {
    let id = IdBuilder::from_pgn(PGN::AddressClaimed)
        .sa(sa)
        .da(0xff)
        .build();

    FrameBuilder::new(id)
        .copy_from_slice(&name.to_bytes()[..])
        .build()
}

/// Create acknowledgment frame.
pub fn acknowledgement(sa: u8, pgn: PGN) -> Frame {
    let pgn_bytes = pgn.to_le_bytes();

    let id = IdBuilder::from_pgn(PGN::AcknowledgmentMessage)
        .da(0xff)
        .sa(sa)
        .build();

    FrameBuilder::new(id)
        .copy_from_slice(&[
            0x01,
            0xff,
            0xff,
            0xff,
            0xff,
            pgn_bytes[0],
            pgn_bytes[1],
            pgn_bytes[2],
        ])
        .build()
}

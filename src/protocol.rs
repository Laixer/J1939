use crate::{Frame, FrameBuilder, IdBuilder, Name, PGN};

/// Create PGN request frame.
pub fn request(da: u8, pgn: PGN) -> Frame {
    let byte_array = pgn.to_le_bytes();

    let id = IdBuilder::from_pgn(PGN::Request).da(da).build();

    FrameBuilder::new(id)
        .copy_from_slice(&[byte_array[0], byte_array[1], byte_array[2]])
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

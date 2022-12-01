use crate::{FrameBuilder, IdBuilder, PGN};

/// Request a PGN message.
pub fn request(da: u8, pgn: PGN) -> crate::Frame {
    let byte_array = pgn.to_le_bytes();

    FrameBuilder::new(IdBuilder::from_pgn(PGN::Request).da(da).build())
        .copy_from_slice(&[byte_array[0], byte_array[1], byte_array[2]])
        .build()
}

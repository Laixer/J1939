# J1939

[![Rust](https://github.com/Laixer/J1939/actions/workflows/rust.yml/badge.svg)](https://github.com/Laixer/J1939/actions/workflows/rust.yml)
[![](https://img.shields.io/crates/v/j1939.svg)](https://crates.io/crates/j1939)

A Rust crate for the SAE J1939 automotive protocol.

# Getting Started

Build a J1939 frame with PGN '_address claimed_'.

```rust
let id = j1939::IdBuilder::from_pgn(j1939::PGN::AddressClaimed)
    .priority(3)
    .sa(0x11)
    .da(0xff)
    .build();

let frame = j1939::FrameBuilder::new(id)
    .copy_from_slice(&name.to_bytes()[..])
    .build();
```

**Note** that this just an example and not the actual '_address claimed_' frame as specified by the SAE J1939 standard.

Some common PGNs have defined data structures. For example the Time/Date PGN is fully implemented.

```rust
let timedate = j1939::spn::TimeDate {
    year: 2024,
    month: 4,
    day: 20,
    hour: 10,
    minute: 1,
    second: 58,
};

let id = j1939::IdBuilder::from_pgn(j1939::PGN::TimeDate)
    .sa(0x28)
    .build();

let frame = j1939::FrameBuilder::new(id)
    .copy_from_slice(&timedate.to_pdu())
    .build();
```

**Example**

```sh
$ cargo run --example j1939decode 0x0CB34A29
```

This runs a J1939 ID decoder on the ID '0x0CB34A29'. Each of the J1939 aid properties can be accessed with this crate.

## no_std

This crate supports no_std. By default the crate creates no_std targets which means you can use the J1939 crate on embedded systems that do not support dynamic allocation.

# Contribution

All feedback welcome. Feel free to file bugs, requests for documentation and
any other feedback to the [issue tracker][issues].

# License

J1939 is distributed under the terms of GPL-3.0.

See [LICENSE](LICENSE) for details.

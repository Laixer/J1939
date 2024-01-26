use std::env;

use j1939::Id;

fn usage() {
    println!("Usage: j1939decode <input>");
    println!();
    println!("Options:");
    println!("  <input>     29-bit CAN ID in hexadecimal format (0x18EAFF00)");
}

fn main() {
    let input = env::args().nth(1);

    if input.is_none() {
        usage();
        return;
    }

    let id_str = input.unwrap();
    if !id_str.starts_with("0x") {
        usage();
        return;
    }

    let id_raw = u32::from_str_radix(id_str.trim_start_matches("0x"), 16).expect("Invalid ID");

    let id = Id::new(id_raw);

    println!("ID");
    println!(" Hex: 0x{:X?}", id.as_raw());
    println!(" Dec: {}", id.as_raw());
    println!(" Bin: {:029b}", id.as_raw());
    println!("Priority");
    println!(" Hex: 0x{:X?}", id.priority());
    println!(" Dec: {}", id.priority());
    println!(" Bin: {:03b}", id.priority());
    println!("Data Page (DP): {}", id.dp());
    println!("Parameter Group Number (PGN): {:?}", id.pgn());
    println!(" Hex: 0x{:X?}", id.pgn_raw());
    println!(" Dec: {}", id.pgn_raw());
    println!(" Bin: {:024b}", id.pgn_raw());
    println!("PDU Type: {:?}", id.pf());
    println!("Broadcast: {}", id.is_broadcast());

    if let Some(ge) = id.group_extension() {
        println!("Group Extension (GE)/PDU Specific (PS)");
        println!(" Hex: 0x{:X?}", ge);
        println!(" Dec: {}", ge);
        println!(" Bin: {:08b}", ge);
    }

    if let Some(da) = id.destination_address() {
        println!("Destination Address (DA)");
        println!(" Hex: 0x{:X?}", da);
        println!(" Dec: {}", da);
        println!(" Bin: {:08b}", da);
    }

    println!("Source Address (SA)");
    println!(" Hex: 0x{:X?}", id.sa());
    println!(" Dec: {}", id.sa());
    println!(" Bin: {:08b}", id.sa());
}

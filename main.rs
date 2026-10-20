use std::io::{self, Read, Write};

// The machine has 4096 bytes of RAM. The first line of input is the size of the ROM
// in bytes (loaded at 0x200). Each later line is one hex address.

fn region(addr: u64, rom_len: u64) -> &'static str {
    // TODO: INTERPRETER, FONT, ROM, FREE or INVALID, as described in the exercise.
    let _ = (addr, rom_len);
    ""
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.split('\n');
    let rom_len: u64 = lines.next().unwrap_or("0").trim().parse().unwrap();
    let mut out = String::new();
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let addr = u64::from_str_radix(line, 16).unwrap();
        out.push_str(region(addr, rom_len));
        out.push('\n');
    }
    io::stdout().write_all(out.as_bytes()).unwrap();
}

use std::{fmt::format, io::{self, Read, Write}};

// Return the text for one 16-bit instruction word, e.g. "LD V3, 0A".
fn decode(op: usize) -> String {
    // TODO: split `op` into its nibbles and return the text from the exercise table.
    let a = (op >> 12) & 0xF;
    let b = (op >> 8) & 0xF;
    let c = (op >> 4) & 0xF;
    let d = op & 0xF;

    match (a, b, c, d) {
        (0, 0, 0xE, 0) => "CLS".to_string(),
        (0, 0, 0xE, 0xE) => "RET".to_string(),
        (0, n1, n2, n3) => format!("SYS {:X}{:X}{:X}", n1, n2, n3),
        (1, n1, n2, n3) => format!("JP {:X}{:X}{:X}", n1, n2, n3),
        (2, n1, n2, n3) => format!("CALL {:X}{:X}{:X}", n1, n2, n3),
        (3, x, c, d) => format!("SE V{:X}, {:X}{:X}", x, c, d),
        (4, x, c, d) => format!("SNE V{:X}, {:X}{:X}", x, c, d),
        (5, x, y, 0) => format!("SE V{:X}, V{:X}", x, y),
        (6, x, c, d) => format!("LD V{:X}, {:X}{:X}", x, c ,d),
        (7, x, c, d) => format!("ADD V{:X}, {:X}{:X}", x, c, d),
        (8, x, y, 0) => format!("LD V{:X}, V{:X}", x, y),
        (8, x, y, 1) => format!("OR V{:X}, V{:X}", x, y),
        (8, x, y, 2) => format!("AND V{:X}, V{:X}", x, y),
        (8, x, y, 3) => format!("XOR V{:X}, V{:X}", x, y),
        (8, x, y, 4) => format!("ADD V{:X}, V{:X}", x, y),
        (8, x, y, 5) => format!("SUB V{:X}, V{:X}", x, y),
        (8, x, y, 6) => format!("SHR V{:X}, V{:X}", x, y),
        (8, x, y, 7) => format!("SUBN V{:X}, V{:X}", x, y),
        (8, x, y, 0xE) => format!("SHL V{:X}, V{:X}", x, y),
        (9, x, y, 0) => format!("SNE V{:X}, V{:X}", x, y),
        (0xA, n1, n2, n3) => format!("LD I, {:X}{:X}{:X}",n1, n2, n3),
        (0xB, n1, n2, n3) => format!("JP V0, {:X}{:X}{:X}", n1, n2, n3),
        (0xC, x, c, d) => format!("RND V{:X}, {:X}{:X}", x, c, d),
        (0xD, x, y, n1) => format!("DRW V{:X}, V{:X}, {:X}", x, y, n1),
        (0xE, x, 9, 0xE) => format!("SKP V{:X}", x),
        (0xE, x, 0xA, 1) => format!("SKNP V{:X}", x),
        (0xF, x, 0, 7) => format!("LD V{:X}, DT", x),
        (0xF, x, 0, 0xA) => format!("LD V{:X}, K", x),
        (0xF, x, 1, 5) => format!("LD DT, V{:X}", x),
        (0xF, x, 1, 8) => format!("LD ST, V{:X}", x),
        (0xF, x, 1, 0xE) => format!("ADD I, V{:X}", x),
        (0xF, x, 2, 9) => format!("LD F, V{:X}", x),
        (0xF, x, 3, 3) => format!("LD B, V{:X}", x),
        (0xF, x, 5, 5) => format!("LD [I], V{:X}", x),
        (0xF, x, 6, 5) => format!("LD V{:X}, [I]", x),
        _ => "UNKNOWN".to_string(),
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let digits: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();
    let mut rom: Vec<usize> = Vec::new();
    let mut k = 0;
    while k + 1 < digits.len() {
        let pair: String = digits[k..k + 2].iter().collect();
        rom.push(usize::from_str_radix(&pair, 16).unwrap());
        k += 2;
    }
    let mut out = String::new();
    let mut i = 0;
    while i + 1 < rom.len() {
        let op = (rom[i] << 8) | rom[i + 1];
        out.push_str(&format!("{:03X}: {:04X} {}\n", 0x200 + i, op, decode(op)));
        i += 2;
    }
    if i < rom.len() {
        out.push_str(&format!("{:03X}: {:02X} DB\n", 0x200 + i, rom[i]));
    }
    io::stdout().write_all(out.as_bytes()).unwrap();
}

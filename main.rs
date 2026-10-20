use std::io::{self, Read, Write};

// ---- provided -------------------------------------------------------------
// The 16 hex-digit sprites (4 x 5 pixels, one byte per row, high nibble used).
const FONT: [usize; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

struct Machine {
    mem: Vec<usize>,
    v: [usize; 16],
}

impl Machine {
    fn new() -> Machine {
        let mut m = Machine { mem: vec![0usize; 4096], v: [0; 16] };
        // TODO: copy the 80 font bytes into RAM, starting at address 0x050.
        m.mem[0x050..(0x050 + FONT.len())].copy_from_slice(&FONT);
        m
    }

    fn ld(&mut self, x: usize, kk: usize) {
        // TODO: 6XKK -- VX = KK.
        self.v[x] = kk;
    }

    fn add_imm(&mut self, x: usize, kk: usize) {
        // TODO: 7XKK -- VX = VX + KK, keeping only 8 bits. VF is not touched.
        self.v[x] = (self.v[x] + kk) & 0xFF;
    }

    fn add_reg(&mut self, x: usize, y: usize) {
        // TODO: 8XY4 -- VX = VX + VY (8 bits). VF = 1 on carry, else 0, written last.
        let sum = self.v[x] + self.v[y];

        let carry = if sum > 255 { 1 } else { 0 };

        self.v[x] = sum & 0xFF;
        self.v[15] = carry;
    }
}

fn hex(s: &str) -> usize {
    usize::from_str_radix(s, 16).unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut m = Machine::new();
    let mut out = String::new();
    for line in input.split('\n') {
        let p: Vec<&str> = line.split_whitespace().collect();
        if p.is_empty() {
            continue;
        }
        match p[0] {
            "LD" => m.ld(hex(&p[1][1..]), hex(p[2])),
            "ADD" => {
                if p[2].starts_with('V') {
                    m.add_reg(hex(&p[1][1..]), hex(&p[2][1..]));
                } else {
                    m.add_imm(hex(&p[1][1..]), hex(p[2]));
                }
            }
            "DUMP" => {
                let regs: Vec<String> = m.v.iter().map(|b| format!("{:02X}", b)).collect();
                out.push_str(&regs.join(" "));
                out.push('\n');
            }
            "MEM" => {
                let a = hex(p[1]);
                let n: usize = p[2].parse().unwrap();
                let bytes: Vec<String> = (0..n).map(|k| format!("{:02X}", m.mem[a + k])).collect();
                out.push_str(&bytes.join(" "));
                out.push('\n');
            }
            _ => {}
        }
    }
    io::stdout().write_all(out.as_bytes()).unwrap();
}

use std::io::{self, Read, Write};

// ---- provided -------------------------------------------------------------
// The 16 hex-digit sprites (4 x 5 pixels, one byte per row, high nibble used).
// They are copied into RAM at 0x050 when the machine is created.
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

struct Chip8 {
    mem: Vec<usize>,
    v: [usize; 16],
    i: usize,
    pc: usize,
    stack: [usize; 16],
    sp: usize,
    dt: usize,
    st: usize,
    keys: [bool; 16],
    screen: Vec<u8>,
    fault: &'static str, // empty while the machine is healthy
}

impl Chip8 {
    fn new(rom: &[usize]) -> Chip8 {
        let mut mem = vec![0usize; 4096];
        for k in 0..80 {
            mem[0x50 + k] = FONT[k];
        }
        for k in 0..rom.len() {
            if k < 3584 {
                mem[0x200 + k] = rom[k];
            }
        }
        Chip8 {
            mem: mem,
            v: [0; 16],
            i: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            dt: 0,
            st: 0,
            keys: [false; 16],
            screen: vec![0u8; 64 * 32],
            fault: "",
        }
    }

    fn press(&mut self, k: usize) {
        self.keys[k] = true;
    }

    fn release(&mut self, k: usize) {
        self.keys[k] = false;
    }

    fn tick(&mut self) {}

    fn step(&mut self) {
        if !self.fault.is_empty() {
            return;
        }
        let op = (self.mem[self.pc] << 8) | self.mem[(self.pc + 1) & 0xFFF];
        self.pc = (self.pc + 2) & 0xFFF;
        let h = op >> 12;
        let x = (op >> 8) & 0xF;
        let y = (op >> 4) & 0xF;
        let n = op & 0xF;
        let kk = op & 0xFF;
        let nnn = op & 0xFFF;
        let _ = (y, n, nnn);
        match h {
            0x6 => {
                self.v[x] = kk;
            }
            0x7 => {
                self.v[x] = (self.v[x] + kk) & 0xFF;
            }
            // TODO (this lesson): 3XKK, 4XKK, 5XY0, 9XY0 -- skip the next instruction
            //       by adding 2 more to pc -- and the nine 8XYn ALU operations.
            0x3 => {
                if self.v[x] == kk {
                    self.pc += 2;
                }
            }
            0x4 => {
                if self.v[x] != kk {
                    self.pc += 2;
                }
            }
            0x5 => {
                if n == 0x0 && self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            }
            0x9 => {
                if n == 0x0 && self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            }
            0x8 => match n {
                0x0 => {
                    self.v[x] = self.v[y];
                }
                0x1 => {
                    self.v[x] |= self.v[y];
                }
                0x2 => {
                    self.v[x] &= self.v[y];
                }
                0x3 => {
                    self.v[x] ^= self.v[y];
                }
                0x4 => {
                    let sum = self.v[x] as u16 + self.v[y] as u16;
                    self.v[x] = (sum & 0xFF) as usize;
                    self.v[0xF] = (sum > 0xFF) as usize;
                }
                0x5 => {
                    let vx = self.v[x];
                    let vy = self.v[y];
                    self.v[x] = vx.wrapping_sub(vy) & 0xFF;
                    self.v[0xF] = (vx >= vy) as usize;
                }
                0x6 => {
                    let lsb = self.v[x] & 1;
                    self.v[x] >>= 1;
                    self.v[0xF] = lsb;
                }
                0x7 => {
                    let vx = self.v[x];
                    let vy = self.v[y];
                    self.v[x] = vy.wrapping_sub(vx) & 0xFF;
                    self.v[0xF] = (vy >= vx) as usize;
                }
                0xE => {
                    let msb = (self.v[x] >> 7) & 1;
                    self.v[x] = (self.v[x] << 1) & 0xFF;
                    self.v[0xF] = msb;
                }
                _ => {}
            },
            0x0 => {
                // 0NNN (SYS) and every other 0xxx word are ignored, as in real emulators.
            }
            0xE => {}
            0xF => {}
            _ => {}
        }
    }
}

fn hexv(b: u8) -> Option<usize> {
    match b {
        b'0'..=b'9' => Some((b - b'0') as usize),
        b'A'..=b'F' => Some((b - b'A') as usize + 10),
        b'a'..=b'f' => Some((b - b'a') as usize + 10),
        _ => None,
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.split('\n');
    let first = lines.next().unwrap_or("");
    let mut rom: Vec<usize> = Vec::new();
    let mut hi: Option<usize> = None;
    for b in first.bytes() {
        if let Some(d) = hexv(b) {
            match hi {
                None => hi = Some(d),
                Some(h) => {
                    rom.push(h * 16 + d);
                    hi = None;
                }
            }
        }
    }
    let mut cpu = Chip8::new(&rom);
    let mut out = String::new();
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "RUN" => {
                let n: usize = parts[1].parse().unwrap();
                for _ in 0..n {
                    cpu.step();
                }
            }
            "SHOW" => {
                out.push_str(&format!(
                    "PC={:03X} I={:03X} SP={} DT={:02X} ST={:02X}\n",
                    cpu.pc, cpu.i, cpu.sp, cpu.dt, cpu.st
                ));
                let regs: Vec<String> = cpu.v.iter().map(|b| format!("{:02X}", b)).collect();
                out.push_str(&format!("V={}\n", regs.join(" ")));
                if !cpu.fault.is_empty() {
                    out.push_str(&format!("FAULT={}\n", cpu.fault));
                }
            }
            "MEM" => {
                let a = usize::from_str_radix(parts[1], 16).unwrap();
                let n: usize = parts[2].parse().unwrap();
                let bytes: Vec<String> = (0..n)
                    .map(|k| format!("{:02X}", cpu.mem[(a + k) & 0xFFF]))
                    .collect();
                out.push_str(&bytes.join(" "));
                out.push('\n');
            }
            "SCREEN" => {
                for r in 0..32 {
                    for c in 0..64 {
                        out.push(if cpu.screen[r * 64 + c] == 1 {
                            '#'
                        } else {
                            '.'
                        });
                    }
                    out.push('\n');
                }
            }
            "KEY" => {
                cpu.press(usize::from_str_radix(parts[1], 16).unwrap());
            }
            "UP" => {
                cpu.release(usize::from_str_radix(parts[1], 16).unwrap());
            }
            "TICK" => {
                cpu.tick();
            }
            "WAITFOR" => {}
            "RAND" => {}
            "FRAME" => {}
            _ => {}
        }
    }
    io::stdout().write_all(out.as_bytes()).unwrap();
}

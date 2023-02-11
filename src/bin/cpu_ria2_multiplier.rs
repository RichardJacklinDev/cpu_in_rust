struct CPU {
    registers: [u8; 16],
    program_counter: usize,
    memory: [u8; 0x1000],
}

impl CPU {
    /// Returns a single u16 representation of an opcode read from memory
    fn read_opcode(&self) -> u16 {
        let p = self.program_counter;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        // Uses logical OR to combine two u8 bit values, each cast a u16
        op_byte1 << 8 | op_byte2
    }

    /// Performs the following three steps:
    /// - Reads and decodes the instruction
    /// - Matches the decoded instruction to an opcode
    /// - Dispatches execution of the operation to the specific function
    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.program_counter += 2;

            // Extracts nibbles from bytes
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            match (c, x, y, d) {
                // Short-circuit and terminate execution for empty opcode
                (0, 0, 0, 0) => {
                    return;
                }
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    /// Performs the addition operation and checks for overflow
    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        }

        self.registers[0xF] = 0;
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        program_counter: 0,
        memory: [0; 4096],
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;
    mem[0] = 0x80;
    mem[1] = 0x14;
    mem[2] = 0x80;
    mem[3] = 0x24;
    mem[4] = 0x80;
    mem[5] = 0x34;

    cpu.run();

    assert_eq!(cpu.registers[0], 35);

    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
}
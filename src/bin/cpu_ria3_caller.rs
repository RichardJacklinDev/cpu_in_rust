struct CPU {
    registers: [u8; 16],
    program_counter: usize,
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: usize,
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

    /// Performs the following three steps to run an instruction:
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

            let nnn = opcode & 0x0FFF;
            // let kk = (opcode & 0x00FF) as u8;

            match (c, x, y, d) {
                // Short-circuit and terminate execution for empty opcode
                (0, 0, 0, 0) => {
                    return;
                }
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    /// Performs the following three steps to call a function:
    /// - Stores the current memory lcoation on the stack
    /// - Increments the stack pointer
    /// - Sets the current memory location to the intended memory address
    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow!");
        }

        // Adds the current position in memory to the stack
        stack[sp] = self.program_counter as u16;

        // Increments the stack pointer
        self.stack_pointer += 1;

        // Sets the current memory location to the intended address
        self.program_counter = addr as usize;
    }

    /// Performs the following three steps to return from a function:
    /// - Decrements the stack pointeres
    /// - Retrieves the calling memory address from the stack
    /// - Sets the current memory location to the intended memory address
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        // Decrements the stack pointer
        self.stack_pointer -= 1;

        // Retrieves the calling memory address from the stack
        let call_addr = self.stack[self.stack_pointer];

        // Sets the current memory location to the intended address
        self.program_counter = call_addr as usize;
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
        memory: [0; 4096],
        program_counter: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
    mem[0x000] = 0x21;
    mem[0x001] = 0x00;
    mem[0x002] = 0x21;
    mem[0x003] = 0x00;
    mem[0x004] = 0x00;
    mem[0x005] = 0x00;

    mem[0x100] = 0x80;
    mem[0x101] = 0x14;
    mem[0x102] = 0x80;
    mem[0x103] = 0x14;
    mem[0x104] = 0x00;
    mem[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}

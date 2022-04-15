type OPCODETYPE = u16;

struct CPU {
    // current_operation: OPCODETYPE,
    registers: [u8; 16],
    // position in memory
    program_counter: usize, // usize as it can be used for indexing, spec is u16
    memory: [u8; 0x1000],
}

impl CPU {
    fn read_opcode(&self) -> OPCODETYPE {
        let p = self.program_counter;
        let op_byte1 = self.memory[p] as OPCODETYPE;
        let op_byte2 = self.memory[p + 1] as OPCODETYPE;

        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.program_counter += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => return,
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!(),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}
fn main() {
    let mut cpu = CPU {
        // current_operation: 0,
        registers: [0; 16],
        program_counter: 0,
        memory: [0; 4096],
    };
    // opcode `0x8014`
    // 8 signifies involment of two registers.
    // 0 maps to cpu.registers[0].
    // 1 maps to cpu.registers[1].
    // 4 indicates addition
    // cpu.current_operation = 0x8014;
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;
    mem[0] = 0x80; // register one high nible  opcode: 0x8014 adds register 1 to 0
    mem[1] = 0x14; // register one low nible ...
    mem[2] = 0x80; // register two high nibble opcode: 0x8024 adds register 2 to 0
    mem[3] = 0x24; // register two low nibble ...
    mem[4] = 0x80; // register three high nibble opcode: 0x8034 adds register 3 to 0
    mem[5] = 0x34; // register three low nibble ...

    cpu.run();

    assert_eq!(cpu.registers[0], 35);
    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
}
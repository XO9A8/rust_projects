//CHIP-8 CPU Emulation
struct CPU {
    registers: [u8; 16],          // Array of 16 general-purpose 8-bit registers
    position_in_memory: usize,    // Program counter indicating the current memory position
    memory: [u8; 0x1000],         // 4096-byte memory space
    stack: [u16; 16],             // Stack with 16 entries for storing return addresses
    stack_pointer: usize,         // Points to the current top of the stack
}

impl CPU {
    fn new() -> Self {
        Self {
            registers: [0; 16],              // Initialize all registers to 0
            position_in_memory: 0,           // Start program counter at 0
            memory: [0; 4096],               // Initialize all memory bytes to 0
            stack: [0; 16],                   // Initialize stack entries to 0
            stack_pointer: 0,                 // Initialize stack pointer to 0
        }
    }

    fn read_opcode(&self) -> u16 {
        let opcode_fraction = self.position_in_memory;
        let high_byte = self.memory[opcode_fraction] as u16;       // Read high byte of opcode
        let low_byte = self.memory[opcode_fraction + 1] as u16;    // Read low byte of opcode
        high_byte << 8 | low_byte                                 // Combine bytes to form a 16-bit opcode
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();                  // Fetch the next opcode
            self.position_in_memory += 2;                     // Move program counter to next instruction
            let c = ((opcode & 0xF000) >> 12) as u8;          // Extract the first nibble
            let x = ((opcode & 0x0F00) >> 8) as u8;           // Extract the second nibble
            let y = ((opcode & 0x00F0) >> 4) as u8;           // Extract the third nibble
            let z = (opcode & 0x000F) as u8;                  // Extract the fourth nibble
            let nnn = opcode & 0x0FFF;                        // Extract the lowest 12 bits
            match (c, x, y, z) {
                (0, 0, 0, 0) => {
                    return;                                     // Halt execution
                },
                (0, 0, 0xE, 0xE) => self.ret(),                // Return from subroutine
                (0x2, _, _, _) => self.call(nnn),             // Call subroutine at address nnn
                (0x8, _, _, 0x4) => self.add_xy(x, y),        // Add register y to register x
                _ => todo!("OPCODE: {:04x}", opcode),         // Handle unimplemented opcodes
            }
        }
    }

    fn call(&mut self, add: u16){
        let stack_pointer = self.stack_pointer;
        let stack = &mut self.stack;
        if stack_pointer >= stack.len(){
            todo!("Stack Overflow!");                        // Handle stack overflow
        }
        stack[stack_pointer as usize] = self.position_in_memory as u16; // Push current position to stack
        self.stack_pointer += 1;                             // Increment stack pointer
        self.position_in_memory = add as usize;              // Jump to subroutine address
    }

    fn ret(&mut self){
        if self.stack_pointer == 0{
            panic!("Stack Underflow!");                      // Handle stack underflow
        }
        self.stack_pointer -= 1;                              // Decrement stack pointer
        let addr = self.stack[self.stack_pointer];            // Pop return address from stack
        self.position_in_memory = addr as usize;             // Set program counter to return address
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];               // Get value from register x
        let arg2 = self.registers[y as usize];               // Get value from register y
        let (val, overflow) = arg1.overflowing_add(arg2);    // Add values with overflow check
        println!("ADD: {arg1} + {arg2} = {val}");            // Print addition result
        self.registers[x as usize] = val;                    // Store result in register x
        self.registers[0xF] = if overflow { 1 } else { 0 };  // Set overflow flag in register 0xF
    }
}

fn main() {
    let mut op: CPU = CPU::new();                            // Create a new CPU instance
    op.registers[0] = 5;                                      // Initialize register 0 to 5
    op.registers[1] = 250;                                    // Initialize register 1 to 250
    let mem = &mut op.memory;
    mem[0] = 0x21; mem[1] = 0x00;                            // Opcode 0x2100: Call subroutine at 0x100
    mem[2] = 0x21; mem[3] = 0x00;                            // Opcode 0x2100: Call subroutine at 0x100
    mem[0x100] = 0x80; mem[0x101] = 0x14;                    // Opcode 0x8014: Add register 1 to register 0
    mem[0x102] = 0x80; mem[0x103] = 0x14;                    // Opcode 0x8014: Add register 1 to register 0
    mem[0x104] = 0x00; mem[0x105] = 0xEE;                    // Opcode 0x00EE: Return from subroutine
    op.run();                                                 // Start executing instructions
    println!("{}", op.registers[0]);                          // Print the value of register 0
}
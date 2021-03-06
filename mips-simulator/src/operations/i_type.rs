use crate::instruction::Instruction;
use crate::math::add_unsigned;
use crate::Processor;

impl Processor {
    /// Branch on equal
    pub(crate) fn op_beq(&mut self, instruction: Instruction) {
        let offset = (instruction.immediate() as i32) << 2;
        let address = add_unsigned(self.next_program_counter, offset);
        let s_value = self.registers.get(instruction.s_register());
        let t_value = self.registers.get(instruction.t_register());

        if s_value == t_value {
            self.jump_to(address);
        } else {
            self.advance_program_counter();
        }
    }

    /// Branch on not equal
    pub(crate) fn op_bne(&mut self, instruction: Instruction) {
        let offset = (instruction.immediate() as i32) << 2;
        let address = add_unsigned(self.next_program_counter, offset);
        let s_value = self.registers.get(instruction.s_register());
        let t_value = self.registers.get(instruction.t_register());

        if s_value != t_value {
            self.jump_to(address);
        } else {
            self.advance_program_counter();
        }
    }

    /// Add immediate (with overflow check)
    pub(crate) fn op_addi(&mut self, instruction: Instruction) {
        let a = self.registers.get(instruction.s_register()) as i32;
        let b = instruction.immediate() as i32;
        let value = a
            .checked_add(b)
            .unwrap_or_else(|| panic!("Overflow in addi"));
        self.registers.set(instruction.t_register(), value as u32);
        self.advance_program_counter()
    }

    /// Add immediate (no overflow check)
    pub(crate) fn op_addiu(&mut self, instruction: Instruction) {
        let a = self.registers.get(instruction.s_register()) as i32;
        let b = instruction.immediate() as i32;
        let value = a.wrapping_add(b);
        self.registers.set(instruction.t_register(), value as u32);
        self.advance_program_counter();
    }

    /// Set on less than immediate (signed)
    pub(crate) fn op_slti(&mut self, instruction: Instruction) {
        let s = self.registers.get(instruction.s_register()) as i32;
        let value = if s < instruction.immediate() as i32 {
            1
        } else {
            0
        };
        self.registers.set(instruction.t_register(), value);
        self.advance_program_counter();
    }

    /// Bitwise or immediate
    pub(crate) fn op_ori(&mut self, instruction: Instruction) {
        let a = self.registers.get(instruction.s_register());
        // Don't sign-extend the immediate
        let immediate = instruction.immediate() as u16 as u32;
        self.registers.set(instruction.t_register(), a | immediate);
        self.advance_program_counter();
    }

    /// Load upper immediate
    pub(crate) fn op_lui(&mut self, instruction: Instruction) {
        let value = (instruction.immediate() as u32) << 16;
        self.registers.set(instruction.t_register(), value);
        self.advance_program_counter();
    }

    /// Load byte
    pub(crate) fn op_lb(&mut self, instruction: Instruction) {
        let s_address = self.registers.get(instruction.s_register());
        let value = self
            .memory
            .get(add_unsigned(s_address, instruction.immediate() as i32));
        self.registers
            .set(instruction.t_register(), value as i32 as u32);
        self.advance_program_counter();
    }

    /// Load word
    pub(crate) fn op_lw(&mut self, instruction: Instruction) {
        let s_address = self.registers.get(instruction.s_register());
        let value = self
            .memory
            .get_word(add_unsigned(s_address, instruction.immediate() as i32));
        self.registers.set(instruction.t_register(), value);
        self.advance_program_counter();
    }

    /// Load byte unsigned
    pub(crate) fn op_lbu(&mut self, instruction: Instruction) {
        let s_address = self.registers.get(instruction.s_register());
        let value = self
            .memory
            .get(add_unsigned(s_address, instruction.immediate() as i32));
        self.registers.set(instruction.t_register(), value as u32);
        self.advance_program_counter();
    }

    /// Store byte
    pub(crate) fn op_sb(&mut self, instruction: Instruction) {
        let s_address = self.registers.get(instruction.s_register());
        let address = add_unsigned(s_address, instruction.immediate() as i32);
        let value = self.registers.get(instruction.t_register()) as u8;
        self.memory.set(address, value);
        self.advance_program_counter();
    }

    /// Store word
    #[inline(always)]
    pub(crate) fn op_sw(&mut self, instruction: Instruction) {
        let s_address = self.registers.get(instruction.s_register());
        let address = add_unsigned(s_address, instruction.immediate() as i32);
        let value = self.registers.get(instruction.t_register());
        self.memory.set_word(address, value);
        self.advance_program_counter();
    }
}

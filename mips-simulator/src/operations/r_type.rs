use crate::instruction::Instruction;
use crate::Processor;

impl Processor {
    /// Shift left logical
    pub(crate) fn op_sll(&mut self, instruction: Instruction) {
        if instruction.0 == 0 {
            debug!("noop");
            self.advance_program_counter();
            return;
        }

        debug!(
            "sll ${}, ${}, {}",
            instruction.d_register(),
            instruction.t_register(),
            instruction.shift_amount()
        );
        let value = self.registers.get(instruction.t_register()) << instruction.shift_amount();
        self.registers.set(instruction.d_register(), value);
        self.advance_program_counter()
    }

    /// Jump register
    pub(crate) fn op_jr(&mut self, instruction: Instruction) {
        debug!("jr ${}", instruction.s_register());
        let address = self.registers.get(instruction.s_register());
        self.jump_to(address);
    }

    /// Break (exceptions/debugger)
    pub(crate) fn op_break(&mut self) {
        debug!("break");
        self.advance_program_counter();
        self.running = false;
    }

    /// Add (with overflow)
    pub(crate) fn op_add(&mut self, instruction: Instruction) {
        debug!(
            "add ${}, ${}, ${}",
            instruction.d_register(),
            instruction.s_register(),
            instruction.t_register()
        );
        let a = self.registers.get(instruction.s_register());
        let b = self.registers.get(instruction.t_register());
        self.registers
            .set(instruction.d_register(), a.wrapping_add(b));
        self.advance_program_counter();
    }

    /// Add unsigned (no overflow)
    pub(crate) fn op_addu(&mut self, instruction: Instruction) {
        debug!(
            "addu ${}, ${}, ${}",
            instruction.d_register(),
            instruction.s_register(),
            instruction.t_register()
        );
        let a = self.registers.get(instruction.s_register());
        let b = self.registers.get(instruction.t_register());
        let value = a
            .checked_add(b)
            .unwrap_or_else(|| panic!("Overflow in addu"));
        self.registers.set(instruction.d_register(), value);
        self.advance_program_counter();
    }
}

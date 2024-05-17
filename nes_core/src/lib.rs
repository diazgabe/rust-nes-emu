/// An NES emulator.
struct Emu {
    /// The accumulator register.  
    a: u8,
    /// Index Register X.
    x: u8,
    /// Index Register Y.
    y: u8,

    /// A flag that denotes whether the last operation caused unsigned overflow or underflow.
    /// I.e. This flag is set to true if the last operation caused a carry out from bit 7 of the result or a carry in to bit 0.
    carry: bool,
    /// A flag that denotes whether to follow the rules of Binary Coded Decimal (BCD) arithmetic during addition and subtraction.
    decimal_mode: bool,
    /// A flag that denotes whether to ignore interrupts.
    interrupt_disable: bool,
    // A flag that denotes whether the last operation had bit 7 set to a one.
    negative: bool,
    /// A flag that denotes whether the last arithmetic operation caused signed overflow.
    /// I.e. This flag is set to true if in the last operation two positive numbers summed to a negative number or if two negative numbers summed to a positive number.
    overflow: bool,
    /// A flag that denotes whether the last operation resulted in a zero.
    zero: bool,
}

impl Emu {
    /// Instantiates a new NES emulator.
    fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,

            carry: false,
            decimal_mode: false,
            interrupt_disable: true,
            negative: false,
            overflow: false,
            zero: false,
        }
    }

    /// Parses and calls an instruction.
    ///
    /// # Panics
    ///
    /// Panics if the instruction is not recognized.
    fn execute(&mut self, opcode: u8) {
        match opcode {
            0xEA => return,
            0x18 => self.clc(),
            0xD8 => self.cld(),
            0x58 => self.cli(),
            0xB8 => self.clv(),
            0x38 => self.sec(),
            0xF8 => self.sed(),
            0x78 => self.sei(),
            0xAA => self.tax(),
            0xA8 => self.tay(),
            0x8A => self.txa(),
            0x98 => self.tya(),
            _ => unimplemented!(),
        }
    }

    // Instructions referenced from https://www.nesdev.org/obelisk-6502-guide/reference.html.

    /// ADC - Add with Carry.
    fn adc(&mut self) {
        unimplemented!();
    }

    /// AND - Logical AND.
    fn and(&mut self) {
        unimplemented!();
    }

    /// ASL - Arithmetic Shift Left.
    fn asl(&mut self) {
        unimplemented!();
    }

    /// BCC - Branch if Carry Clear.
    fn bcc(&mut self) {
        unimplemented!();
    }

    /// BCS - Branch if Carry Set.
    fn bcs(&mut self) {
        unimplemented!();
    }

    /// BEQ - Branch if Equal.
    fn beq(&mut self) {
        unimplemented!();
    }

    /// BIT - Bit Test.
    fn bit(&mut self) {
        unimplemented!();
    }

    /// BMI - Branch if Minus.
    fn bmi(&mut self) {
        unimplemented!();
    }

    /// BNE - Branch if Not Equal.
    fn bne(&mut self) {
        unimplemented!();
    }

    /// BPL - Branch if Positive.
    fn bpl(&mut self) {
        unimplemented!();
    }

    /// BRK - Force Interrupt.
    fn brk(&mut self) {
        unimplemented!();
    }

    /// BVC - Branch if Overflow Clear.
    fn bvc(&mut self) {
        unimplemented!();
    }

    /// BVS - Branch if Overflow Set.
    fn bvs(&mut self) {
        unimplemented!();
    }

    /// CLC - Clear Carry Flag.
    fn clc(&mut self) {
        self.carry = false;
    }

    /// CLD - Clear Decimal Mode.
    fn cld(&mut self) {
        self.decimal_mode = false;
    }

    /// CLI - Clear Interrupt Disable.
    fn cli(&mut self) {
        self.interrupt_disable = false;
    }

    /// CLV - Clear Overflow Flag.
    fn clv(&mut self) {
        self.overflow = false;
    }

    /// CMP - Compare.
    fn cmp(&mut self) {
        unimplemented!();
    }

    /// CPX - Compare X Register.
    fn cpx(&mut self) {
        unimplemented!();
    }

    /// CPY - Compare Y Register.
    fn cpy(&mut self) {
        unimplemented!();
    }

    /// DEC - Decrement Memory.
    fn dec(&mut self) {
        unimplemented!();
    }

    /// DEX - Decrement X Register.
    fn dex(&mut self) {
        unimplemented!();
    }

    /// DEY - Decrement Y Register.
    fn dey(&mut self) {
        unimplemented!();
    }

    /// EOR - Exclusive OR.
    fn eor(&mut self) {
        unimplemented!();
    }

    /// INC - Increment Memory.
    fn inc(&mut self) {
        unimplemented!();
    }

    /// INX - Increment X Register.
    fn inx(&mut self) {
        unimplemented!();
    }

    /// INY - Increment Y Register.
    fn iny(&mut self) {
        unimplemented!();
    }

    /// JMP - Jump.
    fn jmp(&mut self) {
        unimplemented!();
    }

    /// JSR - Jump to Subroutine.
    fn jsr(&mut self) {
        unimplemented!();
    }

    /// LDA - Load Accumulator.
    fn lda(&mut self) {
        unimplemented!();
    }

    /// LDX - Load X Register.
    fn ldx(&mut self) {
        unimplemented!();
    }

    /// LDY - Load Y Register.
    fn ldy(&mut self) {
        unimplemented!();
    }

    /// LSR - Logical Shift Right.
    fn lsr(&mut self) {
        unimplemented!();
    }

    /// ORA - Logical Inclusive OR.
    fn ora(&mut self) {
        unimplemented!();
    }

    /// PHA - Push Accumulator.
    fn pha(&mut self) {
        unimplemented!();
    }

    /// PHP - Push Processor Status.
    fn php(&mut self) {
        unimplemented!();
    }

    /// PLA - Pull Accumulator.
    fn pla(&mut self) {
        unimplemented!();
    }

    /// PLP - Pull Processor Status.
    fn plp(&mut self) {
        unimplemented!();
    }

    /// ROL - Rotate Left.
    fn rol(&mut self) {
        unimplemented!();
    }

    /// ROR - Rotate Right.
    fn ror(&mut self) {
        unimplemented!();
    }

    /// RTI - Return from Interrupt.
    fn rti(&mut self) {
        unimplemented!();
    }

    /// RTS - Return from Subroutine.
    fn rts(&mut self) {
        unimplemented!();
    }

    /// SBC - Subtract with Carry.
    fn sbc(&mut self) {
        unimplemented!();
    }

    /// SEC - Set Carry Flag.
    fn sec(&mut self) {
        self.carry = true;
    }

    /// SED - Set Decimal Flag.
    fn sed(&mut self) {
        self.decimal_mode = true;
    }

    /// SEI - Set Interrupt Disable.
    fn sei(&mut self) {
        self.interrupt_disable = true;
    }

    /// STA - Store Accumulator.
    fn sta(&mut self) {
        unimplemented!();
    }

    /// STX - Store X Register.
    fn stx(&mut self) {
        unimplemented!();
    }

    /// STY - Store Y Register.
    fn sty(&mut self) {
        unimplemented!();
    }

    /// TAX - Transfer Accumulator to X.
    fn tax(&mut self) {
        self.x = self.a;

        // Set zero flag if appropriate.
        self.zero = self.x == 0;

        // Set negative flag if appropriate.
        self.negative = self.x >> 7 == 1;
    }

    /// TAY - Transfer Accumulator to Y.
    fn tay(&mut self) {
        self.y = self.a;

        // Set zero flag if appropriate.
        self.zero = self.y == 0;

        // Set negative flag if appropriate.
        self.negative = self.y >> 7 == 1;
    }

    /// TSX - Transfer Stack Pointer to X.
    fn tsx(&mut self) {
        unimplemented!();
    }

    /// TXA - Transfer X to Accumulator.
    fn txa(&mut self) {
        self.a = self.x;

        // Set zero flag if appropriate.
        self.zero = self.a == 0;

        // Set negative flag if appropriate.
        self.negative = self.a >> 7 == 1;
    }

    /// TXS - Transfer X to Stack Pointer.
    fn txs(&mut self) {
        unimplemented!();
    }

    /// TYA - Transfer Y to Accumulator.
    fn tya(&mut self) {
        self.a = self.y;

        // Set zero flag if appropriate.
        self.zero = self.a == 0;

        // Set negative flag if appropriate.
        self.negative = self.a >> 7 == 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

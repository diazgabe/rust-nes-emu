/// An NES emulator.
struct Emu {
    /// A flag that denotes whether the last operation caused either an overflow or underflow.
    carry: bool,
    /// A flag that denotes whether to follow the rules of Binary Coded Decimal (BCD) arithmetic during addition and subtraction.
    decimal_mode: bool,
    /// A flag that denotes whether to ignore interrupts.
    interrupt_disable: bool,
}

impl Emu {
    /// Instantiates a new NES emulator.
    fn new() -> Self {
        Self {
            carry: false,
            decimal_mode: false,
            interrupt_disable: true,
        }
    }

    /// Parses and calls an instruction.
    ///
    /// # Panics
    ///
    /// Panics if the instruction is not recognized.
    fn execute(&mut self, opcode: u8) {
        match opcode {
            0x18 => self.clc(),
            0xD8 => self.cld(),
            0x58 => self.cli(),
            0x38 => self.sec(),
            0xF8 => self.sed(),
            0x78 => self.sei(),
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
        unimplemented!();
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

    /// NOP - No Operation.
    fn nop(&mut self) {
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
        unimplemented!();
    }

    /// TAY - Transfer Accumulator to Y.
    fn tay(&mut self) {
        unimplemented!();
    }

    /// TSX - Transfer Stack Pointer to X.
    fn tsx(&mut self) {
        unimplemented!();
    }

    /// TXA - Transfer X to Accumulator.
    fn txa(&mut self) {
        unimplemented!();
    }

    /// TXS - Transfer X to Stack Pointer.
    fn txs(&mut self) {
        unimplemented!();
    }

    /// TYA - Transfer Y to Accumulator.
    fn tya(&mut self) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

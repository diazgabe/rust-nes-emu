struct Emu {}

impl Emu {
    // Instructions referenced from https://www.nesdev.org/obelisk-6502-guide/reference.html.

    /// ADC - Add with Carry.
    fn adc() {}

    /// AND - Logical AND.
    fn and() {}

    /// ASL - Arithmetic Shift Left.
    fn asl() {}

    /// BCC - Branch if Carry Clear.
    fn bcc() {}

    /// BCS - Branch if Carry Set.
    fn bcs() {}

    /// BEQ - Branch if Equal.
    fn beq() {}

    /// BIT - Bit Test.
    fn bit() {}

    /// BMI - Branch if Minus.
    fn bmi() {}

    /// BNE - Branch if Not Equal.
    fn bne() {}

    /// BPL - Branch if Positive.
    fn bpl() {}

    /// BRK - Force Interrupt.
    fn brk() {}

    /// BVC - Branch if Overflow Clear.
    fn bvc() {}

    /// BVS - Branch if Overflow Set.
    fn bvs() {}

    /// CLC - Clear Carry Flag.
    fn clc() {}

    /// CLD - Clear Decimal Mode.
    fn cld() {}

    /// CLI - Clear Interrupt Disable.
    fn cli() {}

    /// CLV - Clear Overflow Flag.
    fn clv() {}

    /// CMP - Compare.
    fn cmp() {}

    /// CPX - Compare X Register.
    fn cpx() {}

    /// CPY - Compare Y Register.
    fn cpy() {}

    /// DEC - Decrement Memory.
    fn dec() {}

    /// DEX - Decrement X Register.
    fn dex() {}

    /// DEY - Decrement Y Register.
    fn dey() {}

    /// EOR - Exclusive OR.
    fn eor() {}

    /// INC - Increment Memory.
    fn inc() {}

    /// INX - Increment X Register.
    fn inx() {}

    /// INY - Increment Y Register.
    fn iny() {}

    /// JMP - Jump.
    fn jmp() {}

    /// JSR - Jump to Subroutine.
    fn jsr() {}

    /// LDA - Load Accumulator.
    fn lda() {}

    /// LDX - Load X Register.
    fn ldx() {}

    /// LDY - Load Y Register.
    fn ldy() {}

    /// LSR - Logical Shift Right.
    fn lsr() {}

    /// NOP - No Operation.
    fn nop() {}

    /// ORA - Logical Inclusive OR.
    fn ora() {}

    /// PHA - Push Accumulator.
    fn pha() {}

    /// PHP - Push Processor Status.
    fn php() {}

    /// PLA - Pull Accumulator.
    fn pla() {}

    /// PLP - Pull Processor Status.
    fn plp() {}

    /// ROL - Rotate Left.
    fn rol() {}

    /// ROR - Rotate Right.
    fn ror() {}

    /// RTI - Return from Interrupt.
    fn rti() {}

    /// RTS - Return from Subroutine.
    fn rts() {}

    /// SBC - Subtract with Carry.
    fn sbc() {}

    /// SEC - Set Carry Flag.
    fn sec() {}

    /// SED - Set Decimal Flag.
    fn sed() {}

    /// SEI - Set Interrupt Disable.
    fn sei() {}

    /// STA - Store Accumulator.
    fn sta() {}

    /// STX - Store X Register.
    fn stx() {}

    /// STY - Store Y Register.
    fn sty() {}

    /// TAX - Transfer Accumulator to X.
    fn tax() {}

    /// TAY - Transfer Accumulator to Y.
    fn tay() {}

    /// TSX - Transfer Stack Pointer to X.
    fn tsx() {}

    /// TXA - Transfer X to Accumulator.
    fn txa() {}

    /// TXS - Transfer X to Stack Pointer.
    fn txs() {}

    /// TYA - Transfer Y to Accumulator.
    fn tya() {}
}

#[cfg(test)]
mod tests {
    use super::*;
}

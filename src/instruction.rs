use crate::{CpuHart, instruction_impl::*};

use bitfields::bitfield;

// They should make this into a game
#[bitfield(u32)]
pub struct RTypeInstruction {
    #[bits(7)]
    funct7: u8,

    #[bits(5)]
    rs2: u8,

    #[bits(5)]
    rs1: u8,

    #[bits(3)]
    funct3: u8,

    #[bits(5)]
    rd: u8,

    #[bits(7)]
    opcode: u8    
}

#[bitfield(u32)]
pub struct ITypeInstruction {
    #[bits(12)]
    imm: u16,

    #[bits(5)]
    rs1: u8,

    #[bits(3)]
    funct3: u8,

    #[bits(5)]
    rd: u8,

    #[bits(7)]
    opcode: u8    
}

#[bitfield(u32)]
pub struct STypeInstruction {
    #[bits(7)]
    imm: u8,

    #[bits(5)]
    rs2: u8,

    #[bits(5)]
    rs1: u8,

    #[bits(3)]
    funct3: u8,

    #[bits(5)]
    rd: u8,

    #[bits(7)]
    opcode: u8
}

#[bitfield(u32)]
pub struct UTypeInstruction {
    #[bits(20)]
    imm: u32,

    #[bits(5)]
    rd: u8,

    #[bits(7)]
    opcode: u8   
}

#[bitfield(u32)]
/// immpart1: Bit 12 |
/// immpart2: Bits 5 to 10 |
/// immpart3: Bits 1 to 4 |
/// immpart4: Bit 11 |
pub struct BTypeInstruction {
    #[bits(1)]
    immpart1: u8,
    
    #[bits(6)]    
    immpart2: u8,
    
    #[bits(5)]
    rs2: u8,
    
    #[bits(5)]
    rs1: u8,

    #[bits(3)]
    funct3: u8,
    
    #[bits(4)]
    immpart3: u8,
    
    #[bits(1)]
    immpart4: u8,
    
    #[bits(7)]
    opcode: u8
}

#[bitfield(u32)]
/// immpart1: Bit 20 |
/// immpart2: Bits 1 to 10 |
/// immpart3: Bit 11 |
/// immpart4: Bits 12 to 19
pub struct JTypeInstruction {
    #[bits(1)]
    immpart1: u8,

    #[bits(10)]
    immpart2: u16,

    #[bits(1)]
    immpart3: u8,

    immpart4: u8,

    #[bits(5)]
    rd: u8,

    #[bits(7)]
    opcode: u8  
}

pub enum Instruction {
    RType(RTypeInstruction),
    IType(ITypeInstruction),
    SType(STypeInstruction),
    UType(UTypeInstruction),
    BType(BTypeInstruction),
    JType(JTypeInstruction)
}

pub struct InstructionInfo<F>
where F: Fn(Instruction)
{
    callback: F,
    name: &'static str
}
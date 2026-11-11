use crate::{CpuHart, instruction_impl::*};

use bitfields::bitfield;

#[bitfield(u8)]
pub struct Opcode {
    #[bits(1)]
    _unused: u8,

    #[bits(3)]
    map_column: u8,

    #[bits(2)]
    map_row: u8,

    #[bits(2)]
    instr_len: u8
}

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

#[derive(Clone, Copy)]
pub enum InstructionCategoryId {
    Load,
    Store,
    Madd,
    Branch,
    LoadFp,
    StoreFp,
    Msub,
    Jalr,
    Nmsub,
    MiscMem,
    Amo,
    Nmadd,
    Jal,
    OpImm,
    Op,
    OpFp,
    System,
    Auipc,
    Lui,
    OpV,
    OpVe,
    OpImm32,
    Op32
}

#[derive(Clone, Copy)]
pub enum Instruction {
    RType(RTypeInstruction),
    IType(ITypeInstruction),
    SType(STypeInstruction),
    UType(UTypeInstruction),
    BType(BTypeInstruction),
    JType(JTypeInstruction)
}

#[derive(Clone, Copy)]
pub enum InstructionType {
    RType,
    IType,
    SType,
    UType,
    JType
}

impl InstructionCategoryId {
    pub fn from_opcode(opcode: Opcode) -> Option<InstructionCategoryId> {
        const CATEGORY_MAP_NUM_ROWS: usize = 8;
        const CATEGORY_MAP_NUM_COLS: usize = 4;

        #[rustfmt::skip]
        const CATEGORY_MAP: [[Option<InstructionCategoryId>; CATEGORY_MAP_NUM_COLS]; CATEGORY_MAP_NUM_ROWS] = [
            [ Some(InstructionCategoryId::Load),    Some(InstructionCategoryId::Store),   Some(InstructionCategoryId::Madd),  Some(InstructionCategoryId::Branch) ],
            [ Some(InstructionCategoryId::LoadFp),  Some(InstructionCategoryId::StoreFp), Some(InstructionCategoryId::Msub),  Some(InstructionCategoryId::Jalr) ],
            [ None,                                 None,                                 Some(InstructionCategoryId::Nmsub), None ],
            [ Some(InstructionCategoryId::MiscMem), Some(InstructionCategoryId::Amo),     Some(InstructionCategoryId::Nmadd), Some(InstructionCategoryId::Jal)],
            [ Some(InstructionCategoryId::OpImm),   Some(InstructionCategoryId::Op),      Some(InstructionCategoryId::OpFp),  Some(InstructionCategoryId::System) ],
            [ Some(InstructionCategoryId::Auipc),   Some(InstructionCategoryId::Lui),     Some(InstructionCategoryId::OpV),   Some(InstructionCategoryId::OpVe) ],
            [ Some(InstructionCategoryId::OpImm32), Some(InstructionCategoryId::Op32),    None,                               None ],
            [ None,                                 None,                                 None,                               None ]
        ];

        CATEGORY_MAP[opcode.map_row() as usize][opcode.map_column() as usize]
    }

    pub fn call(&self, opcode: Opcode, instruction: Instruction, hart: &mut CpuHart) {
        match self {
            InstructionCategoryId::Load => (LOAD.0)(opcode, instruction, hart),
            InstructionCategoryId::Store => (STORE.0)(opcode, instruction, hart),
            InstructionCategoryId::Madd => (MADD.0)(opcode, instruction, hart),
            InstructionCategoryId::Branch => (BRANCH.0)(opcode, instruction, hart),
            InstructionCategoryId::LoadFp => (LOADFP.0)(opcode, instruction, hart),
            InstructionCategoryId::StoreFp => (STOREFP.0)(opcode, instruction, hart),
            InstructionCategoryId::Msub => (MSUB.0)(opcode, instruction, hart),
            InstructionCategoryId::Jalr => (JALR.0)(opcode, instruction, hart),
            InstructionCategoryId::Nmsub => (NMSUB.0)(opcode, instruction, hart),
            InstructionCategoryId::MiscMem => (MISCMEM.0)(opcode, instruction, hart),
            InstructionCategoryId::Amo => (AMO.0)(opcode, instruction, hart),
            InstructionCategoryId::Nmadd => (NMADD.0)(opcode, instruction, hart),
            InstructionCategoryId::Jal => (JAL.0)(opcode, instruction, hart),
            InstructionCategoryId::OpImm => (OPIMM.0)(opcode, instruction, hart),
            InstructionCategoryId::Op => (OP.0)(opcode, instruction, hart),
            InstructionCategoryId::OpFp => (OPFP.0)(opcode, instruction, hart),
            InstructionCategoryId::System => (SYSTEM.0)(opcode, instruction, hart),
            InstructionCategoryId::Auipc => (AUIPC.0)(opcode, instruction, hart),
            InstructionCategoryId::Lui => (LUI.0)(opcode, instruction, hart),
            InstructionCategoryId::OpV => (OPV.0)(opcode, instruction, hart),
            InstructionCategoryId::OpVe => (OPVE.0)(opcode, instruction, hart),
            InstructionCategoryId::OpImm32 => (OPIMM32.0)(opcode, instruction, hart),
            InstructionCategoryId::Op32 => (OP32.0)(opcode, instruction, hart)
        }
    }
}
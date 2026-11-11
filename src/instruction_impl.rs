use crate::instruction::{Opcode, Instruction};
use crate::CpuHart;

#[derive(Clone, Copy)]
pub struct InstructionCategoryImpl(pub fn(Opcode, Instruction, &mut CpuHart));

pub const LOAD: InstructionCategoryImpl = InstructionCategoryImpl(load);
pub const STORE: InstructionCategoryImpl = InstructionCategoryImpl(store);
pub const MADD: InstructionCategoryImpl = InstructionCategoryImpl(madd);
pub const BRANCH: InstructionCategoryImpl = InstructionCategoryImpl(branch);
pub const LOADFP: InstructionCategoryImpl = InstructionCategoryImpl(loadfp);
pub const STOREFP: InstructionCategoryImpl = InstructionCategoryImpl(storefp);
pub const MSUB: InstructionCategoryImpl = InstructionCategoryImpl(msub);
pub const JALR: InstructionCategoryImpl = InstructionCategoryImpl(jalr);
pub const NMSUB: InstructionCategoryImpl = InstructionCategoryImpl(nmsub);
pub const MISCMEM: InstructionCategoryImpl = InstructionCategoryImpl(miscmem);
pub const AMO: InstructionCategoryImpl = InstructionCategoryImpl(amo);
pub const NMADD: InstructionCategoryImpl = InstructionCategoryImpl(nmadd);
pub const JAL: InstructionCategoryImpl = InstructionCategoryImpl(jal);
pub const OPIMM: InstructionCategoryImpl = InstructionCategoryImpl(opimm);
pub const OP: InstructionCategoryImpl = InstructionCategoryImpl(op);
pub const OPFP: InstructionCategoryImpl = InstructionCategoryImpl(opfp);
pub const SYSTEM: InstructionCategoryImpl = InstructionCategoryImpl(system);
pub const AUIPC: InstructionCategoryImpl = InstructionCategoryImpl(auipc);
pub const LUI: InstructionCategoryImpl = InstructionCategoryImpl(lui);
pub const OPV: InstructionCategoryImpl = InstructionCategoryImpl(opv);
pub const OPVE: InstructionCategoryImpl = InstructionCategoryImpl(opve);
pub const OPIMM32: InstructionCategoryImpl = InstructionCategoryImpl(opimm32);
pub const OP32: InstructionCategoryImpl = InstructionCategoryImpl(op32);

pub fn load(opcode: Opcode, instruction: Instruction, hart: &mut CpuHart) { todo!(); }
pub fn store(opcode: Opcode, instruction: Instruction, hart: &mut CpuHart) { todo!(); }
pub fn madd(opcode: Opcode, instruction: Instruction, hart: &mut CpuHart) { todo!(); }
pub fn branch(opcode: Opcode, instruction: Instruction, hart: &mut CpuHart) { todo!(); }
pub fn loadfp(opcode: Opcode, instruction: Instruction, hart: &mut CpuHart) { todo!(); }
pub fn storefp(opcode: Opcode, instruction: Instruction, hart: &mut CpuHart) { todo!(); }
pub fn msub(opcode: Opcode, instruction: Instruction, hart: &mut CpuHart) { todo!(); }
pub fn jalr(opcode: Opcode, instruction: Instruction, hart: &mut CpuHart) { todo!(); }
pub fn nmsub(opcode: Opcode, instruction: Instruction, hart: &mut CpuHart) { todo!(); }
pub fn miscmem(opcode: Opcode, instruction: Instruction, hart: &mut CpuHart) { todo!(); }
pub fn amo(opcode: Opcode, instruction: Instruction, hart: &mut CpuHart) { todo!(); }
pub fn nmadd(opcode: Opcode, instruction: Instruction, hart: &mut CpuHart) { todo!(); }
pub fn jal(opcode: Opcode, instruction: Instruction, hart: &mut CpuHart) { todo!(); }
pub fn opimm(opcode: Opcode, instruction: Instruction, hart: &mut CpuHart) { todo!(); }
pub fn op(opcode: Opcode, instruction: Instruction, hart: &mut CpuHart) { todo!(); }
pub fn opfp(opcode: Opcode, instruction: Instruction, hart: &mut CpuHart) { todo!(); }
pub fn system(opcode: Opcode, instruction: Instruction, hart: &mut CpuHart) { todo!(); }
pub fn auipc(opcode: Opcode, instruction: Instruction, hart: &mut CpuHart) { todo!(); }
pub fn lui(opcode: Opcode, instruction: Instruction, hart: &mut CpuHart) { todo!(); }
pub fn opv(opcode: Opcode, instruction: Instruction, hart: &mut CpuHart) { todo!(); }
pub fn opve(opcode: Opcode, instruction: Instruction, hart: &mut CpuHart) { todo!(); }
pub fn opimm32(opcode: Opcode, instruction: Instruction, hart: &mut CpuHart) { todo!(); }
pub fn op32(opcode: Opcode, instruction: Instruction, hart: &mut CpuHart) { todo!(); }
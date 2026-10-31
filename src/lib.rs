#![no_std]
// This is a library
#![allow(dead_code)]
#![feature(const_trait_impl, maybe_uninit_array_assume_init, f128)]

use alloc::vec::Vec;

extern crate alloc;

pub mod config;
pub mod registers;

pub const MAX_HEAP_BASED_HART_LENGTH: usize = 128;
pub const CSR_LEN: usize = 4096;

pub(crate) const trait FromConfig {
    fn from_config(config: &config::Config) -> Self;
}

pub struct CpuBase {
    pub config: config::Config,
    pub harts: CpuHarts,
    pub memory: Memory,
    pub csr: [u8; CSR_LEN]
}

pub enum CpuHarts {
    StackBacked(arrayvec::ArrayVec<CpuHart, MAX_HEAP_BASED_HART_LENGTH>),
    HeapBacked(alloc::vec::Vec<CpuHart>),
}

pub struct CpuHart {
    pub integer_registers: [registers::IntegerRegister; 32],
    pub float_registers: [registers::FloatRegister; 32],
    pub vector_registers: [registers::VectorRegister; 32],
    pub pc: registers::IntegerRegister,
}

pub struct Memory {
    pub base_ptr: *mut u8,
    pub len: usize,
    pub backing_buf: Vec<u8>
}

impl FromConfig for CpuBase {
    fn from_config(config: &config::Config) -> Self {
        let harts = CpuHarts::from_config(config);
        let memory = Memory::from_config(config);

        CpuBase { config: *config, harts, memory, csr: [0u8; CSR_LEN] }
    }
}

impl FromConfig for CpuHarts {
    fn from_config(config: &config::Config) -> Self {
        let mut val = if config.num_harts <= MAX_HEAP_BASED_HART_LENGTH {
            CpuHarts::StackBacked(arrayvec::ArrayVec::new_const())

        } else {
            CpuHarts::HeapBacked(Vec::new())
        };

        for _ in 0..config.num_harts {
            val.push_init_hart_from_config(config);
        }

        val
    }
}

impl FromConfig for CpuHart {
    fn from_config(config: &config::Config) -> Self {
        use core::mem::MaybeUninit;

        let integer_registers = [registers::IntegerRegister::from_config(config); 32];
        let float_registers = [registers::FloatRegister::from_config(config); 32];
        let mut vector_registers: [MaybeUninit<registers::VectorRegister>; 32] =
            [const { MaybeUninit::uninit() }; 32];

        for reg in &mut vector_registers {
            reg.write(registers::VectorRegister::from_config(config));
        }

        let mut pc = registers::IntegerRegister::from_config(config);
        pc.set_u64(config.starting_pc);

        CpuHart {
            integer_registers,
            float_registers,
            vector_registers: unsafe { MaybeUninit::array_assume_init(vector_registers) },
            pc,
        }
    }
}

impl FromConfig for Memory {
    fn from_config(config: &config::Config) -> Self {
        let mut backing_buf = Vec::with_capacity(config.memory_len as usize);
        let base_ptr = backing_buf.as_mut_ptr();
        let len = backing_buf.len();

        Memory { base_ptr, len, backing_buf }
    }
}

impl CpuHarts {
    fn push_init_hart_from_config(&mut self, config: &config::Config) {
        match self {
            CpuHarts::StackBacked(sb) => sb.try_push(CpuHart::from_config(config)).unwrap(),
            CpuHarts::HeapBacked(hb) => hb.push(CpuHart::from_config(config)),
        }
    }
}
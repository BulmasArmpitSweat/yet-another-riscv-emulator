use crate::alloc::vec::Vec;
use crate::FromConfig;

#[derive(Debug, Clone, Copy)]
pub enum IntegerRegister {
    Word(u32),
    DWord(u64)
}

#[derive(Debug, Clone, Copy)]
pub enum FloatRegister {
    SinglePrec(f32),
    DoublePrec(f64),
    QuadPrec(f128)
}

pub const VECTOR_MAX_STACK_BASED_SIZE: usize = 512;

#[derive(Debug, Clone)]
pub enum VectorRegister {
    StackBased(arrayvec::ArrayVec<u8, VECTOR_MAX_STACK_BASED_SIZE>),
    HeapBased(Vec<u8>)
}

impl IntegerRegister {
    pub fn set_u64(&mut self, val: u64) {
        match self {
            IntegerRegister::Word(w) => {
                let upper_half = val >> 32;
                *w = upper_half as u32;
            },
            IntegerRegister::DWord(d) => *d = val
        }
    }

    pub fn set_u32(&mut self, val: u32) {
        match self {
            IntegerRegister::Word(w) => *w = val,
            IntegerRegister::DWord(d) => *d = val as u64
        }
    }
}

impl FromConfig for IntegerRegister {
    fn from_config(config: &crate::config::Config) -> Self {
        match config.xlen {
            32 => IntegerRegister::Word(0),
            64 => IntegerRegister::DWord(0),
            _ => panic!("Unknown xlen value")
        }
    }
}

impl FromConfig for FloatRegister {
    fn from_config(config: &crate::config::Config) -> Self {
        match config.flen {
            32 => FloatRegister::SinglePrec(0.0f32),
            64 => FloatRegister::DoublePrec(0.0f64),
            128 => FloatRegister::QuadPrec(0.0f128),
            _ => panic!("Unknown flen value")
        }
    }
}

impl FromConfig for VectorRegister {
    fn from_config(config: &crate::config::Config) -> Self {
        if config.vlen <= VECTOR_MAX_STACK_BASED_SIZE {
            VectorRegister::StackBased(arrayvec::ArrayVec::new_const())
        } else {
            VectorRegister::HeapBased(Vec::new())
        }
    }
}

impl core::ops::Add for IntegerRegister {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (IntegerRegister::Word(a), IntegerRegister::Word(b)) => IntegerRegister::Word(a + b),
            (IntegerRegister::DWord(a), IntegerRegister::DWord(b)) => IntegerRegister::DWord(a + b),
            _ => panic!("Registers cannot differ in size")
        }
    }
}

impl core::ops::Sub for IntegerRegister {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (IntegerRegister::Word(a), IntegerRegister::Word(b)) => IntegerRegister::Word(a - b),
            (IntegerRegister::DWord(a), IntegerRegister::DWord(b)) => IntegerRegister::DWord(a - b),
            _ => panic!("Registers cannot differ in size")
        }
    }
}

impl core::ops::Mul for IntegerRegister {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (IntegerRegister::Word(a), IntegerRegister::Word(b)) => IntegerRegister::Word(a * b),
            (IntegerRegister::DWord(a), IntegerRegister::DWord(b)) => IntegerRegister::DWord(a * b),
            _ => panic!("Registers cannot differ in size")
        }
    }
}

impl core::ops::Div for IntegerRegister {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (IntegerRegister::Word(a), IntegerRegister::Word(b)) => IntegerRegister::Word(a / b),
            (IntegerRegister::DWord(a), IntegerRegister::DWord(b)) => IntegerRegister::DWord(a / b),
            _ => panic!("Registers cannot differ in size")
        }
    }
}

impl core::ops::AddAssign for IntegerRegister {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl core::ops::SubAssign for IntegerRegister {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl core::ops::MulAssign for IntegerRegister {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl core::ops::DivAssign for IntegerRegister {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl core::ops::BitAnd for IntegerRegister {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (IntegerRegister::Word(a), IntegerRegister::Word(b)) => IntegerRegister::Word(a & b),
            (IntegerRegister::DWord(a), IntegerRegister::DWord(b)) => IntegerRegister::DWord(a & b),
            _ => panic!("Registers cannot differ in size")
        }
    }
}

impl core::ops::BitOr for IntegerRegister {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (IntegerRegister::Word(a), IntegerRegister::Word(b)) => IntegerRegister::Word(a | b),
            (IntegerRegister::DWord(a), IntegerRegister::DWord(b)) => IntegerRegister::DWord(a | b),
            _ => panic!("Registers cannot differ in size")
        }
    }
}

impl core::ops::BitXor for IntegerRegister {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (IntegerRegister::Word(a), IntegerRegister::Word(b)) => IntegerRegister::Word(a ^ b),
            (IntegerRegister::DWord(a), IntegerRegister::DWord(b)) => IntegerRegister::DWord(a ^ b),
            _ => panic!("Registers cannot differ in size")
        }
    }
}

impl core::ops::BitAndAssign for IntegerRegister {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl core::ops::BitOrAssign for IntegerRegister {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl core::ops::BitXorAssign for IntegerRegister {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl core::ops::Not for IntegerRegister {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            IntegerRegister::Word(a) => IntegerRegister::Word(!a),
            IntegerRegister::DWord(a) => IntegerRegister::DWord(!a)
        }
    }
}

impl core::ops::Shl for IntegerRegister {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (IntegerRegister::Word(a), IntegerRegister::Word(b)) => IntegerRegister::Word(a << b),
            (IntegerRegister::DWord(a), IntegerRegister::DWord(b)) => IntegerRegister::DWord(a << b),
            _ => panic!("Registers cannot differ in size")
        }
    }
}

impl core::ops::Shr for IntegerRegister {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (IntegerRegister::Word(a), IntegerRegister::Word(b)) => IntegerRegister::Word(a >> b),
            (IntegerRegister::DWord(a), IntegerRegister::DWord(b)) => IntegerRegister::DWord(a >> b),
            _ => panic!("Registers cannot differ in size")
        }
    }
}

impl core::ops::ShlAssign for IntegerRegister {
    fn shl_assign(&mut self, rhs: Self) {
        *self = *self << rhs;
    }
}

impl core::ops::ShrAssign for IntegerRegister {
    fn shr_assign(&mut self, rhs: Self) {
        *self = *self >> rhs;
    }
}

impl PartialEq for IntegerRegister {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (IntegerRegister::Word(a), IntegerRegister::Word(b)) => a == b,
            (IntegerRegister::DWord(a), IntegerRegister::DWord(b)) => a == b,
            _ => false
        }
    }
}

impl core::ops::Add for FloatRegister {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (FloatRegister::SinglePrec(a), FloatRegister::SinglePrec(b)) => FloatRegister::SinglePrec(a + b),
            (FloatRegister::DoublePrec(a), FloatRegister::DoublePrec(b)) => FloatRegister::DoublePrec(a + b),
            (FloatRegister::QuadPrec(a), FloatRegister::QuadPrec(b)) => FloatRegister::QuadPrec(a + b),
            _ => panic!("Registers cannot differ in size")
        }
    }
}

impl core::ops::Sub for FloatRegister {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (FloatRegister::SinglePrec(a), FloatRegister::SinglePrec(b)) => FloatRegister::SinglePrec(a - b),
            (FloatRegister::DoublePrec(a), FloatRegister::DoublePrec(b)) => FloatRegister::DoublePrec(a - b),
            (FloatRegister::QuadPrec(a), FloatRegister::QuadPrec(b)) => FloatRegister::QuadPrec(a - b),
            _ => panic!("Registers cannot differ in size")
        }
    }
}

impl core::ops::Mul for FloatRegister {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (FloatRegister::SinglePrec(a), FloatRegister::SinglePrec(b)) => FloatRegister::SinglePrec(a * b),
            (FloatRegister::DoublePrec(a), FloatRegister::DoublePrec(b)) => FloatRegister::DoublePrec(a * b),
            (FloatRegister::QuadPrec(a), FloatRegister::QuadPrec(b)) => FloatRegister::QuadPrec(a * b),
            _ => panic!("Registers cannot differ in size")
        }
    }
}

impl core::ops::Div for FloatRegister {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (FloatRegister::SinglePrec(a), FloatRegister::SinglePrec(b)) => FloatRegister::SinglePrec(a / b),
            (FloatRegister::DoublePrec(a), FloatRegister::DoublePrec(b)) => FloatRegister::DoublePrec(a / b),
            (FloatRegister::QuadPrec(a), FloatRegister::QuadPrec(b)) => FloatRegister::QuadPrec(a / b),
            _ => panic!("Registers cannot differ in size")
        }
    }
}

impl core::ops::AddAssign for FloatRegister {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl core::ops::SubAssign for FloatRegister {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl core::ops::MulAssign for FloatRegister {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl core::ops::DivAssign for FloatRegister {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl PartialEq for FloatRegister {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FloatRegister::SinglePrec(a), FloatRegister::SinglePrec(b)) => a == b,
            (FloatRegister::DoublePrec(a), FloatRegister::DoublePrec(b)) => a == b,
            (FloatRegister::QuadPrec(a), FloatRegister::QuadPrec(b)) => a == b,
            _ => false
        }
    }
}
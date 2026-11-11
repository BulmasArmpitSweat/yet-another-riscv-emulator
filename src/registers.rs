use crate::alloc::vec::Vec;
use crate::FromConfig;
use crate::utils::Sizeof;

#[derive(Debug, Clone, Copy)]
pub enum IntegerRegister {
    Word(u32),
    DWord(u64),
    QWord(u128)
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

impl Default for IntegerRegister {
    fn default() -> Self {
        IntegerRegister::Word(0)
    }
}

impl Default for FloatRegister {
    fn default() -> Self {
        FloatRegister::SinglePrec(0.)
    }
}

impl Default for VectorRegister {
    fn default() -> Self {
        VectorRegister::StackBased(arrayvec::ArrayVec::new_const())
    }
}

impl IntegerRegister {
    pub const fn to_usize(self) -> usize {
        match self {
            IntegerRegister::Word(w) => w as usize,
            IntegerRegister::DWord(d) => d as usize,
            IntegerRegister::QWord(q) => q as usize,
        }
    }

    pub const fn larger_than_usize(&self) -> bool {
        self.byte_length() > core::mem::size_of::<usize>()
    }
    
    pub const fn byte_length(&self) -> usize {
        match self {
            IntegerRegister::Word(_) => core::mem::size_of::<u32>(),
            IntegerRegister::DWord(_) => core::mem::size_of::<u64>(),
            IntegerRegister::QWord(_) => core::mem::size_of::<u128>()
        }
    }
    pub fn set_u128(&mut self, val: u128) {
        match self {
            IntegerRegister::Word(w) => {
                let upper_half = val >> 96;
                *w = upper_half as u32;
            },
            IntegerRegister::DWord(d) => {
                let upper_half = val >> 64;
                *d = upper_half as u64;
            },
            IntegerRegister::QWord(q) => *q = val
        }
    }

    pub fn set_u64(&mut self, val: u64) {
        match self {
            IntegerRegister::Word(w) => {
                let upper_half = val >> 32;
                *w = upper_half as u32;
            },
            IntegerRegister::DWord(d) => *d = val,
            IntegerRegister::QWord(q) => *q = val as u128
        }
    }

    pub fn set_u32(&mut self, val: u32) {
        match self {
            IntegerRegister::Word(w) => *w = val,
            IntegerRegister::DWord(d) => *d = val as u64,
            IntegerRegister::QWord(q) => *q = val as u128
        }
    }
}

impl FromConfig for IntegerRegister {
    fn from_config(config: &crate::config::Config) -> Self {
        match config.xlen {
            32 => IntegerRegister::Word(0),
            64 => IntegerRegister::DWord(0),
            128 => IntegerRegister::QWord(0),
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

macro_rules! int_reg_arith_impl {
    ($trait:path, $fnname:tt, $op:tt) => {
        impl $trait for IntegerRegister {
            type Output = Self;

            fn $fnname(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    (IntegerRegister::Word(a), IntegerRegister::Word(b)) => IntegerRegister::Word(a $op b),
                    (IntegerRegister::DWord(a), IntegerRegister::DWord(b)) => IntegerRegister::DWord(a $op b),
                    (IntegerRegister::QWord(a), IntegerRegister::QWord(b)) => IntegerRegister::QWord(a $op b),
                    _ => panic!("Registers cannot differ in size")
                }
            }
        }
    };
}

macro_rules! int_reg_arith_assign_impl {
    ($trait:path, $fnname:tt, $op:tt) => {
        impl $trait for IntegerRegister {
            fn $fnname(&mut self, rhs: Self) {
                *self = *self $op rhs;
            }
        }
    };
}

macro_rules! float_reg_arith_impl {
    ($trait:path, $fnname:tt, $op:tt) => {
        impl $trait for FloatRegister {
            type Output = Self;

            fn $fnname(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    (FloatRegister::SinglePrec(a), FloatRegister::SinglePrec(b)) => FloatRegister::SinglePrec(a $op b),
                    (FloatRegister::DoublePrec(a), FloatRegister::DoublePrec(b)) => FloatRegister::DoublePrec(a $op b),
                    (FloatRegister::QuadPrec(a), FloatRegister::QuadPrec(b)) => FloatRegister::QuadPrec(a $op b),
                    _ => panic!("Registers cannot differ in size")
                }
            }
        }
    };
}

macro_rules! float_reg_arith_assign_impl {
    ($trait:path, $fnname:tt, $op:tt) => {
        impl $trait for FloatRegister {
            fn $fnname(&mut self, rhs: Self) {
                *self = *self $op rhs;
            }
        }
    };
}

int_reg_arith_impl!(core::ops::Add, add, +);
int_reg_arith_impl!(core::ops::Sub, sub, -);
int_reg_arith_impl!(core::ops::Mul, mul, *);
int_reg_arith_impl!(core::ops::Div, div, /);
int_reg_arith_impl!(core::ops::BitAnd, bitand, &);
int_reg_arith_impl!(core::ops::BitOr, bitor, |);
int_reg_arith_impl!(core::ops::BitXor, bitxor, ^);
int_reg_arith_impl!(core::ops::Shl, shl, <<);
int_reg_arith_impl!(core::ops::Shr, shr, >>);

int_reg_arith_assign_impl!(core::ops::AddAssign, add_assign, +);
int_reg_arith_assign_impl!(core::ops::SubAssign, sub_assign, -);
int_reg_arith_assign_impl!(core::ops::MulAssign, mul_assign, *);
int_reg_arith_assign_impl!(core::ops::DivAssign, div_assign, /);
int_reg_arith_assign_impl!(core::ops::BitAndAssign, bitand_assign, &);
int_reg_arith_assign_impl!(core::ops::BitOrAssign, bitor_assign, |);
int_reg_arith_assign_impl!(core::ops::BitXorAssign, bitxor_assign, ^);
int_reg_arith_assign_impl!(core::ops::ShlAssign, shl_assign, <<);
int_reg_arith_assign_impl!(core::ops::ShrAssign, shr_assign, >>);

float_reg_arith_impl!(core::ops::Add, add, +);
float_reg_arith_impl!(core::ops::Sub, sub, -);
float_reg_arith_impl!(core::ops::Mul, mul, *);
float_reg_arith_impl!(core::ops::Div, div, /);

float_reg_arith_assign_impl!(core::ops::AddAssign, add_assign, +);
float_reg_arith_assign_impl!(core::ops::SubAssign, sub_assign, -);
float_reg_arith_assign_impl!(core::ops::MulAssign, mul_assign, *);
float_reg_arith_assign_impl!(core::ops::DivAssign, div_assign, /);

impl PartialEq for IntegerRegister {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (IntegerRegister::Word(a), IntegerRegister::Word(b)) => a == b,
            (IntegerRegister::DWord(a), IntegerRegister::DWord(b)) => a == b,
            (IntegerRegister::QWord(a), IntegerRegister::QWord(b)) => a == b,
            _ => false
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !Self::eq(&self, other)
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

    fn ne(&self, other: &Self) -> bool {
        !Self::eq(&self, other)
    }
}

impl PartialOrd for IntegerRegister {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match (self, other) {
            (IntegerRegister::Word(a), IntegerRegister::Word(b)) => Some(a.cmp(b)),
            (IntegerRegister::DWord(a), IntegerRegister::DWord(b)) => Some(a.cmp(b)),
            (IntegerRegister::QWord(a), IntegerRegister::QWord(b)) => Some(a.cmp(b)),
            _ => None
        }
    }
}

impl PartialOrd for FloatRegister {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        macro_rules! equality_because_im_lazy {
            ($a:expr, $b:expr) => {
                if $a < $b {
                    Some(core::cmp::Ordering::Less)
                } else if $a > $b {
                    Some(core::cmp::Ordering::Greater)
                } else {
                    Some(core::cmp::Ordering::Equal)
                }
            };
        }

        match (self, other) {
            (FloatRegister::SinglePrec(a), FloatRegister::SinglePrec(b)) => {
                equality_because_im_lazy!(a, b)
            },
            (FloatRegister::DoublePrec(a), FloatRegister::DoublePrec(b)) => {
                equality_because_im_lazy!(a, b)
            },
            (FloatRegister::QuadPrec(a), FloatRegister::QuadPrec(b)) => {
                equality_because_im_lazy!(a, b)
            },
            _ => None
        }
    }
}

pub(crate) trait IntoIntegerRegister {
    fn into_integer_register(self) -> IntegerRegister;
}

impl IntoIntegerRegister for u8 {
    fn into_integer_register(self) -> IntegerRegister {
        IntegerRegister::Word(self as u32)
    }
}

impl IntoIntegerRegister for u16 {
    fn into_integer_register(self) -> IntegerRegister {
        IntegerRegister::Word(self as u32)
    }
}

impl IntoIntegerRegister for u32 {
    fn into_integer_register(self) -> IntegerRegister {
        IntegerRegister::Word(self)
    }
}

impl IntoIntegerRegister for u64 {
    fn into_integer_register(self) -> IntegerRegister {
        IntegerRegister::DWord(self)
    }
}

impl IntoIntegerRegister for u128 {
    fn into_integer_register(self) -> IntegerRegister {
        IntegerRegister::QWord(self)
    }
}

impl IntoIntegerRegister for usize {
    fn into_integer_register(self) -> IntegerRegister {
        if usize::sizeof() <= IntegerRegister::DWord(0).sizeof_self() {
            IntegerRegister::Word(self as u32)
        } else if usize::sizeof() <= IntegerRegister::QWord(0).sizeof_self() {
            IntegerRegister::DWord(self as u64)
        } else {
            IntegerRegister::QWord(self as u128)
        }
    }
}
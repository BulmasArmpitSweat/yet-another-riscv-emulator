#[derive(Clone, Copy)]
#[repr(u8)]
pub enum ExceptionType {
    InstrAccessFault = 1,
    IllegalInstruction = 2,
    Breakpoint = 3,
    LoadAccessFault = 4,
    StoreAmoAccessFault = 7,
    ECallUMode = 8,
    ECallMMode = 11,
    InstrBusFault = 24,
    InstrParityChecksumFault = 25
}

impl ExceptionType {
    pub fn priority(&self) -> u8 {
        match self {
            ExceptionType::InstrAccessFault => 8,
            ExceptionType::InstrParityChecksumFault => 7,
            ExceptionType::InstrBusFault => 6,
            ExceptionType::IllegalInstruction => 5,
            ExceptionType::ECallUMode => 4,
            ExceptionType::ECallMMode => 3,
            ExceptionType::Breakpoint => 2,
            ExceptionType::StoreAmoAccessFault => 1,
            ExceptionType::LoadAccessFault => 0
        }
    }
}

impl PartialEq for ExceptionType {
    fn eq(&self, other: &Self) -> bool {
        self.priority() == other.priority()
    }
}

impl Eq for ExceptionType {}

impl PartialOrd for ExceptionType {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.priority().partial_cmp(&other.priority())
    }
}

impl Ord for ExceptionType {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.priority().cmp(&other.priority())
    }
}

// For potential forwards compatibility
unsafe impl Send for ExceptionType {}
unsafe impl Sync for ExceptionType {}
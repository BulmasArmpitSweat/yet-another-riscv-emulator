use enable_elements::EnableElement;

use crate::{FromConfig, registers::{IntegerRegister}};

pub const NUM_HARTS_DEFAULT: usize = 1;
pub const STARTING_PC_DEFAULT: u64 = 0x80000000;
pub const MEMORY_LENGTH_DEFAULT: u64 = STARTING_PC_DEFAULT;
pub const BASE_ISA_DEFAULT: BaseExtension = BaseExtension::Rv32i;

pub const FULL_REGISTER_FILE_WIDTH: usize = 32;
pub const REDUCED_REGISTER_FILE_WIDTH: usize = 16;

pub enum IntegerRegisterFile {
    Full([IntegerRegister; FULL_REGISTER_FILE_WIDTH]),
    Reduced([IntegerRegister; REDUCED_REGISTER_FILE_WIDTH])
}

impl core::ops::Index<usize> for IntegerRegisterFile {
    type Output = IntegerRegister;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            IntegerRegisterFile::Full(f) => &f[index],
            IntegerRegisterFile::Reduced(r) => &r[index]
        }
    }
}

impl core::ops::IndexMut<usize> for IntegerRegisterFile {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            IntegerRegisterFile::Full(f) => &mut f[index],
            IntegerRegisterFile::Reduced(r) => &mut r[index]
        }
    }
}

impl FromConfig for IntegerRegisterFile {
    fn from_config(config: &self::Config) -> Self {
        if config.base_isa.integer_register_file_reduced() {
            IntegerRegisterFile::Reduced([IntegerRegister::from_config(config); REDUCED_REGISTER_FILE_WIDTH])
        } else {
            IntegerRegisterFile::Full([IntegerRegister::from_config(config); FULL_REGISTER_FILE_WIDTH])
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum BaseExtension {
    #[default]
    Rv32i,
    Rv64i,
    Rv32e,
    Rv64e,
    Rv128i
}

// Reference: https://tech.riscv.org/isa-explorer/
#[derive(Debug, Default, EnableElement)]
pub struct EnabledExtensions {
    pub a: bool,
    pub b: bool,
    pub c: bool,
    pub d: bool,
    pub f: bool,
    pub h: bool,
    pub l: bool,
    pub m: bool,
    pub p: bool,
    pub q: bool,
    pub s: bool,
    pub u: bool,
    pub t: bool,
    pub v: bool,
    pub sm: bool,
    pub zba: bool,
    pub zbb: bool,
    pub zbc: bool,
    pub zbs: bool,
    pub za128rs: bool,
    pub zaamo: bool,
    pub zacas: bool,
    pub zalrsc: bool,
    pub zawrs: bool, // rawr
    pub za64rs: bool,
    pub zabha: bool,
    pub zalasr: bool,
    pub zama16b: bool,
    pub ziccrse: bool,
    pub zca: bool,
    pub zcb: bool,
    pub zcd: bool,
    pub zce: bool,
    pub zcf: bool,
    pub zcmp: bool,
    pub zcmt: bool,
    pub zcmop: bool,
    pub zclsd: bool,
    pub zdinx: bool,
    pub zfa: bool,
    pub zfbfmin: bool,
    pub zfh: bool,
    pub zfhmin: bool,
    pub zfinx: bool,
    pub zhinx: bool,
    pub zhinxmin: bool,
    pub zicclsm: bool,
    pub zilsd: bool,
    pub zibi: bool,
    pub zicond: bool,
    pub zmmul: bool,
    pub zve32x: bool,
    pub zve32f: bool,
    pub zve64x: bool,
    pub zve64f: bool,
    pub zve64d: bool,
    pub zvl64b: bool,
    pub zvl128b: bool,
    pub zvl256b: bool,
    pub zvl512b: bool,
    pub zvl1024b: bool,
    pub zvl2048b: bool,
    pub zvl4096b: bool,
    pub zvl8192b: bool,
    pub zvl16384b: bool,
    pub zvl32768b: bool,
    pub zvl65536b: bool,
    pub zvfh: bool,
    pub zvfhmin: bool,
    pub zvfbfmin: bool,
    pub zvfbfa: bool,
    pub zvfbfwma: bool,
    pub zvfofp8min: bool,
    pub zvabd: bool,
    pub zvbb: bool,
    pub zvbc: bool,
    pub zvbdota: bool,
    pub zvdota: bool,
    pub zvdot4a: bool,
    pub smcfiss: bool,
    pub zicflip: bool,
    pub zicfiss: bool,
    pub zbkb: bool,
    pub zbkc: bool,
    pub zbkx: bool,
    pub zk: bool,
    pub zkn: bool,
    pub zknd: bool,
    pub zkne: bool,
    pub zknh: bool,
    pub zkr: bool,
    pub zks: bool,
    pub zksed: bool,
    pub zksh: bool,
    pub zkt: bool,
    pub zvbc32e: bool,
    pub zvkb: bool,
    pub zvkg: bool,
    pub zvkgs: bool,
    pub zvkn: bool,
    pub zvknc: bool,
    pub zvkned: bool,
    pub zvknf: bool,
    pub zvkng: bool,
    pub zvknha: bool,
    pub zvknhb: bool,
    pub zvks: bool,
    pub zvksc: bool,
    pub zvksed: bool,
    pub zvksg: bool,
    pub zvksh: bool,
    pub zvkt: bool,
    pub zicntr: bool,
    pub zicsr: bool,
    pub zifencei: bool,
    pub zihintntl: bool,
    pub zihintpause: bool,
    pub zihpm: bool,
    pub zimop: bool,
    pub zimt: bool,
    pub zjid: bool,
    pub zjpm: bool,
    pub ztso: bool,
    pub zic64b: bool,
    pub zicbom: bool,
    pub zicbop: bool,
    pub zicboz: bool,
    pub ziccamoa: bool,
    pub ziccamoc: bool,
    pub ziccid: bool,
    pub ziccif: bool,
    pub smcdeleg: bool,
    pub smcntrpmf: bool,
    pub ssccfg: bool,
    pub sscounterenw: bool,
    pub smaia: bool,
    pub ssaia: bool,
    pub smclic: bool,
    pub smclicconfig: bool,
    pub smclicshv: bool,
    pub ssclic: bool,
    pub cuclic: bool,
    pub sstc: bool,
    pub sscofpmf: bool,
    pub shlcofideleg: bool,
    pub ssqosid: bool,
    pub smrnmi: bool,
    pub smsdia: bool,
    pub sv32: bool,
    pub sv39: bool,
    pub sv48: bool,
    pub sv57: bool,
    pub svbare: bool,
    pub svpbmt: bool,
    pub svnapot: bool,
    pub svinval: bool,
    pub svade: bool,
    pub svadu: bool,
    pub svvptc: bool,
    pub svukte: bool,
    pub supm: bool,
    pub ssnpm: bool,
    pub sspm: bool,
    pub ssccptr: bool,
    pub smvatag: bool,
    pub smepmp: bool,
    pub smmpm: bool,
    pub smnpm: bool,
    pub smpmpmt: bool,
    pub sspmp: bool,
    pub sspmpen: bool,
    pub sspmpdeleg: bool,
    pub sdext: bool,
    pub sdtrig: bool,
    pub sdtrigepm: bool,
    pub sdtrigpend: bool,
    pub smcsrind: bool,
    pub sscsrind: bool,
    pub smctr: bool,
    pub ssctr: bool,
    pub sddbltrp: bool,
    pub ssdbltrp: bool,
    pub smdbltrp: bool,
    pub smstateen: bool,
    pub ssstateen: bool,
    pub sm1p11: bool,
    pub ss1p11: bool,
    pub sm1p12: bool,
    pub ss1p12: bool,
    pub sm1p13: bool,
    pub ss1p13: bool,
    pub sstvala: bool,
    pub sstvecd: bool,
    pub sstvecv: bool,
    pub ssdtso: bool,
    pub sstcfg: bool,
    pub ssstrict: bool,
    pub ssu32xl: bool,
    pub ssu64xl: bool,
    pub ssube: bool,
    pub smtdeleg: bool,
    pub reri: bool,
    pub hti: bool,
    pub sha: bool,
    pub shcounterenw: bool,
    pub shgatpa: bool,
    pub shtvala: bool,
    pub shvsatpa: bool,
    pub shvstvala: bool,
    pub shvstvecd: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub num_harts: usize,
    pub starting_pc: u64,
    pub memory_len: u64,
    pub base_isa: BaseExtension
}

impl Default for Config {
    fn default() -> Self {
        Config {
            num_harts: NUM_HARTS_DEFAULT,
            starting_pc: STARTING_PC_DEFAULT,
            memory_len: MEMORY_LENGTH_DEFAULT,
            base_isa: BASE_ISA_DEFAULT
        }
    }
}

impl BaseExtension {
    fn integer_register_file_reduced(&self) -> bool {
        match self {
            BaseExtension::Rv32e => true,
            BaseExtension::Rv64e => true,
            _ => false
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }

    pub fn with_num_harts(mut self, num_harts: usize) -> Self {
        self.num_harts = num_harts;
        self
    }

    pub fn with_starting_pc(mut self, starting_pc: u64) -> Self {
        self.starting_pc = starting_pc;
        self
    }

    pub fn with_memory_len(mut self, memory_len: u64) -> Self {
        self.memory_len = memory_len;
        self
    }

    pub fn with_base_isa(mut self, base_isa: BaseExtension) -> Self {
        self.base_isa = base_isa;
        self
    }
}

impl EnabledExtensions {
    pub fn new() -> EnabledExtensions {
        EnabledExtensions::default()
    }
}
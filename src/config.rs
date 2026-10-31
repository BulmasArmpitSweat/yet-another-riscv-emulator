pub const XLEN_DEFAULT: u8 = 32;
pub const FLEN_DEFAULT: u8 = 0; // Old value: 32
pub const VLEN_DEFAULT: usize = 0; // Old value: 128
pub const NUM_HARTS_DEFAULT: usize = 1;
pub const STARTING_PC_DEFAULT: u64 = 0x80000000;
pub const MEMORY_LENGTH_DEFAULT: u64 = STARTING_PC_DEFAULT;

#[derive(Debug, Default)]
pub struct EnabledExtensions {
    pub a: bool,
    pub b: bool,
    pub c: bool,
    pub d: bool,
    pub e: bool,
    pub f: bool,
    pub g: bool,
    pub h: bool,
    pub j: bool,
    pub l: bool,
    pub m: bool,
    pub p: bool,
    pub q: bool,
    pub s: bool,
    pub t: bool,
    pub v: bool,
    pub wmo: bool, // Weak Memory Ordering
    pub zicsr: bool,
    pub zfencei: bool,
    pub zam: bool,
    pub zihintpause: bool,
    pub zihintntl: bool,
    pub zfa: bool,
    pub zfh: bool,
    pub zfhmin: bool,
    pub zfinx: bool,
    pub zdinx: bool,
    pub zhinx: bool,
    pub zhinxmin: bool,
    pub zmmul: bool,
    pub ztso: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub xlen: u8,
    pub flen: u8,
    pub vlen: usize, // vlen can be an arbitrary size, much larger than xlen or flen
    pub num_harts: usize,
    pub starting_pc: u64,
    pub memory_len: u64
}

impl Default for Config {
    fn default() -> Self {
        Config {
            xlen: XLEN_DEFAULT,
            flen: FLEN_DEFAULT,
            vlen: VLEN_DEFAULT,
            num_harts: NUM_HARTS_DEFAULT,
            starting_pc: STARTING_PC_DEFAULT,
            memory_len: MEMORY_LENGTH_DEFAULT
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }

    pub fn with_xlen(mut self, xlen: u8) -> Self {
        self.xlen = xlen;
        self
    }

    pub fn with_flen(mut self, flen: u8) -> Self {
        self.flen = flen;
        self
    }

    pub fn with_vlen(mut self, vlen: usize) -> Self {
        self.vlen = vlen;
        self
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
}

impl EnabledExtensions {
    pub fn new() -> EnabledExtensions {
        EnabledExtensions::default()
    }

    fn enable_a(mut self) -> Self {
        self.a = true;

        self
    }

    fn enable_b(mut self) -> Self {
        self.b = true;

        self
    }

    fn enable_c(mut self) -> Self {
        self.c = true;

        self
    }

    fn enable_d(mut self) -> Self {
        self.d = true;
        self = self.enable_f(); // D implies F

        self
    }

    fn enable_f(mut self) -> Self {
        self.f = true;
        self = self.enable_zicsr(); // F implies Zicsr

        self
    }

    fn enable_g(mut self) -> Self {
        self.g = true;
        self = self.enable_a()
            .enable_f()
            .enable_d()
            .enable_m()
            .enable_zicsr()
            .enable_zfencei(); // G implies A, F, D, M, Zicsr, and Zfencei

        self
    }

    fn enable_h(mut self) -> Self {
        self.h = true;

        self
    }

    fn enable_j(mut self) -> Self {
        self.j = true;

        self
    }

    fn enable_l(mut self) -> Self {
        self.l = true;

        self
    }

    fn enable_m(mut self) -> Self {
        self.m = true;
        self = self.enable_zmmul(); // M implies Zmmul

        self
    }

    fn enable_p(mut self) -> Self {
        self.p = true;

        self
    }

    fn enable_q(mut self) -> Self {
        self.q = true;
        self = self.enable_d(); // Q implies D, which implies F and Zicsr

        self
    }

    fn enable_s(mut self) -> Self {
        self.s = true;

        self
    }

    fn enable_t(mut self) -> Self {
        self.t = true;

        self
    }

    fn enable_v(mut self) -> Self {
        self.v = true;
        self = self.enable_d(); // V implies D, which implies F and Zicsr

        self
    }

    fn enable_wmo(mut self) -> Self {
        self.wmo = true;

        self
    }

    fn enable_zicsr(mut self) -> Self {
        self.zicsr = true;

        self
    }

    fn enable_zfencei(mut self) -> Self {
        self.zfencei = true;

        self
    }

    fn enable_zam(mut self) -> Self {
        self.zam = true;

        self
    }

    fn enable_zihintpause(mut self) -> Self {
        self.zihintpause = true;

        self
    }

    fn enable_zihintntl(mut self) -> Self {
        self.zihintntl = true;

        self
    }

    fn enable_zfa(mut self) -> Self {
        self.zfa = true;

        self
    }

    fn enable_zfh(mut self) -> Self {
        self.zfh = true;

        self
    }

    fn enable_zfhmin(mut self) -> Self {
        self.zfhmin = true;

        self
    }

    fn enable_zfinx(mut self) -> Self {
        self.zfinx = true;

        self
    }

    fn enable_zdinx(mut self) -> Self {
        self.zdinx = true;

        self
    }

    fn enable_zhinx(mut self) -> Self {
        self.zhinx = true;

        self
    }

    fn enable_zhinxmin(mut self) -> Self {
        self.zhinxmin = true;

        self
    }

    fn enable_zmmul(mut self) -> Self {
        self.zmmul = true;

        self
    }

    fn enable_ztso(mut self) -> Self {
        self.ztso = true;

        self
    }
}
pub const XLEN_DEFAULT: u8 = 32;
pub const FLEN_DEFAULT: u8 = 0; // Old value: 32
pub const VLEN_DEFAULT: usize = 0; // Old value: 128
pub const NUM_HARTS_DEFAULT: usize = 1;
pub const STARTING_PC_DEFAULT: u64 = 0x80000000;
pub const MEMORY_LENGTH_DEFAULT: u64 = STARTING_PC_DEFAULT;

#[derive(Debug, Default)]
pub struct EnabledExtensions {
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    f: bool,
    g: bool,
    h: bool,
    j: bool,
    l: bool,
    m: bool,
    n: bool,
    p: bool,
    q: bool,
    s: bool,
    t: bool,
    v: bool,
    zfencei: bool,
    zicsr: bool,
    zicntr: bool,
    zihpm: bool,
    zihintntl: bool,
    zihintpause: bool,
    zimop: bool,
    zicond: bool,
    zawrs: bool, // rawr
    zacas: bool,
    zabha: bool,
    zalasr: bool,
    ztso: bool,
    zfh: bool,
    zfhmin: bool,
    zfa: bool,
    zfinx: bool,
    zdinx: bool,
    zhinx: bool,
    zca: bool,
    zcd: bool,
    zcf: bool,
    zilsd: bool,
    zclsd: bool
}

#[derive(Debug)]
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

    pub fn enable_a(mut self) -> Self {
        self.a = true;
        self.zabha = true;
        self.zacas = true;
        self.zalasr = true;
        self.zawrs = true;

        self
    }

    pub fn enable_b(mut self) -> Self {
        self.b = true;

        self
    }

    pub fn enable_c(mut self) -> Self {
        self.c = true;
        self.zca = true;
        self.zcd = true;
        self.zcf = true;
        self.zclsd = true;

        self
    }

    pub fn enable_d(mut self) -> Self {
        self.d = true;
        self.zdinx = true;

        self
    }

    pub fn enable_f(mut self) -> Self {
        self.f = true;
        self.zfa = true;
        self.zfencei = true;
        self.zfh = true;
        self.zfhmin = true;
        self.zfinx = true;

        self
    }

    pub fn enable_g(mut self) -> Self {
        self.g = true;

        self
    }

    pub fn enable_h(mut self) -> Self {
        self.h = true;
        self.zhinx = true;

        self
    }

    pub fn enable_j(mut self) -> Self {
        self.j = true;

        self
    }

    pub fn enable_l(mut self) -> Self {
        self.l = true;

        self
    }

    pub fn enable_m(mut self) -> Self {
        self.m = true;

        self
    }

    pub fn enable_n(mut self) -> Self {
        self.n = true;

        self
    }

    pub fn enable_p(mut self) -> Self {
        self.p = true;

        self
    }

    pub fn enable_q(mut self) -> Self {
        self.q = true;

        self
    }

    pub fn enable_s(mut self) -> Self {
        self.s = true;

        self
    }

    pub fn enable_t(mut self) -> Self {
        self.t = true;
        self.ztso = true;

        self
    }

    pub fn enable_v(mut self) -> Self {
        self.v = true;

        self
    }

    pub fn enable_zfencei(mut self) -> Self {
        self.zfencei = true;

        self
    }

    pub fn enable_zicsr(mut self) -> Self {
        self.zicsr = true;

        self
    }

    pub fn enable_zicntr(mut self) -> Self {
        self.zicntr = true;

        self
    }

    pub fn enable_zihpm(mut self) -> Self {
        self.zihpm = true;

        self
    }

    pub fn enable_zihintntl(mut self) -> Self {
        self.zihintntl = true;

        self
    }

    pub fn enable_zihintpause(mut self) -> Self {
        self.zihintpause = true;

        self
    }

    pub fn enable_zimop(mut self) -> Self {
        self.zimop = true;

        self
    }

    pub fn enable_zicond(mut self) -> Self {
        self.zicond = true;

        self
    }

    pub fn enable_zawrs(mut self) -> Self {
        self.zawrs = true;

        self
    }

    pub fn enable_zacas(mut self) -> Self {
        self.zacas = true;

        self
    }

    pub fn enable_zabha(mut self) -> Self {
        self.zabha = true;

        self
    }

    pub fn enable_zalasr(mut self) -> Self {
        self.zalasr = true;

        self
    }

    pub fn enable_ztso(mut self) -> Self {
        self.ztso = true;

        self
    }

    pub fn enable_zfh(mut self) -> Self {
        self.zfh = true;

        self
    }

    pub fn enable_zfhmin(mut self) -> Self {
        self.zfhmin = true;

        self
    }

    pub fn enable_zfa(mut self) -> Self {
        self.zfa = true;

        self
    }

    pub fn enable_zfinx(mut self) -> Self {
        self.zfinx = true;

        self
    }

    pub fn enable_zdinx(mut self) -> Self {
        self.zdinx = true;

        self
    }

    pub fn enable_zhinx(mut self) -> Self {
        self.zhinx = true;

        self
    }

    pub fn enable_zca(mut self) -> Self {
        self.zca = true;

        self
    }

    pub fn enable_zcd(mut self) -> Self {
        self.zcd = true;

        self
    }

    pub fn enable_zcf(mut self) -> Self {
        self.zcf = true;

        self
    }

    pub fn enable_zilsd(mut self) -> Self {
        self.zilsd = true;

        self
    }

    pub fn enable_zclsd(mut self) -> Self {
        self.zclsd = true;

        self
    }
}
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Input Selector Configuration"]
    pub ins: INS,
    #[doc = "0x04 - Connection Matrix Control"]
    pub cmc: CMC,
    #[doc = "0x08 - Slice Timer Status"]
    pub tcst: TCST,
    #[doc = "0x0c - Slice Timer Run Set"]
    pub tcset: TCSET,
    #[doc = "0x10 - Slice Timer Clear"]
    pub tcclr: TCCLR,
    #[doc = "0x14 - Slice Timer Control"]
    pub tc: TC,
    #[doc = "0x18 - Passive Level Config"]
    pub psl: PSL,
    #[doc = "0x1c - Dither Config"]
    pub dit: DIT,
    #[doc = "0x20 - Dither Shadow Register"]
    pub dits: DITS,
    #[doc = "0x24 - Prescaler Control"]
    pub psc: PSC,
    #[doc = "0x28 - Floating Prescaler Control"]
    pub fpc: FPC,
    #[doc = "0x2c - Floating Prescaler Shadow"]
    pub fpcs: FPCS,
    #[doc = "0x30 - Timer Period Value"]
    pub pr: PR,
    #[doc = "0x34 - Timer Shadow Period Value"]
    pub prs: PRS,
    #[doc = "0x38 - Timer Compare Value"]
    pub cr: CR,
    #[doc = "0x3c - Timer Shadow Compare Value"]
    pub crs: CRS,
    _reserved0: [u8; 48usize],
    #[doc = "0x70 - Timer Value"]
    pub timer: TIMER,
    #[doc = "0x74 - Capture Register 0"]
    pub c0v: C0V,
    #[doc = "0x78 - Capture Register 1"]
    pub c1v: C1V,
    #[doc = "0x7c - Capture Register 2"]
    pub c2v: C2V,
    #[doc = "0x80 - Capture Register 3"]
    pub c3v: C3V,
    _reserved1: [u8; 28usize],
    #[doc = "0xa0 - Interrupt Status"]
    pub ints: INTS,
    #[doc = "0xa4 - Interrupt Enable Control"]
    pub inte: INTE,
    #[doc = "0xa8 - Service Request Selector"]
    pub srs: SRS,
    #[doc = "0xac - Interrupt Status Set"]
    pub sws: SWS,
    #[doc = "0xb0 - Interrupt Status Clear"]
    pub swr: SWR,
    _reserved2: [u8; 4usize],
    #[doc = "0xb8 - Extended Read Back 0"]
    pub ecrd0: ECRD0,
    #[doc = "0xbc - Extended Read Back 1"]
    pub ecrd1: ECRD1,
}
#[doc = "Input Selector Configuration"]
pub struct INS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Selector Configuration"]
pub mod ins;
#[doc = "Connection Matrix Control"]
pub struct CMC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Connection Matrix Control"]
pub mod cmc;
#[doc = "Slice Timer Status"]
pub struct TCST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slice Timer Status"]
pub mod tcst;
#[doc = "Slice Timer Run Set"]
pub struct TCSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slice Timer Run Set"]
pub mod tcset;
#[doc = "Slice Timer Clear"]
pub struct TCCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slice Timer Clear"]
pub mod tcclr;
#[doc = "Slice Timer Control"]
pub struct TC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slice Timer Control"]
pub mod tc;
#[doc = "Passive Level Config"]
pub struct PSL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Passive Level Config"]
pub mod psl;
#[doc = "Dither Config"]
pub struct DIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Dither Config"]
pub mod dit;
#[doc = "Dither Shadow Register"]
pub struct DITS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Dither Shadow Register"]
pub mod dits;
#[doc = "Prescaler Control"]
pub struct PSC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Prescaler Control"]
pub mod psc;
#[doc = "Floating Prescaler Control"]
pub struct FPC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Floating Prescaler Control"]
pub mod fpc;
#[doc = "Floating Prescaler Shadow"]
pub struct FPCS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Floating Prescaler Shadow"]
pub mod fpcs;
#[doc = "Timer Period Value"]
pub struct PR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Period Value"]
pub mod pr;
#[doc = "Timer Shadow Period Value"]
pub struct PRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Shadow Period Value"]
pub mod prs;
#[doc = "Timer Compare Value"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Compare Value"]
pub mod cr;
#[doc = "Timer Shadow Compare Value"]
pub struct CRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Shadow Compare Value"]
pub mod crs;
#[doc = "Timer Value"]
pub struct TIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Value"]
pub mod timer;
#[doc = "Capture Register 0"]
pub struct C0V {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture Register 0"]
pub mod c0v;
#[doc = "Capture Register 1"]
pub struct C1V {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture Register 1"]
pub mod c1v;
#[doc = "Capture Register 2"]
pub struct C2V {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture Register 2"]
pub mod c2v;
#[doc = "Capture Register 3"]
pub struct C3V {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture Register 3"]
pub mod c3v;
#[doc = "Interrupt Status"]
pub struct INTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status"]
pub mod ints;
#[doc = "Interrupt Enable Control"]
pub struct INTE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Control"]
pub mod inte;
#[doc = "Service Request Selector"]
pub struct SRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Service Request Selector"]
pub mod srs;
#[doc = "Interrupt Status Set"]
pub struct SWS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Set"]
pub mod sws;
#[doc = "Interrupt Status Clear"]
pub struct SWR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Clear"]
pub mod swr;
#[doc = "Extended Read Back 0"]
pub struct ECRD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Extended Read Back 0"]
pub mod ecrd0;
#[doc = "Extended Read Back 1"]
pub struct ECRD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Extended Read Back 1"]
pub mod ecrd1;

#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 128usize],
    #[doc = "0x80 - Arbitration Configuration Register"]
    pub arbcfg: ARBCFG,
    #[doc = "0x84 - Arbitration Priority Register"]
    pub arbpr: ARBPR,
    #[doc = "0x88 - Channel Assignment Register"]
    pub chass: CHASS,
    _reserved1: [u8; 20usize],
    #[doc = "0xa0 - Input Class Register"]
    pub iclass: [ICLASS; 2],
    _reserved2: [u8; 8usize],
    #[doc = "0xb0 - Alias Register"]
    pub alias: ALIAS,
    _reserved3: [u8; 4usize],
    #[doc = "0xb8 - Boundary Select Register"]
    pub bound: BOUND,
    _reserved4: [u8; 4usize],
    #[doc = "0xc0 - Synchronization Control Register"]
    pub synctr: SYNCTR,
    _reserved5: [u8; 4usize],
    #[doc = "0xc8 - Boundary Flag Register"]
    pub bfl: BFL,
    #[doc = "0xcc - Boundary Flag Software Register"]
    pub bfls: BFLS,
    #[doc = "0xd0 - Boundary Flag Control Register"]
    pub bflc: BFLC,
    #[doc = "0xd4 - Boundary Flag Node Pointer Register"]
    pub bflnp: BFLNP,
    _reserved6: [u8; 40usize],
    #[doc = "0x100 - Queue 0 Source Control Register"]
    pub qctrl0: QCTRL0,
    #[doc = "0x104 - Queue 0 Mode Register"]
    pub qmr0: QMR0,
    #[doc = "0x108 - Queue 0 Status Register"]
    pub qsr0: QSR0,
    #[doc = "0x10c - Queue 0 Register 0"]
    pub q0r0: Q0R0,
    #[doc = "0x110 - Queue 0 Input Register"]
    pub qinr0: QINR0,
    _reserved7: [u8; 12usize],
    #[doc = "0x120 - Autoscan Source Control Register"]
    pub asctrl: ASCTRL,
    #[doc = "0x124 - Autoscan Source Mode Register"]
    pub asmr: ASMR,
    #[doc = "0x128 - Autoscan Source Channel Select Register"]
    pub assel: ASSEL,
    #[doc = "0x12c - Autoscan Source Pending Register"]
    pub aspnd: ASPND,
    _reserved8: [u8; 80usize],
    #[doc = "0x180 - Channel Event Flag Register"]
    pub ceflag: CEFLAG,
    #[doc = "0x184 - Result Event Flag Register"]
    pub reflag: REFLAG,
    #[doc = "0x188 - Source Event Flag Register"]
    pub seflag: SEFLAG,
    _reserved9: [u8; 4usize],
    #[doc = "0x190 - Channel Event Flag Clear Register"]
    pub cefclr: CEFCLR,
    #[doc = "0x194 - Result Event Flag Clear Register"]
    pub refclr: REFCLR,
    #[doc = "0x198 - Source Event Flag Clear Register"]
    pub sefclr: SEFCLR,
    _reserved10: [u8; 4usize],
    #[doc = "0x1a0 - Channel Event Node Pointer Register 0"]
    pub cevnp0: CEVNP0,
    _reserved11: [u8; 12usize],
    #[doc = "0x1b0 - Result Event Node Pointer Register 0"]
    pub revnp0: REVNP0,
    #[doc = "0x1b4 - Result Event Node Pointer Register 1"]
    pub revnp1: REVNP1,
    _reserved12: [u8; 8usize],
    #[doc = "0x1c0 - Source Event Node Pointer Register"]
    pub sevnp: SEVNP,
    _reserved13: [u8; 4usize],
    #[doc = "0x1c8 - Service Request Software Activation Trigger"]
    pub sract: SRACT,
    _reserved14: [u8; 36usize],
    #[doc = "0x1f0 - E0ternal Multiplexer Control Register"]
    pub emuxctr: EMUXCTR,
    _reserved15: [u8; 4usize],
    #[doc = "0x1f8 - Valid Flag Register"]
    pub vfr: VFR,
    _reserved16: [u8; 4usize],
    #[doc = "0x200 - Channel Ctrl. Reg."]
    pub chctr: [CHCTR; 8],
    _reserved17: [u8; 96usize],
    #[doc = "0x280 - Result Control Register"]
    pub rcr: [RCR; 16],
    _reserved18: [u8; 64usize],
    #[doc = "0x300 - Result Register"]
    pub res: [RES; 16],
    _reserved19: [u8; 64usize],
    #[doc = "0x380 - Result Register, Debug"]
    pub resd: [RESD; 16],
}
#[doc = "Arbitration Configuration Register"]
pub struct ARBCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Arbitration Configuration Register"]
pub mod arbcfg;
#[doc = "Arbitration Priority Register"]
pub struct ARBPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Arbitration Priority Register"]
pub mod arbpr;
#[doc = "Channel Assignment Register"]
pub struct CHASS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Assignment Register"]
pub mod chass;
#[doc = "Input Class Register"]
pub struct ICLASS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Class Register"]
pub mod iclass;
#[doc = "Alias Register"]
pub struct ALIAS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alias Register"]
pub mod alias;
#[doc = "Boundary Select Register"]
pub struct BOUND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Boundary Select Register"]
pub mod bound;
#[doc = "Synchronization Control Register"]
pub struct SYNCTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronization Control Register"]
pub mod synctr;
#[doc = "Boundary Flag Register"]
pub struct BFL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Boundary Flag Register"]
pub mod bfl;
#[doc = "Boundary Flag Software Register"]
pub struct BFLS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Boundary Flag Software Register"]
pub mod bfls;
#[doc = "Boundary Flag Control Register"]
pub struct BFLC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Boundary Flag Control Register"]
pub mod bflc;
#[doc = "Boundary Flag Node Pointer Register"]
pub struct BFLNP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Boundary Flag Node Pointer Register"]
pub mod bflnp;
#[doc = "Queue 0 Source Control Register"]
pub struct QCTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Queue 0 Source Control Register"]
pub mod qctrl0;
#[doc = "Queue 0 Mode Register"]
pub struct QMR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Queue 0 Mode Register"]
pub mod qmr0;
#[doc = "Queue 0 Status Register"]
pub struct QSR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Queue 0 Status Register"]
pub mod qsr0;
#[doc = "Queue 0 Register 0"]
pub struct Q0R0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Queue 0 Register 0"]
pub mod q0r0;
#[doc = "Queue 0 Input Register"]
pub struct QINR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Queue 0 Input Register"]
pub mod qinr0;
#[doc = "Queue 0 Backup Register"]
pub struct QBUR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Queue 0 Backup Register"]
pub mod qbur0;
#[doc = "Autoscan Source Control Register"]
pub struct ASCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Autoscan Source Control Register"]
pub mod asctrl;
#[doc = "Autoscan Source Mode Register"]
pub struct ASMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Autoscan Source Mode Register"]
pub mod asmr;
#[doc = "Autoscan Source Channel Select Register"]
pub struct ASSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Autoscan Source Channel Select Register"]
pub mod assel;
#[doc = "Autoscan Source Pending Register"]
pub struct ASPND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Autoscan Source Pending Register"]
pub mod aspnd;
#[doc = "Channel Event Flag Register"]
pub struct CEFLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Event Flag Register"]
pub mod ceflag;
#[doc = "Result Event Flag Register"]
pub struct REFLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Result Event Flag Register"]
pub mod reflag;
#[doc = "Source Event Flag Register"]
pub struct SEFLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Source Event Flag Register"]
pub mod seflag;
#[doc = "Channel Event Flag Clear Register"]
pub struct CEFCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Event Flag Clear Register"]
pub mod cefclr;
#[doc = "Result Event Flag Clear Register"]
pub struct REFCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Result Event Flag Clear Register"]
pub mod refclr;
#[doc = "Source Event Flag Clear Register"]
pub struct SEFCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Source Event Flag Clear Register"]
pub mod sefclr;
#[doc = "Channel Event Node Pointer Register 0"]
pub struct CEVNP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Event Node Pointer Register 0"]
pub mod cevnp0;
#[doc = "Result Event Node Pointer Register 0"]
pub struct REVNP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Result Event Node Pointer Register 0"]
pub mod revnp0;
#[doc = "Result Event Node Pointer Register 1"]
pub struct REVNP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Result Event Node Pointer Register 1"]
pub mod revnp1;
#[doc = "Source Event Node Pointer Register"]
pub struct SEVNP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Source Event Node Pointer Register"]
pub mod sevnp;
#[doc = "Service Request Software Activation Trigger"]
pub struct SRACT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Service Request Software Activation Trigger"]
pub mod sract;
#[doc = "E0ternal Multiplexer Control Register"]
pub struct EMUXCTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "E0ternal Multiplexer Control Register"]
pub mod emuxctr;
#[doc = "Valid Flag Register"]
pub struct VFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Valid Flag Register"]
pub mod vfr;
#[doc = "Channel Ctrl. Reg."]
pub struct CHCTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Ctrl. Reg."]
pub mod chctr;
#[doc = "Result Control Register"]
pub struct RCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Result Control Register"]
pub mod rcr;
#[doc = "Result Register"]
pub struct RES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Result Register"]
pub mod res;
#[doc = "Result Register, Debug"]
pub struct RESD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Result Register, Debug"]
pub mod resd;

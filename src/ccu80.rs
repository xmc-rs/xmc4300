#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global Control Register"]
    pub gctrl: GCTRL,
    #[doc = "0x04 - Global Status Register"]
    pub gstat: GSTAT,
    #[doc = "0x08 - Global Idle Set"]
    pub gidls: GIDLS,
    #[doc = "0x0c - Global Idle Clear"]
    pub gidlc: GIDLC,
    #[doc = "0x10 - Global Channel Set"]
    pub gcss: GCSS,
    #[doc = "0x14 - Global Channel Clear"]
    pub gcsc: GCSC,
    #[doc = "0x18 - Global Channel status"]
    pub gcst: GCST,
    #[doc = "0x1c - Parity Checker Configuration"]
    pub gpchk: GPCHK,
    _reserved0: [u8; 96usize],
    #[doc = "0x80 - Module Identification"]
    pub midr: MIDR,
}
#[doc = "Global Control Register"]
pub struct GCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Control Register"]
pub mod gctrl;
#[doc = "Global Status Register"]
pub struct GSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Status Register"]
pub mod gstat;
#[doc = "Global Idle Set"]
pub struct GIDLS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Idle Set"]
pub mod gidls;
#[doc = "Global Idle Clear"]
pub struct GIDLC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Idle Clear"]
pub mod gidlc;
#[doc = "Global Channel Set"]
pub struct GCSS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Channel Set"]
pub mod gcss;
#[doc = "Global Channel Clear"]
pub struct GCSC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Channel Clear"]
pub mod gcsc;
#[doc = "Global Channel status"]
pub struct GCST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Channel status"]
pub mod gcst;
#[doc = "Parity Checker Configuration"]
pub struct GPCHK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parity Checker Configuration"]
pub mod gpchk;
#[doc = "Module Identification"]
pub struct MIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Identification"]
pub mod midr;

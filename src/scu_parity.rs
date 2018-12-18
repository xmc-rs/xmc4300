#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Parity Error Enable Register"]
    pub peen: PEEN,
    #[doc = "0x04 - Memory Checking Control Register"]
    pub mchkcon: MCHKCON,
    #[doc = "0x08 - Parity Error Trap Enable Register"]
    pub pete: PETE,
    #[doc = "0x0c - Parity Error Reset Enable Register"]
    pub persten: PERSTEN,
    _reserved0: [u8; 4usize],
    #[doc = "0x14 - Parity Error Flag Register"]
    pub peflag: PEFLAG,
    #[doc = "0x18 - Parity Memory Test Pattern Register"]
    pub pmtpr: PMTPR,
    #[doc = "0x1c - Parity Memory Test Select Register"]
    pub pmtsr: PMTSR,
}
#[doc = "Parity Error Enable Register"]
pub struct PEEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parity Error Enable Register"]
pub mod peen;
#[doc = "Memory Checking Control Register"]
pub struct MCHKCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Checking Control Register"]
pub mod mchkcon;
#[doc = "Parity Error Trap Enable Register"]
pub struct PETE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parity Error Trap Enable Register"]
pub mod pete;
#[doc = "Parity Error Reset Enable Register"]
pub struct PERSTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parity Error Reset Enable Register"]
pub mod persten;
#[doc = "Parity Error Flag Register"]
pub struct PEFLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parity Error Flag Register"]
pub mod peflag;
#[doc = "Parity Memory Test Pattern Register"]
pub struct PMTPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parity Memory Test Pattern Register"]
pub mod pmtpr;
#[doc = "Parity Memory Test Select Register"]
pub struct PMTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parity Memory Test Select Register"]
pub mod pmtsr;

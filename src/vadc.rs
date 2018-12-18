#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Control Register"]
    pub clc: CLC,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - Module Identification Register"]
    pub id: ID,
    _reserved1: [u8; 28usize],
    #[doc = "0x28 - OCDS Control and Status Register"]
    pub ocs: OCS,
    _reserved2: [u8; 84usize],
    #[doc = "0x80 - Global Configuration Register"]
    pub globcfg: GLOBCFG,
    _reserved3: [u8; 28usize],
    #[doc = "0xa0 - Input Class Register, Global"]
    pub globiclass: [GLOBICLASS; 2],
    _reserved4: [u8; 16usize],
    #[doc = "0xb8 - Global Boundary Select Register"]
    pub globbound: GLOBBOUND,
    _reserved5: [u8; 36usize],
    #[doc = "0xe0 - Global Event Flag Register"]
    pub globeflag: GLOBEFLAG,
    _reserved6: [u8; 92usize],
    #[doc = "0x140 - Global Event Node Pointer Register"]
    pub globevnp: GLOBEVNP,
    _reserved7: [u8; 28usize],
    #[doc = "0x160 - Global Test Functions Register"]
    pub globtf: GLOBTF,
    _reserved8: [u8; 28usize],
    #[doc = "0x180 - Background Request Source Channel Select Register"]
    pub brssel: [BRSSEL; 2],
    _reserved9: [u8; 56usize],
    #[doc = "0x1c0 - Background Request Source Pending Register"]
    pub brspnd: [BRSPND; 2],
    _reserved10: [u8; 56usize],
    #[doc = "0x200 - Background Request Source Control Register"]
    pub brsctrl: BRSCTRL,
    #[doc = "0x204 - Background Request Source Mode Register"]
    pub brsmr: BRSMR,
    _reserved11: [u8; 120usize],
    #[doc = "0x280 - Global Result Control Register"]
    pub globrcr: GLOBRCR,
    _reserved12: [u8; 124usize],
    #[doc = "0x300 - Global Result Register"]
    pub globres: GLOBRES,
    _reserved13: [u8; 124usize],
    #[doc = "0x380 - Global Result Register, Debug"]
    pub globresd: GLOBRESD,
    _reserved14: [u8; 108usize],
    #[doc = "0x3f0 - External Multiplexer Select Register"]
    pub emuxsel: EMUXSEL,
}
#[doc = "Clock Control Register"]
pub struct CLC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Control Register"]
pub mod clc;
#[doc = "Module Identification Register"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Identification Register"]
pub mod id;
#[doc = "OCDS Control and Status Register"]
pub struct OCS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OCDS Control and Status Register"]
pub mod ocs;
#[doc = "Global Configuration Register"]
pub struct GLOBCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Configuration Register"]
pub mod globcfg;
#[doc = "Input Class Register, Global"]
pub struct GLOBICLASS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Class Register, Global"]
pub mod globiclass;
#[doc = "Global Boundary Select Register"]
pub struct GLOBBOUND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Boundary Select Register"]
pub mod globbound;
#[doc = "Global Event Flag Register"]
pub struct GLOBEFLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Event Flag Register"]
pub mod globeflag;
#[doc = "Global Event Node Pointer Register"]
pub struct GLOBEVNP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Event Node Pointer Register"]
pub mod globevnp;
#[doc = "Global Test Functions Register"]
pub struct GLOBTF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Test Functions Register"]
pub mod globtf;
#[doc = "Background Request Source Channel Select Register"]
pub struct BRSSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Background Request Source Channel Select Register"]
pub mod brssel;
#[doc = "Background Request Source Pending Register"]
pub struct BRSPND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Background Request Source Pending Register"]
pub mod brspnd;
#[doc = "Background Request Source Control Register"]
pub struct BRSCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Background Request Source Control Register"]
pub mod brsctrl;
#[doc = "Background Request Source Mode Register"]
pub struct BRSMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Background Request Source Mode Register"]
pub mod brsmr;
#[doc = "Global Result Control Register"]
pub struct GLOBRCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Result Control Register"]
pub mod globrcr;
#[doc = "Global Result Register"]
pub struct GLOBRES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Result Register"]
pub mod globres;
#[doc = "Global Result Register, Debug"]
pub struct GLOBRESD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Result Register, Debug"]
pub mod globresd;
#[doc = "External Multiplexer Select Register"]
pub struct EMUXSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Multiplexer Select Register"]
pub mod emuxsel;

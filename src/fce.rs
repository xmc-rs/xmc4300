#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Control Register"]
    pub clc: CLC,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - Module Identification Register"]
    pub id: ID,
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

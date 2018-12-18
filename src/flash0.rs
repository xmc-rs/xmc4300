#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4104usize],
    #[doc = "0x1008 - Flash Module Identification Register"]
    pub id: ID,
    _reserved1: [u8; 4usize],
    #[doc = "0x1010 - Flash Status Register"]
    pub fsr: FSR,
    #[doc = "0x1014 - Flash Configuration Register"]
    pub fcon: FCON,
    #[doc = "0x1018 - Margin Control Register PFLASH"]
    pub marp: MARP,
    _reserved2: [u8; 4usize],
    #[doc = "0x1020 - Flash Protection Configuration Register User 0"]
    pub procon0: PROCON0,
    #[doc = "0x1024 - Flash Protection Configuration Register User 1"]
    pub procon1: PROCON1,
    #[doc = "0x1028 - Flash Protection Configuration Register User 2"]
    pub procon2: PROCON2,
}
#[doc = "Flash Module Identification Register"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Module Identification Register"]
pub mod id;
#[doc = "Flash Status Register"]
pub struct FSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Status Register"]
pub mod fsr;
#[doc = "Flash Configuration Register"]
pub struct FCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Configuration Register"]
pub mod fcon;
#[doc = "Margin Control Register PFLASH"]
pub struct MARP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Margin Control Register PFLASH"]
pub mod marp;
#[doc = "Flash Protection Configuration Register User 0"]
pub struct PROCON0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Protection Configuration Register User 0"]
pub mod procon0;
#[doc = "Flash Protection Configuration Register User 1"]
pub struct PROCON1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Protection Configuration Register User 1"]
pub mod procon1;
#[doc = "Flash Protection Configuration Register User 2"]
pub struct PROCON2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Protection Configuration Register User 2"]
pub mod procon2;

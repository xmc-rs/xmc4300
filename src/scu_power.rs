#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PCU Status Register"]
    pub pwrstat: PWRSTAT,
    #[doc = "0x04 - PCU Set Control Register"]
    pub pwrset: PWRSET,
    #[doc = "0x08 - PCU Clear Control Register"]
    pub pwrclr: PWRCLR,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - EVR Status Register"]
    pub evrstat: EVRSTAT,
    #[doc = "0x14 - EVR VADC Status Register"]
    pub evrvadcstat: EVRVADCSTAT,
    _reserved1: [u8; 20usize],
    #[doc = "0x2c - Power Monitor Control"]
    pub pwrmon: PWRMON,
}
#[doc = "PCU Status Register"]
pub struct PWRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCU Status Register"]
pub mod pwrstat;
#[doc = "PCU Set Control Register"]
pub struct PWRSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCU Set Control Register"]
pub mod pwrset;
#[doc = "PCU Clear Control Register"]
pub struct PWRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCU Clear Control Register"]
pub mod pwrclr;
#[doc = "EVR Status Register"]
pub struct EVRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EVR Status Register"]
pub mod evrstat;
#[doc = "EVR VADC Status Register"]
pub struct EVRVADCSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EVR VADC Status Register"]
pub mod evrvadcstat;
#[doc = "Power Monitor Control"]
pub struct PWRMON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Monitor Control"]
pub mod pwrmon;

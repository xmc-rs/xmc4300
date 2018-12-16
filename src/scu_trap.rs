#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Trap Status Register"]
    pub trapstat: TRAPSTAT,
    #[doc = "0x04 - Trap Raw Status Register"]
    pub trapraw: TRAPRAW,
    #[doc = "0x08 - Trap Disable Register"]
    pub trapdis: TRAPDIS,
    #[doc = "0x0c - Trap Clear Register"]
    pub trapclr: TRAPCLR,
    #[doc = "0x10 - Trap Set Register"]
    pub trapset: TRAPSET,
}
#[doc = "Trap Status Register"]
pub struct TRAPSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trap Status Register"]
pub mod trapstat;
#[doc = "Trap Raw Status Register"]
pub struct TRAPRAW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trap Raw Status Register"]
pub mod trapraw;
#[doc = "Trap Disable Register"]
pub struct TRAPDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trap Disable Register"]
pub mod trapdis;
#[doc = "Trap Clear Register"]
pub struct TRAPCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trap Clear Register"]
pub mod trapclr;
#[doc = "Trap Set Register"]
pub struct TRAPSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trap Set Register"]
pub mod trapset;

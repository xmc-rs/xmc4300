#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Peripheral Bridge Status Register"]
    pub sts: STS,
    #[doc = "0x04 - PBA Write Error Address Register"]
    pub waddr: WADDR,
}
#[doc = "Peripheral Bridge Status Register"]
pub struct STS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Bridge Status Register"]
pub mod sts;
#[doc = "PBA Write Error Address Register"]
pub struct WADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PBA Write Error Address Register"]
pub mod waddr;

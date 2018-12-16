#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Prefetch Configuration Register"]
    pub pcon: PCON,
}
#[doc = "Prefetch Configuration Register"]
pub struct PCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Prefetch Configuration Register"]
pub mod pcon;

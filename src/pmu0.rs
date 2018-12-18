#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PMU0 Identification Register"]
    pub id: ID,
}
#[doc = "PMU0 Identification Register"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMU0 Identification Register"]
pub mod id;

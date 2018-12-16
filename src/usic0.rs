#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Identification Register"]
    pub id: ID,
}
#[doc = "Module Identification Register"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Identification Register"]
pub mod id;

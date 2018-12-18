#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event Input Select"]
    pub exisel: EXISEL,
    _reserved0: [u8; 12usize],
    #[doc = "0x10 - Event Input Control"]
    pub exicon: [EXICON; 4],
    #[doc = "0x20 - Event Output Trigger Control"]
    pub exocon: [EXOCON; 4],
}
#[doc = "Event Input Select"]
pub struct EXISEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Input Select"]
pub mod exisel;
#[doc = "Event Input Control"]
pub struct EXICON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Input Control"]
pub mod exicon;
#[doc = "Event Output Trigger Control"]
pub struct EXOCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Output Trigger Control"]
pub mod exocon;

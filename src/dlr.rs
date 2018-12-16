#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Overrun Status"]
    pub ovrstat: OVRSTAT,
    #[doc = "0x04 - Overrun Clear"]
    pub ovrclr: OVRCLR,
    #[doc = "0x08 - Service Request Selection 0"]
    pub srsel0: SRSEL0,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - Line Enable"]
    pub lnen: LNEN,
}
#[doc = "Overrun Status"]
pub struct OVRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Overrun Status"]
pub mod ovrstat;
#[doc = "Overrun Clear"]
pub struct OVRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Overrun Clear"]
pub mod ovrclr;
#[doc = "Service Request Selection 0"]
pub struct SRSEL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Service Request Selection 0"]
pub mod srsel0;
#[doc = "Line Enable"]
pub struct LNEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Line Enable"]
pub mod lnen;

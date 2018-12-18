#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Hibernate Domain Status Register"]
    pub hdstat: HDSTAT,
    #[doc = "0x04 - Hibernate Domain Status Clear Register"]
    pub hdclr: HDCLR,
    #[doc = "0x08 - Hibernate Domain Status Set Register"]
    pub hdset: HDSET,
    #[doc = "0x0c - Hibernate Domain Control Register"]
    pub hdcr: HDCR,
    _reserved0: [u8; 4usize],
    #[doc = "0x14 - fOSI Control Register"]
    pub oscsictrl: OSCSICTRL,
    #[doc = "0x18 - OSC_ULP Status Register"]
    pub osculstat: OSCULSTAT,
    #[doc = "0x1c - OSC_ULP Control Register"]
    pub osculctrl: OSCULCTRL,
}
#[doc = "Hibernate Domain Status Register"]
pub struct HDSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernate Domain Status Register"]
pub mod hdstat;
#[doc = "Hibernate Domain Status Clear Register"]
pub struct HDCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernate Domain Status Clear Register"]
pub mod hdclr;
#[doc = "Hibernate Domain Status Set Register"]
pub struct HDSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernate Domain Status Set Register"]
pub mod hdset;
#[doc = "Hibernate Domain Control Register"]
pub struct HDCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernate Domain Control Register"]
pub mod hdcr;
#[doc = "fOSI Control Register"]
pub struct OSCSICTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "fOSI Control Register"]
pub mod oscsictrl;
#[doc = "OSC_ULP Status Register"]
pub struct OSCULSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OSC_ULP Status Register"]
pub mod osculstat;
#[doc = "OSC_ULP Control Register"]
pub struct OSCULCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OSC_ULP Control Register"]
pub mod osculctrl;

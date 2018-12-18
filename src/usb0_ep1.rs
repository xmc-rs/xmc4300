#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device Endpoint Control Register \\[ISOCONT\\]"]
    pub diepctl_isocont: DIEPCTL_ISOCONT,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - Device Endpoint Interrupt Register"]
    pub diepint: DIEPINT,
    _reserved1: [u8; 4usize],
    #[doc = "0x10 - Device Endpoint Transfer Size Register"]
    pub dieptsiz: DIEPTSIZ,
    #[doc = "0x14 - Device Endpoint DMA Address Register"]
    pub diepdma: DIEPDMA,
    #[doc = "0x18 - Device IN Endpoint Transmit FIFO Status Register"]
    pub dtxfsts: DTXFSTS,
    #[doc = "0x1c - Device Endpoint DMA Buffer Address Register"]
    pub diepdmab: DIEPDMAB,
    _reserved2: [u8; 480usize],
    #[doc = "0x200 - Device Endpoint Control Register \\[ISOCONT\\]"]
    pub doepctl_isocont: DOEPCTL_ISOCONT,
    _reserved3: [u8; 4usize],
    #[doc = "0x208 - Device Endpoint Interrupt Register"]
    pub doepint: DOEPINT,
    _reserved4: [u8; 4usize],
    #[doc = "0x210 - Device Endpoint Transfer Size Register \\[ISO\\]"]
    pub doeptsiz_iso: DOEPTSIZ_ISO,
    #[doc = "0x214 - Device Endpoint DMA Address Register"]
    pub doepdma: DOEPDMA,
    _reserved5: [u8; 4usize],
    #[doc = "0x21c - Device Endpoint DMA Buffer Address Register"]
    pub doepdmab: DOEPDMAB,
}
#[doc = "Device Endpoint Control Register \\[ISOCONT\\]"]
pub struct DIEPCTL_ISOCONT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Control Register \\[ISOCONT\\]"]
pub mod diepctl_isocont;
#[doc = "Device Endpoint Control Register \\[INTBULK\\]"]
pub struct DIEPCTL_INTBULK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Control Register \\[INTBULK\\]"]
pub mod diepctl_intbulk;
#[doc = "Device Endpoint Interrupt Register"]
pub struct DIEPINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Register"]
pub mod diepint;
#[doc = "Device Endpoint Transfer Size Register"]
pub struct DIEPTSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Transfer Size Register"]
pub mod dieptsiz;
#[doc = "Device Endpoint DMA Address Register"]
pub struct DIEPDMA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint DMA Address Register"]
pub mod diepdma;
#[doc = "Device IN Endpoint Transmit FIFO Status Register"]
pub struct DTXFSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint Transmit FIFO Status Register"]
pub mod dtxfsts;
#[doc = "Device Endpoint DMA Buffer Address Register"]
pub struct DIEPDMAB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint DMA Buffer Address Register"]
pub mod diepdmab;
#[doc = "Device Endpoint Control Register \\[ISOCONT\\]"]
pub struct DOEPCTL_ISOCONT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Control Register \\[ISOCONT\\]"]
pub mod doepctl_isocont;
#[doc = "Device Endpoint Control Register \\[INTBULK\\]"]
pub struct DOEPCTL_INTBULK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Control Register \\[INTBULK\\]"]
pub mod doepctl_intbulk;
#[doc = "Device Endpoint Interrupt Register"]
pub struct DOEPINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Register"]
pub mod doepint;
#[doc = "Device Endpoint Transfer Size Register \\[ISO\\]"]
pub struct DOEPTSIZ_ISO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Transfer Size Register \\[ISO\\]"]
pub mod doeptsiz_iso;
#[doc = "Device Endpoint Transfer Size Register \\[CONT\\]"]
pub struct DOEPTSIZ_CONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Transfer Size Register \\[CONT\\]"]
pub mod doeptsiz_control;
#[doc = "Device Endpoint DMA Address Register"]
pub struct DOEPDMA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint DMA Address Register"]
pub mod doepdma;
#[doc = "Device Endpoint DMA Buffer Address Register"]
pub struct DOEPDMAB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint DMA Buffer Address Register"]
pub mod doepdmab;

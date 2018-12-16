#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device Control IN Endpoint Control Register"]
    pub diepctl0: DIEPCTL0,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - Device Endpoint Interrupt Register"]
    pub diepint0: DIEPINT0,
    _reserved1: [u8; 4usize],
    #[doc = "0x10 - Device IN Endpoint Transfer Size Register"]
    pub dieptsiz0: DIEPTSIZ0,
    #[doc = "0x14 - Device Endpoint DMA Address Register"]
    pub diepdma0: DIEPDMA0,
    #[doc = "0x18 - Device IN Endpoint Transmit FIFO Status Register"]
    pub dtxfsts0: DTXFSTS0,
    #[doc = "0x1c - Device Endpoint DMA Buffer Address Register"]
    pub diepdmab0: DIEPDMAB0,
    _reserved2: [u8; 480usize],
    #[doc = "0x200 - Device Control OUT Endpoint Control Register"]
    pub doepctl0: DOEPCTL0,
    _reserved3: [u8; 4usize],
    #[doc = "0x208 - Device Endpoint Interrupt Register"]
    pub doepint0: DOEPINT0,
    _reserved4: [u8; 4usize],
    #[doc = "0x210 - Device OUT Endpoint Transfer Size Register"]
    pub doeptsiz0: DOEPTSIZ0,
    #[doc = "0x214 - Device Endpoint DMA Address Register"]
    pub doepdma0: DOEPDMA0,
    _reserved5: [u8; 4usize],
    #[doc = "0x21c - Device Endpoint DMA Buffer Address Register"]
    pub doepdmab0: DOEPDMAB0,
}
#[doc = "Device Control IN Endpoint Control Register"]
pub struct DIEPCTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Control IN Endpoint Control Register"]
pub mod diepctl0;
#[doc = "Device Endpoint Interrupt Register"]
pub struct DIEPINT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Register"]
pub mod diepint0;
#[doc = "Device IN Endpoint Transfer Size Register"]
pub struct DIEPTSIZ0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint Transfer Size Register"]
pub mod dieptsiz0;
#[doc = "Device Endpoint DMA Address Register"]
pub struct DIEPDMA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint DMA Address Register"]
pub mod diepdma0;
#[doc = "Device IN Endpoint Transmit FIFO Status Register"]
pub struct DTXFSTS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint Transmit FIFO Status Register"]
pub mod dtxfsts0;
#[doc = "Device Endpoint DMA Buffer Address Register"]
pub struct DIEPDMAB0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint DMA Buffer Address Register"]
pub mod diepdmab0;
#[doc = "Device Control OUT Endpoint Control Register"]
pub struct DOEPCTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Control OUT Endpoint Control Register"]
pub mod doepctl0;
#[doc = "Device Endpoint Interrupt Register"]
pub struct DOEPINT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Interrupt Register"]
pub mod doepint0;
#[doc = "Device OUT Endpoint Transfer Size Register"]
pub struct DOEPTSIZ0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint Transfer Size Register"]
pub mod doeptsiz0;
#[doc = "Device Endpoint DMA Address Register"]
pub struct DOEPDMA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint DMA Address Register"]
pub mod doepdma0;
#[doc = "Device Endpoint DMA Buffer Address Register"]
pub struct DOEPDMAB0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint DMA Buffer Address Register"]
pub mod doepdmab0;

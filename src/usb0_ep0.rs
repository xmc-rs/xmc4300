#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device Control IN Endpoint Control Register"]
    pub diepctl0: DIEPCTL0,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Device Endpoint Interrupt Register"]
    pub diepint0: DIEPINT0,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - Device IN Endpoint Transfer Size Register"]
    pub dieptsiz0: DIEPTSIZ0,
    #[doc = "0x14 - Device Endpoint DMA Address Register"]
    pub diepdma0: DIEPDMA0,
    #[doc = "0x18 - Device IN Endpoint Transmit FIFO Status Register"]
    pub dtxfsts0: DTXFSTS0,
    #[doc = "0x1c - Device Endpoint DMA Buffer Address Register"]
    pub diepdmab0: DIEPDMAB0,
    _reserved6: [u8; 0x01e0],
    #[doc = "0x200 - Device Control OUT Endpoint Control Register"]
    pub doepctl0: DOEPCTL0,
    _reserved7: [u8; 0x04],
    #[doc = "0x208 - Device Endpoint Interrupt Register"]
    pub doepint0: DOEPINT0,
    _reserved8: [u8; 0x04],
    #[doc = "0x210 - Device OUT Endpoint Transfer Size Register"]
    pub doeptsiz0: DOEPTSIZ0,
    #[doc = "0x214 - Device Endpoint DMA Address Register"]
    pub doepdma0: DOEPDMA0,
    _reserved10: [u8; 0x04],
    #[doc = "0x21c - Device Endpoint DMA Buffer Address Register"]
    pub doepdmab0: DOEPDMAB0,
}
#[doc = "DIEPCTL0 (rw) register accessor: an alias for `Reg<DIEPCTL0_SPEC>`"]
pub type DIEPCTL0 = crate::Reg<diepctl0::DIEPCTL0_SPEC>;
#[doc = "Device Control IN Endpoint Control Register"]
pub mod diepctl0;
#[doc = "DIEPINT0 (rw) register accessor: an alias for `Reg<DIEPINT0_SPEC>`"]
pub type DIEPINT0 = crate::Reg<diepint0::DIEPINT0_SPEC>;
#[doc = "Device Endpoint Interrupt Register"]
pub mod diepint0;
#[doc = "DIEPTSIZ0 (rw) register accessor: an alias for `Reg<DIEPTSIZ0_SPEC>`"]
pub type DIEPTSIZ0 = crate::Reg<dieptsiz0::DIEPTSIZ0_SPEC>;
#[doc = "Device IN Endpoint Transfer Size Register"]
pub mod dieptsiz0;
#[doc = "DIEPDMA0 (rw) register accessor: an alias for `Reg<DIEPDMA0_SPEC>`"]
pub type DIEPDMA0 = crate::Reg<diepdma0::DIEPDMA0_SPEC>;
#[doc = "Device Endpoint DMA Address Register"]
pub mod diepdma0;
#[doc = "DTXFSTS0 (r) register accessor: an alias for `Reg<DTXFSTS0_SPEC>`"]
pub type DTXFSTS0 = crate::Reg<dtxfsts0::DTXFSTS0_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register"]
pub mod dtxfsts0;
#[doc = "DIEPDMAB0 (r) register accessor: an alias for `Reg<DIEPDMAB0_SPEC>`"]
pub type DIEPDMAB0 = crate::Reg<diepdmab0::DIEPDMAB0_SPEC>;
#[doc = "Device Endpoint DMA Buffer Address Register"]
pub mod diepdmab0;
#[doc = "DOEPCTL0 (rw) register accessor: an alias for `Reg<DOEPCTL0_SPEC>`"]
pub type DOEPCTL0 = crate::Reg<doepctl0::DOEPCTL0_SPEC>;
#[doc = "Device Control OUT Endpoint Control Register"]
pub mod doepctl0;
#[doc = "DOEPINT0 (rw) register accessor: an alias for `Reg<DOEPINT0_SPEC>`"]
pub type DOEPINT0 = crate::Reg<doepint0::DOEPINT0_SPEC>;
#[doc = "Device Endpoint Interrupt Register"]
pub mod doepint0;
#[doc = "DOEPTSIZ0 (rw) register accessor: an alias for `Reg<DOEPTSIZ0_SPEC>`"]
pub type DOEPTSIZ0 = crate::Reg<doeptsiz0::DOEPTSIZ0_SPEC>;
#[doc = "Device OUT Endpoint Transfer Size Register"]
pub mod doeptsiz0;
#[doc = "DOEPDMA0 (rw) register accessor: an alias for `Reg<DOEPDMA0_SPEC>`"]
pub type DOEPDMA0 = crate::Reg<doepdma0::DOEPDMA0_SPEC>;
#[doc = "Device Endpoint DMA Address Register"]
pub mod doepdma0;
#[doc = "DOEPDMAB0 (r) register accessor: an alias for `Reg<DOEPDMAB0_SPEC>`"]
pub type DOEPDMAB0 = crate::Reg<doepdmab0::DOEPDMAB0_SPEC>;
#[doc = "Device Endpoint DMA Buffer Address Register"]
pub mod doepdmab0;

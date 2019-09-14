#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device Control IN Endpoint Control Register"]
    pub diepctl0: DIEPCTL0,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Device Endpoint Interrupt Register"]
    pub diepint0: DIEPINT0,
    _reserved2: [u8; 4usize],
    #[doc = "0x10 - Device IN Endpoint Transfer Size Register"]
    pub dieptsiz0: DIEPTSIZ0,
    #[doc = "0x14 - Device Endpoint DMA Address Register"]
    pub diepdma0: DIEPDMA0,
    #[doc = "0x18 - Device IN Endpoint Transmit FIFO Status Register"]
    pub dtxfsts0: DTXFSTS0,
    #[doc = "0x1c - Device Endpoint DMA Buffer Address Register"]
    pub diepdmab0: DIEPDMAB0,
    _reserved6: [u8; 480usize],
    #[doc = "0x200 - Device Control OUT Endpoint Control Register"]
    pub doepctl0: DOEPCTL0,
    _reserved7: [u8; 4usize],
    #[doc = "0x208 - Device Endpoint Interrupt Register"]
    pub doepint0: DOEPINT0,
    _reserved8: [u8; 4usize],
    #[doc = "0x210 - Device OUT Endpoint Transfer Size Register"]
    pub doeptsiz0: DOEPTSIZ0,
    #[doc = "0x214 - Device Endpoint DMA Address Register"]
    pub doepdma0: DOEPDMA0,
    _reserved10: [u8; 4usize],
    #[doc = "0x21c - Device Endpoint DMA Buffer Address Register"]
    pub doepdmab0: DOEPDMAB0,
}
#[doc = "Device Control IN Endpoint Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diepctl0](diepctl0) module"]
pub type DIEPCTL0 = crate::Reg<u32, _DIEPCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPCTL0;
#[doc = "`read()` method returns [diepctl0::R](diepctl0::R) reader structure"]
impl crate::Readable for DIEPCTL0 {}
#[doc = "`write(|w| ..)` method takes [diepctl0::W](diepctl0::W) writer structure"]
impl crate::Writable for DIEPCTL0 {}
#[doc = "Device Control IN Endpoint Control Register"]
pub mod diepctl0;
#[doc = "Device Endpoint Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diepint0](diepint0) module"]
pub type DIEPINT0 = crate::Reg<u32, _DIEPINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPINT0;
#[doc = "`read()` method returns [diepint0::R](diepint0::R) reader structure"]
impl crate::Readable for DIEPINT0 {}
#[doc = "`write(|w| ..)` method takes [diepint0::W](diepint0::W) writer structure"]
impl crate::Writable for DIEPINT0 {}
#[doc = "Device Endpoint Interrupt Register"]
pub mod diepint0;
#[doc = "Device IN Endpoint Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dieptsiz0](dieptsiz0) module"]
pub type DIEPTSIZ0 = crate::Reg<u32, _DIEPTSIZ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTSIZ0;
#[doc = "`read()` method returns [dieptsiz0::R](dieptsiz0::R) reader structure"]
impl crate::Readable for DIEPTSIZ0 {}
#[doc = "`write(|w| ..)` method takes [dieptsiz0::W](dieptsiz0::W) writer structure"]
impl crate::Writable for DIEPTSIZ0 {}
#[doc = "Device IN Endpoint Transfer Size Register"]
pub mod dieptsiz0;
#[doc = "Device Endpoint DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diepdma0](diepdma0) module"]
pub type DIEPDMA0 = crate::Reg<u32, _DIEPDMA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPDMA0;
#[doc = "`read()` method returns [diepdma0::R](diepdma0::R) reader structure"]
impl crate::Readable for DIEPDMA0 {}
#[doc = "`write(|w| ..)` method takes [diepdma0::W](diepdma0::W) writer structure"]
impl crate::Writable for DIEPDMA0 {}
#[doc = "Device Endpoint DMA Address Register"]
pub mod diepdma0;
#[doc = "Device IN Endpoint Transmit FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dtxfsts0](dtxfsts0) module"]
pub type DTXFSTS0 = crate::Reg<u32, _DTXFSTS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTXFSTS0;
#[doc = "`read()` method returns [dtxfsts0::R](dtxfsts0::R) reader structure"]
impl crate::Readable for DTXFSTS0 {}
#[doc = "Device IN Endpoint Transmit FIFO Status Register"]
pub mod dtxfsts0;
#[doc = "Device Endpoint DMA Buffer Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diepdmab0](diepdmab0) module"]
pub type DIEPDMAB0 = crate::Reg<u32, _DIEPDMAB0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPDMAB0;
#[doc = "`read()` method returns [diepdmab0::R](diepdmab0::R) reader structure"]
impl crate::Readable for DIEPDMAB0 {}
#[doc = "Device Endpoint DMA Buffer Address Register"]
pub mod diepdmab0;
#[doc = "Device Control OUT Endpoint Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doepctl0](doepctl0) module"]
pub type DOEPCTL0 = crate::Reg<u32, _DOEPCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPCTL0;
#[doc = "`read()` method returns [doepctl0::R](doepctl0::R) reader structure"]
impl crate::Readable for DOEPCTL0 {}
#[doc = "`write(|w| ..)` method takes [doepctl0::W](doepctl0::W) writer structure"]
impl crate::Writable for DOEPCTL0 {}
#[doc = "Device Control OUT Endpoint Control Register"]
pub mod doepctl0;
#[doc = "Device Endpoint Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doepint0](doepint0) module"]
pub type DOEPINT0 = crate::Reg<u32, _DOEPINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPINT0;
#[doc = "`read()` method returns [doepint0::R](doepint0::R) reader structure"]
impl crate::Readable for DOEPINT0 {}
#[doc = "`write(|w| ..)` method takes [doepint0::W](doepint0::W) writer structure"]
impl crate::Writable for DOEPINT0 {}
#[doc = "Device Endpoint Interrupt Register"]
pub mod doepint0;
#[doc = "Device OUT Endpoint Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doeptsiz0](doeptsiz0) module"]
pub type DOEPTSIZ0 = crate::Reg<u32, _DOEPTSIZ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPTSIZ0;
#[doc = "`read()` method returns [doeptsiz0::R](doeptsiz0::R) reader structure"]
impl crate::Readable for DOEPTSIZ0 {}
#[doc = "`write(|w| ..)` method takes [doeptsiz0::W](doeptsiz0::W) writer structure"]
impl crate::Writable for DOEPTSIZ0 {}
#[doc = "Device OUT Endpoint Transfer Size Register"]
pub mod doeptsiz0;
#[doc = "Device Endpoint DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doepdma0](doepdma0) module"]
pub type DOEPDMA0 = crate::Reg<u32, _DOEPDMA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPDMA0;
#[doc = "`read()` method returns [doepdma0::R](doepdma0::R) reader structure"]
impl crate::Readable for DOEPDMA0 {}
#[doc = "`write(|w| ..)` method takes [doepdma0::W](doepdma0::W) writer structure"]
impl crate::Writable for DOEPDMA0 {}
#[doc = "Device Endpoint DMA Address Register"]
pub mod doepdma0;
#[doc = "Device Endpoint DMA Buffer Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doepdmab0](doepdmab0) module"]
pub type DOEPDMAB0 = crate::Reg<u32, _DOEPDMAB0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPDMAB0;
#[doc = "`read()` method returns [doepdmab0::R](doepdmab0::R) reader structure"]
impl crate::Readable for DOEPDMAB0 {}
#[doc = "Device Endpoint DMA Buffer Address Register"]
pub mod doepdmab0;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_diepctl: [u8; 4usize],
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Device Endpoint Interrupt Register"]
    pub diepint: DIEPINT,
    _reserved2: [u8; 4usize],
    #[doc = "0x10 - Device Endpoint Transfer Size Register"]
    pub dieptsiz: DIEPTSIZ,
    #[doc = "0x14 - Device Endpoint DMA Address Register"]
    pub diepdma: DIEPDMA,
    #[doc = "0x18 - Device IN Endpoint Transmit FIFO Status Register"]
    pub dtxfsts: DTXFSTS,
    #[doc = "0x1c - Device Endpoint DMA Buffer Address Register"]
    pub diepdmab: DIEPDMAB,
    _reserved6: [u8; 480usize],
    _reserved_6_doepctl: [u8; 4usize],
    _reserved7: [u8; 4usize],
    #[doc = "0x208 - Device Endpoint Interrupt Register"]
    pub doepint: DOEPINT,
    _reserved8: [u8; 4usize],
    _reserved_8_doeptsiz: [u8; 4usize],
    #[doc = "0x214 - Device Endpoint DMA Address Register"]
    pub doepdma: DOEPDMA,
    _reserved10: [u8; 4usize],
    #[doc = "0x21c - Device Endpoint DMA Buffer Address Register"]
    pub doepdmab: DOEPDMAB,
}
impl RegisterBlock {
    #[doc = "0x00 - Device Endpoint Control Register \\[INTBULK\\]"]
    #[inline(always)]
    pub fn diepctl_intbulk(&self) -> &DIEPCTL_INTBULK {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DIEPCTL_INTBULK) }
    }
    #[doc = "0x00 - Device Endpoint Control Register \\[INTBULK\\]"]
    #[inline(always)]
    pub fn diepctl_intbulk_mut(&self) -> &mut DIEPCTL_INTBULK {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut DIEPCTL_INTBULK) }
    }
    #[doc = "0x00 - Device Endpoint Control Register \\[ISOCONT\\]"]
    #[inline(always)]
    pub fn diepctl_isocont(&self) -> &DIEPCTL_ISOCONT {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DIEPCTL_ISOCONT) }
    }
    #[doc = "0x00 - Device Endpoint Control Register \\[ISOCONT\\]"]
    #[inline(always)]
    pub fn diepctl_isocont_mut(&self) -> &mut DIEPCTL_ISOCONT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut DIEPCTL_ISOCONT) }
    }
    #[doc = "0x200 - Device Endpoint Control Register \\[INTBULK\\]"]
    #[inline(always)]
    pub fn doepctl_intbulk(&self) -> &DOEPCTL_INTBULK {
        unsafe { &*(((self as *const Self) as *const u8).add(512usize) as *const DOEPCTL_INTBULK) }
    }
    #[doc = "0x200 - Device Endpoint Control Register \\[INTBULK\\]"]
    #[inline(always)]
    pub fn doepctl_intbulk_mut(&self) -> &mut DOEPCTL_INTBULK {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(512usize) as *mut DOEPCTL_INTBULK) }
    }
    #[doc = "0x200 - Device Endpoint Control Register \\[ISOCONT\\]"]
    #[inline(always)]
    pub fn doepctl_isocont(&self) -> &DOEPCTL_ISOCONT {
        unsafe { &*(((self as *const Self) as *const u8).add(512usize) as *const DOEPCTL_ISOCONT) }
    }
    #[doc = "0x200 - Device Endpoint Control Register \\[ISOCONT\\]"]
    #[inline(always)]
    pub fn doepctl_isocont_mut(&self) -> &mut DOEPCTL_ISOCONT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(512usize) as *mut DOEPCTL_ISOCONT) }
    }
    #[doc = "0x210 - Device Endpoint Transfer Size Register \\[CONT\\]"]
    #[inline(always)]
    pub fn doeptsiz_control(&self) -> &DOEPTSIZ_CONTROL {
        unsafe { &*(((self as *const Self) as *const u8).add(528usize) as *const DOEPTSIZ_CONTROL) }
    }
    #[doc = "0x210 - Device Endpoint Transfer Size Register \\[CONT\\]"]
    #[inline(always)]
    pub fn doeptsiz_control_mut(&self) -> &mut DOEPTSIZ_CONTROL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(528usize) as *mut DOEPTSIZ_CONTROL) }
    }
    #[doc = "0x210 - Device Endpoint Transfer Size Register \\[ISO\\]"]
    #[inline(always)]
    pub fn doeptsiz_iso(&self) -> &DOEPTSIZ_ISO {
        unsafe { &*(((self as *const Self) as *const u8).add(528usize) as *const DOEPTSIZ_ISO) }
    }
    #[doc = "0x210 - Device Endpoint Transfer Size Register \\[ISO\\]"]
    #[inline(always)]
    pub fn doeptsiz_iso_mut(&self) -> &mut DOEPTSIZ_ISO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(528usize) as *mut DOEPTSIZ_ISO) }
    }
}
#[doc = "Device Endpoint Control Register \\[ISOCONT\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diepctl_isocont](diepctl_isocont) module"]
pub type DIEPCTL_ISOCONT = crate::Reg<u32, _DIEPCTL_ISOCONT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPCTL_ISOCONT;
#[doc = "`read()` method returns [diepctl_isocont::R](diepctl_isocont::R) reader structure"]
impl crate::Readable for DIEPCTL_ISOCONT {}
#[doc = "`write(|w| ..)` method takes [diepctl_isocont::W](diepctl_isocont::W) writer structure"]
impl crate::Writable for DIEPCTL_ISOCONT {}
#[doc = "Device Endpoint Control Register \\[ISOCONT\\]"]
pub mod diepctl_isocont;
#[doc = "Device Endpoint Control Register \\[INTBULK\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diepctl_intbulk](diepctl_intbulk) module"]
pub type DIEPCTL_INTBULK = crate::Reg<u32, _DIEPCTL_INTBULK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPCTL_INTBULK;
#[doc = "`read()` method returns [diepctl_intbulk::R](diepctl_intbulk::R) reader structure"]
impl crate::Readable for DIEPCTL_INTBULK {}
#[doc = "`write(|w| ..)` method takes [diepctl_intbulk::W](diepctl_intbulk::W) writer structure"]
impl crate::Writable for DIEPCTL_INTBULK {}
#[doc = "Device Endpoint Control Register \\[INTBULK\\]"]
pub mod diepctl_intbulk;
#[doc = "Device Endpoint Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diepint](diepint) module"]
pub type DIEPINT = crate::Reg<u32, _DIEPINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPINT;
#[doc = "`read()` method returns [diepint::R](diepint::R) reader structure"]
impl crate::Readable for DIEPINT {}
#[doc = "`write(|w| ..)` method takes [diepint::W](diepint::W) writer structure"]
impl crate::Writable for DIEPINT {}
#[doc = "Device Endpoint Interrupt Register"]
pub mod diepint;
#[doc = "Device Endpoint Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dieptsiz](dieptsiz) module"]
pub type DIEPTSIZ = crate::Reg<u32, _DIEPTSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTSIZ;
#[doc = "`read()` method returns [dieptsiz::R](dieptsiz::R) reader structure"]
impl crate::Readable for DIEPTSIZ {}
#[doc = "`write(|w| ..)` method takes [dieptsiz::W](dieptsiz::W) writer structure"]
impl crate::Writable for DIEPTSIZ {}
#[doc = "Device Endpoint Transfer Size Register"]
pub mod dieptsiz;
#[doc = "Device Endpoint DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diepdma](diepdma) module"]
pub type DIEPDMA = crate::Reg<u32, _DIEPDMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPDMA;
#[doc = "`read()` method returns [diepdma::R](diepdma::R) reader structure"]
impl crate::Readable for DIEPDMA {}
#[doc = "`write(|w| ..)` method takes [diepdma::W](diepdma::W) writer structure"]
impl crate::Writable for DIEPDMA {}
#[doc = "Device Endpoint DMA Address Register"]
pub mod diepdma;
#[doc = "Device IN Endpoint Transmit FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dtxfsts](dtxfsts) module"]
pub type DTXFSTS = crate::Reg<u32, _DTXFSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTXFSTS;
#[doc = "`read()` method returns [dtxfsts::R](dtxfsts::R) reader structure"]
impl crate::Readable for DTXFSTS {}
#[doc = "Device IN Endpoint Transmit FIFO Status Register"]
pub mod dtxfsts;
#[doc = "Device Endpoint DMA Buffer Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diepdmab](diepdmab) module"]
pub type DIEPDMAB = crate::Reg<u32, _DIEPDMAB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPDMAB;
#[doc = "`read()` method returns [diepdmab::R](diepdmab::R) reader structure"]
impl crate::Readable for DIEPDMAB {}
#[doc = "Device Endpoint DMA Buffer Address Register"]
pub mod diepdmab;
#[doc = "Device Endpoint Control Register \\[ISOCONT\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doepctl_isocont](doepctl_isocont) module"]
pub type DOEPCTL_ISOCONT = crate::Reg<u32, _DOEPCTL_ISOCONT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPCTL_ISOCONT;
#[doc = "`read()` method returns [doepctl_isocont::R](doepctl_isocont::R) reader structure"]
impl crate::Readable for DOEPCTL_ISOCONT {}
#[doc = "`write(|w| ..)` method takes [doepctl_isocont::W](doepctl_isocont::W) writer structure"]
impl crate::Writable for DOEPCTL_ISOCONT {}
#[doc = "Device Endpoint Control Register \\[ISOCONT\\]"]
pub mod doepctl_isocont;
#[doc = "Device Endpoint Control Register \\[INTBULK\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doepctl_intbulk](doepctl_intbulk) module"]
pub type DOEPCTL_INTBULK = crate::Reg<u32, _DOEPCTL_INTBULK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPCTL_INTBULK;
#[doc = "`read()` method returns [doepctl_intbulk::R](doepctl_intbulk::R) reader structure"]
impl crate::Readable for DOEPCTL_INTBULK {}
#[doc = "`write(|w| ..)` method takes [doepctl_intbulk::W](doepctl_intbulk::W) writer structure"]
impl crate::Writable for DOEPCTL_INTBULK {}
#[doc = "Device Endpoint Control Register \\[INTBULK\\]"]
pub mod doepctl_intbulk;
#[doc = "Device Endpoint Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doepint](doepint) module"]
pub type DOEPINT = crate::Reg<u32, _DOEPINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPINT;
#[doc = "`read()` method returns [doepint::R](doepint::R) reader structure"]
impl crate::Readable for DOEPINT {}
#[doc = "`write(|w| ..)` method takes [doepint::W](doepint::W) writer structure"]
impl crate::Writable for DOEPINT {}
#[doc = "Device Endpoint Interrupt Register"]
pub mod doepint;
#[doc = "Device Endpoint Transfer Size Register \\[ISO\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doeptsiz_iso](doeptsiz_iso) module"]
pub type DOEPTSIZ_ISO = crate::Reg<u32, _DOEPTSIZ_ISO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPTSIZ_ISO;
#[doc = "`read()` method returns [doeptsiz_iso::R](doeptsiz_iso::R) reader structure"]
impl crate::Readable for DOEPTSIZ_ISO {}
#[doc = "`write(|w| ..)` method takes [doeptsiz_iso::W](doeptsiz_iso::W) writer structure"]
impl crate::Writable for DOEPTSIZ_ISO {}
#[doc = "Device Endpoint Transfer Size Register \\[ISO\\]"]
pub mod doeptsiz_iso;
#[doc = "Device Endpoint Transfer Size Register \\[CONT\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doeptsiz_control](doeptsiz_control) module"]
pub type DOEPTSIZ_CONTROL = crate::Reg<u32, _DOEPTSIZ_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPTSIZ_CONTROL;
#[doc = "`read()` method returns [doeptsiz_control::R](doeptsiz_control::R) reader structure"]
impl crate::Readable for DOEPTSIZ_CONTROL {}
#[doc = "`write(|w| ..)` method takes [doeptsiz_control::W](doeptsiz_control::W) writer structure"]
impl crate::Writable for DOEPTSIZ_CONTROL {}
#[doc = "Device Endpoint Transfer Size Register \\[CONT\\]"]
pub mod doeptsiz_control;
#[doc = "Device Endpoint DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doepdma](doepdma) module"]
pub type DOEPDMA = crate::Reg<u32, _DOEPDMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPDMA;
#[doc = "`read()` method returns [doepdma::R](doepdma::R) reader structure"]
impl crate::Readable for DOEPDMA {}
#[doc = "`write(|w| ..)` method takes [doepdma::W](doepdma::W) writer structure"]
impl crate::Writable for DOEPDMA {}
#[doc = "Device Endpoint DMA Address Register"]
pub mod doepdma;
#[doc = "Device Endpoint DMA Buffer Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doepdmab](doepdmab) module"]
pub type DOEPDMAB = crate::Reg<u32, _DOEPDMAB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPDMAB;
#[doc = "`read()` method returns [doepdmab::R](doepdmab::R) reader structure"]
impl crate::Readable for DOEPDMAB {}
#[doc = "Device Endpoint DMA Buffer Address Register"]
pub mod doepdmab;

#[doc = r"Register block"]
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
    _reserved4: [u8; 4usize],
    #[doc = "0x14 - fOSI Control Register"]
    pub oscsictrl: OSCSICTRL,
    #[doc = "0x18 - OSC_ULP Status Register"]
    pub osculstat: OSCULSTAT,
    #[doc = "0x1c - OSC_ULP Control Register"]
    pub osculctrl: OSCULCTRL,
}
#[doc = "Hibernate Domain Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdstat](hdstat) module"]
pub type HDSTAT = crate::Reg<u32, _HDSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDSTAT;
#[doc = "`read()` method returns [hdstat::R](hdstat::R) reader structure"]
impl crate::Readable for HDSTAT {}
#[doc = "Hibernate Domain Status Register"]
pub mod hdstat;
#[doc = "Hibernate Domain Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdclr](hdclr) module"]
pub type HDCLR = crate::Reg<u32, _HDCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDCLR;
#[doc = "`write(|w| ..)` method takes [hdclr::W](hdclr::W) writer structure"]
impl crate::Writable for HDCLR {}
#[doc = "Hibernate Domain Status Clear Register"]
pub mod hdclr;
#[doc = "Hibernate Domain Status Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdset](hdset) module"]
pub type HDSET = crate::Reg<u32, _HDSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDSET;
#[doc = "`write(|w| ..)` method takes [hdset::W](hdset::W) writer structure"]
impl crate::Writable for HDSET {}
#[doc = "Hibernate Domain Status Set Register"]
pub mod hdset;
#[doc = "Hibernate Domain Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdcr](hdcr) module"]
pub type HDCR = crate::Reg<u32, _HDCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDCR;
#[doc = "`read()` method returns [hdcr::R](hdcr::R) reader structure"]
impl crate::Readable for HDCR {}
#[doc = "`write(|w| ..)` method takes [hdcr::W](hdcr::W) writer structure"]
impl crate::Writable for HDCR {}
#[doc = "Hibernate Domain Control Register"]
pub mod hdcr;
#[doc = "fOSI Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscsictrl](oscsictrl) module"]
pub type OSCSICTRL = crate::Reg<u32, _OSCSICTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCSICTRL;
#[doc = "`read()` method returns [oscsictrl::R](oscsictrl::R) reader structure"]
impl crate::Readable for OSCSICTRL {}
#[doc = "`write(|w| ..)` method takes [oscsictrl::W](oscsictrl::W) writer structure"]
impl crate::Writable for OSCSICTRL {}
#[doc = "fOSI Control Register"]
pub mod oscsictrl;
#[doc = "OSC_ULP Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osculstat](osculstat) module"]
pub type OSCULSTAT = crate::Reg<u32, _OSCULSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCULSTAT;
#[doc = "`read()` method returns [osculstat::R](osculstat::R) reader structure"]
impl crate::Readable for OSCULSTAT {}
#[doc = "OSC_ULP Status Register"]
pub mod osculstat;
#[doc = "OSC_ULP Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osculctrl](osculctrl) module"]
pub type OSCULCTRL = crate::Reg<u32, _OSCULCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCULCTRL;
#[doc = "`read()` method returns [osculctrl::R](osculctrl::R) reader structure"]
impl crate::Readable for OSCULCTRL {}
#[doc = "`write(|w| ..)` method takes [osculctrl::W](osculctrl::W) writer structure"]
impl crate::Writable for OSCULCTRL {}
#[doc = "OSC_ULP Control Register"]
pub mod osculctrl;

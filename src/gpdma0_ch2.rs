#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Source Address Register"]
    pub sar: SAR,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Destination Address Register"]
    pub dar: DAR,
    _reserved2: [u8; 12usize],
    #[doc = "0x18 - Control Register Low"]
    pub ctll: CTLL,
    #[doc = "0x1c - Control Register High"]
    pub ctlh: CTLH,
    _reserved4: [u8; 32usize],
    #[doc = "0x40 - Configuration Register Low"]
    pub cfgl: CFGL,
    #[doc = "0x44 - Configuration Register High"]
    pub cfgh: CFGH,
}
#[doc = "Source Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar](sar) module"]
pub type SAR = crate::Reg<u32, _SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR;
#[doc = "`read()` method returns [sar::R](sar::R) reader structure"]
impl crate::Readable for SAR {}
#[doc = "`write(|w| ..)` method takes [sar::W](sar::W) writer structure"]
impl crate::Writable for SAR {}
#[doc = "Source Address Register"]
pub mod sar;
#[doc = "Destination Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dar](dar) module"]
pub type DAR = crate::Reg<u32, _DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAR;
#[doc = "`read()` method returns [dar::R](dar::R) reader structure"]
impl crate::Readable for DAR {}
#[doc = "`write(|w| ..)` method takes [dar::W](dar::W) writer structure"]
impl crate::Writable for DAR {}
#[doc = "Destination Address Register"]
pub mod dar;
#[doc = "Control Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctll](ctll) module"]
pub type CTLL = crate::Reg<u32, _CTLL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTLL;
#[doc = "`read()` method returns [ctll::R](ctll::R) reader structure"]
impl crate::Readable for CTLL {}
#[doc = "`write(|w| ..)` method takes [ctll::W](ctll::W) writer structure"]
impl crate::Writable for CTLL {}
#[doc = "Control Register Low"]
pub mod ctll;
#[doc = "Control Register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctlh](ctlh) module"]
pub type CTLH = crate::Reg<u32, _CTLH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTLH;
#[doc = "`read()` method returns [ctlh::R](ctlh::R) reader structure"]
impl crate::Readable for CTLH {}
#[doc = "`write(|w| ..)` method takes [ctlh::W](ctlh::W) writer structure"]
impl crate::Writable for CTLH {}
#[doc = "Control Register High"]
pub mod ctlh;
#[doc = "Configuration Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfgl](cfgl) module"]
pub type CFGL = crate::Reg<u32, _CFGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGL;
#[doc = "`read()` method returns [cfgl::R](cfgl::R) reader structure"]
impl crate::Readable for CFGL {}
#[doc = "`write(|w| ..)` method takes [cfgl::W](cfgl::W) writer structure"]
impl crate::Writable for CFGL {}
#[doc = "Configuration Register Low"]
pub mod cfgl;
#[doc = "Configuration Register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfgh](cfgh) module"]
pub type CFGH = crate::Reg<u32, _CFGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGH;
#[doc = "`read()` method returns [cfgh::R](cfgh::R) reader structure"]
impl crate::Readable for CFGH {}
#[doc = "`write(|w| ..)` method takes [cfgh::W](cfgh::W) writer structure"]
impl crate::Writable for CFGH {}
#[doc = "Configuration Register High"]
pub mod cfgh;

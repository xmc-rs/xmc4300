#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Source Address Register"]
    pub sar: SAR,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Destination Address Register"]
    pub dar: DAR,
    _reserved2: [u8; 4usize],
    #[doc = "0x10 - Linked List Pointer Register"]
    pub llp: LLP,
    _reserved3: [u8; 4usize],
    #[doc = "0x18 - Control Register Low"]
    pub ctll: CTLL,
    #[doc = "0x1c - Control Register High"]
    pub ctlh: CTLH,
    #[doc = "0x20 - Source Status Register"]
    pub sstat: SSTAT,
    _reserved6: [u8; 4usize],
    #[doc = "0x28 - Destination Status Register"]
    pub dstat: DSTAT,
    _reserved7: [u8; 4usize],
    #[doc = "0x30 - Source Status Address Register"]
    pub sstatar: SSTATAR,
    _reserved8: [u8; 4usize],
    #[doc = "0x38 - Destination Status Address Register"]
    pub dstatar: DSTATAR,
    _reserved9: [u8; 4usize],
    #[doc = "0x40 - Configuration Register Low"]
    pub cfgl: CFGL,
    #[doc = "0x44 - Configuration Register High"]
    pub cfgh: CFGH,
    #[doc = "0x48 - Source Gather Register"]
    pub sgr: SGR,
    _reserved12: [u8; 4usize],
    #[doc = "0x50 - Destination Scatter Register"]
    pub dsr: DSR,
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
#[doc = "Linked List Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [llp](llp) module"]
pub type LLP = crate::Reg<u32, _LLP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LLP;
#[doc = "`read()` method returns [llp::R](llp::R) reader structure"]
impl crate::Readable for LLP {}
#[doc = "`write(|w| ..)` method takes [llp::W](llp::W) writer structure"]
impl crate::Writable for LLP {}
#[doc = "Linked List Pointer Register"]
pub mod llp;
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
#[doc = "Source Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sstat](sstat) module"]
pub type SSTAT = crate::Reg<u32, _SSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSTAT;
#[doc = "`read()` method returns [sstat::R](sstat::R) reader structure"]
impl crate::Readable for SSTAT {}
#[doc = "`write(|w| ..)` method takes [sstat::W](sstat::W) writer structure"]
impl crate::Writable for SSTAT {}
#[doc = "Source Status Register"]
pub mod sstat;
#[doc = "Destination Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dstat](dstat) module"]
pub type DSTAT = crate::Reg<u32, _DSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSTAT;
#[doc = "`read()` method returns [dstat::R](dstat::R) reader structure"]
impl crate::Readable for DSTAT {}
#[doc = "`write(|w| ..)` method takes [dstat::W](dstat::W) writer structure"]
impl crate::Writable for DSTAT {}
#[doc = "Destination Status Register"]
pub mod dstat;
#[doc = "Source Status Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sstatar](sstatar) module"]
pub type SSTATAR = crate::Reg<u32, _SSTATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSTATAR;
#[doc = "`read()` method returns [sstatar::R](sstatar::R) reader structure"]
impl crate::Readable for SSTATAR {}
#[doc = "`write(|w| ..)` method takes [sstatar::W](sstatar::W) writer structure"]
impl crate::Writable for SSTATAR {}
#[doc = "Source Status Address Register"]
pub mod sstatar;
#[doc = "Destination Status Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dstatar](dstatar) module"]
pub type DSTATAR = crate::Reg<u32, _DSTATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSTATAR;
#[doc = "`read()` method returns [dstatar::R](dstatar::R) reader structure"]
impl crate::Readable for DSTATAR {}
#[doc = "`write(|w| ..)` method takes [dstatar::W](dstatar::W) writer structure"]
impl crate::Writable for DSTATAR {}
#[doc = "Destination Status Address Register"]
pub mod dstatar;
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
#[doc = "Source Gather Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sgr](sgr) module"]
pub type SGR = crate::Reg<u32, _SGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SGR;
#[doc = "`read()` method returns [sgr::R](sgr::R) reader structure"]
impl crate::Readable for SGR {}
#[doc = "`write(|w| ..)` method takes [sgr::W](sgr::W) writer structure"]
impl crate::Writable for SGR {}
#[doc = "Source Gather Register"]
pub mod sgr;
#[doc = "Destination Scatter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dsr](dsr) module"]
pub type DSR = crate::Reg<u32, _DSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSR;
#[doc = "`read()` method returns [dsr::R](dsr::R) reader structure"]
impl crate::Readable for DSR {}
#[doc = "`write(|w| ..)` method takes [dsr::W](dsr::W) writer structure"]
impl crate::Writable for DSR {}
#[doc = "Destination Scatter Register"]
pub mod dsr;

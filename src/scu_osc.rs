#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OSC_HP Status Register"]
    pub oschpstat: OSCHPSTAT,
    #[doc = "0x04 - OSC_HP Control Register"]
    pub oschpctrl: OSCHPCTRL,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - Clock Calibration Constant Register"]
    pub clkcalconst: CLKCALCONST,
}
#[doc = "OSC_HP Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oschpstat](oschpstat) module"]
pub type OSCHPSTAT = crate::Reg<u32, _OSCHPSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCHPSTAT;
#[doc = "`read()` method returns [oschpstat::R](oschpstat::R) reader structure"]
impl crate::Readable for OSCHPSTAT {}
#[doc = "OSC_HP Status Register"]
pub mod oschpstat;
#[doc = "OSC_HP Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oschpctrl](oschpctrl) module"]
pub type OSCHPCTRL = crate::Reg<u32, _OSCHPCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCHPCTRL;
#[doc = "`read()` method returns [oschpctrl::R](oschpctrl::R) reader structure"]
impl crate::Readable for OSCHPCTRL {}
#[doc = "`write(|w| ..)` method takes [oschpctrl::W](oschpctrl::W) writer structure"]
impl crate::Writable for OSCHPCTRL {}
#[doc = "OSC_HP Control Register"]
pub mod oschpctrl;
#[doc = "Clock Calibration Constant Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkcalconst](clkcalconst) module"]
pub type CLKCALCONST = crate::Reg<u32, _CLKCALCONST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKCALCONST;
#[doc = "`read()` method returns [clkcalconst::R](clkcalconst::R) reader structure"]
impl crate::Readable for CLKCALCONST {}
#[doc = "Clock Calibration Constant Register"]
pub mod clkcalconst;

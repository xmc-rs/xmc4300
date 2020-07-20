#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Peripheral Bridge Status Register"]
    pub sts: STS,
    #[doc = "0x04 - PBA Write Error Address Register"]
    pub waddr: WADDR,
}
#[doc = "Peripheral Bridge Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts](sts) module"]
pub type STS = crate::Reg<u32, _STS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STS;
#[doc = "`read()` method returns [sts::R](sts::R) reader structure"]
impl crate::Readable for STS {}
#[doc = "`write(|w| ..)` method takes [sts::W](sts::W) writer structure"]
impl crate::Writable for STS {}
#[doc = "Peripheral Bridge Status Register"]
pub mod sts;
#[doc = "PBA Write Error Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [waddr](waddr) module"]
pub type WADDR = crate::Reg<u32, _WADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WADDR;
#[doc = "`read()` method returns [waddr::R](waddr::R) reader structure"]
impl crate::Readable for WADDR {}
#[doc = "PBA Write Error Address Register"]
pub mod waddr;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Control Register"]
    pub clc: CLC,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Module Identification Register"]
    pub id: ID,
}
#[doc = "Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clc](clc) module"]
pub type CLC = crate::Reg<u32, _CLC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLC;
#[doc = "`read()` method returns [clc::R](clc::R) reader structure"]
impl crate::Readable for CLC {}
#[doc = "`write(|w| ..)` method takes [clc::W](clc::W) writer structure"]
impl crate::Writable for CLC {}
#[doc = "Clock Control Register"]
pub mod clc;
#[doc = "Module Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id](id) module"]
pub type ID = crate::Reg<u32, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
#[doc = "`read()` method returns [id::R](id::R) reader structure"]
impl crate::Readable for ID {}
#[doc = "Module Identification Register"]
pub mod id;

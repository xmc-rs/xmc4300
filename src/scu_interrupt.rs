#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SCU Service Request Status"]
    pub srstat: SRSTAT,
    #[doc = "0x04 - SCU Raw Service Request Status"]
    pub srraw: SRRAW,
    #[doc = "0x08 - SCU Service Request Mask"]
    pub srmsk: SRMSK,
    #[doc = "0x0c - SCU Service Request Clear"]
    pub srclr: SRCLR,
    #[doc = "0x10 - SCU Service Request Set"]
    pub srset: SRSET,
    #[doc = "0x14 - SCU Service Request Mask"]
    pub nmireqen: NMIREQEN,
}
#[doc = "SCU Service Request Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srstat](srstat) module"]
pub type SRSTAT = crate::Reg<u32, _SRSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRSTAT;
#[doc = "`read()` method returns [srstat::R](srstat::R) reader structure"]
impl crate::Readable for SRSTAT {}
#[doc = "SCU Service Request Status"]
pub mod srstat;
#[doc = "SCU Raw Service Request Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srraw](srraw) module"]
pub type SRRAW = crate::Reg<u32, _SRRAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRRAW;
#[doc = "`read()` method returns [srraw::R](srraw::R) reader structure"]
impl crate::Readable for SRRAW {}
#[doc = "SCU Raw Service Request Status"]
pub mod srraw;
#[doc = "SCU Service Request Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srmsk](srmsk) module"]
pub type SRMSK = crate::Reg<u32, _SRMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRMSK;
#[doc = "`read()` method returns [srmsk::R](srmsk::R) reader structure"]
impl crate::Readable for SRMSK {}
#[doc = "`write(|w| ..)` method takes [srmsk::W](srmsk::W) writer structure"]
impl crate::Writable for SRMSK {}
#[doc = "SCU Service Request Mask"]
pub mod srmsk;
#[doc = "SCU Service Request Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srclr](srclr) module"]
pub type SRCLR = crate::Reg<u32, _SRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCLR;
#[doc = "`write(|w| ..)` method takes [srclr::W](srclr::W) writer structure"]
impl crate::Writable for SRCLR {}
#[doc = "SCU Service Request Clear"]
pub mod srclr;
#[doc = "SCU Service Request Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srset](srset) module"]
pub type SRSET = crate::Reg<u32, _SRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRSET;
#[doc = "`write(|w| ..)` method takes [srset::W](srset::W) writer structure"]
impl crate::Writable for SRSET {}
#[doc = "SCU Service Request Set"]
pub mod srset;
#[doc = "SCU Service Request Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nmireqen](nmireqen) module"]
pub type NMIREQEN = crate::Reg<u32, _NMIREQEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NMIREQEN;
#[doc = "`read()` method returns [nmireqen::R](nmireqen::R) reader structure"]
impl crate::Readable for NMIREQEN {}
#[doc = "`write(|w| ..)` method takes [nmireqen::W](nmireqen::W) writer structure"]
impl crate::Writable for NMIREQEN {}
#[doc = "SCU Service Request Mask"]
pub mod nmireqen;

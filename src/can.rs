#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAN Clock Control Register"]
    pub clc: CLC,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Module Identification Register"]
    pub id: ID,
    #[doc = "0x0c - CAN Fractional Divider Register"]
    pub fdr: FDR,
    _reserved3: [u8; 240usize],
    #[doc = "0x100 - List Register"]
    pub list: [LIST; 16],
    #[doc = "0x140 - Message Pending Register"]
    pub mspnd: [MSPND; 8],
    _reserved5: [u8; 32usize],
    #[doc = "0x180 - Message Index Register"]
    pub msid: [MSID; 8],
    _reserved6: [u8; 32usize],
    #[doc = "0x1c0 - Message Index Mask Register"]
    pub msimask: MSIMASK,
    #[doc = "0x1c4 - Panel Control Register"]
    pub panctr: PANCTR,
    #[doc = "0x1c8 - Module Control Register"]
    pub mcr: MCR,
    #[doc = "0x1cc - Module Interrupt Trigger Register"]
    pub mitr: MITR,
}
#[doc = "CAN Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clc](clc) module"]
pub type CLC = crate::Reg<u32, _CLC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLC;
#[doc = "`read()` method returns [clc::R](clc::R) reader structure"]
impl crate::Readable for CLC {}
#[doc = "`write(|w| ..)` method takes [clc::W](clc::W) writer structure"]
impl crate::Writable for CLC {}
#[doc = "CAN Clock Control Register"]
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
#[doc = "CAN Fractional Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdr](fdr) module"]
pub type FDR = crate::Reg<u32, _FDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDR;
#[doc = "`read()` method returns [fdr::R](fdr::R) reader structure"]
impl crate::Readable for FDR {}
#[doc = "`write(|w| ..)` method takes [fdr::W](fdr::W) writer structure"]
impl crate::Writable for FDR {}
#[doc = "CAN Fractional Divider Register"]
pub mod fdr;
#[doc = "List Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [list](list) module"]
pub type LIST = crate::Reg<u32, _LIST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LIST;
#[doc = "`read()` method returns [list::R](list::R) reader structure"]
impl crate::Readable for LIST {}
#[doc = "List Register"]
pub mod list;
#[doc = "Message Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mspnd](mspnd) module"]
pub type MSPND = crate::Reg<u32, _MSPND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSPND;
#[doc = "`read()` method returns [mspnd::R](mspnd::R) reader structure"]
impl crate::Readable for MSPND {}
#[doc = "`write(|w| ..)` method takes [mspnd::W](mspnd::W) writer structure"]
impl crate::Writable for MSPND {}
#[doc = "Message Pending Register"]
pub mod mspnd;
#[doc = "Message Index Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msid](msid) module"]
pub type MSID = crate::Reg<u32, _MSID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSID;
#[doc = "`read()` method returns [msid::R](msid::R) reader structure"]
impl crate::Readable for MSID {}
#[doc = "Message Index Register"]
pub mod msid;
#[doc = "Message Index Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msimask](msimask) module"]
pub type MSIMASK = crate::Reg<u32, _MSIMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSIMASK;
#[doc = "`read()` method returns [msimask::R](msimask::R) reader structure"]
impl crate::Readable for MSIMASK {}
#[doc = "`write(|w| ..)` method takes [msimask::W](msimask::W) writer structure"]
impl crate::Writable for MSIMASK {}
#[doc = "Message Index Mask Register"]
pub mod msimask;
#[doc = "Panel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [panctr](panctr) module"]
pub type PANCTR = crate::Reg<u32, _PANCTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PANCTR;
#[doc = "`read()` method returns [panctr::R](panctr::R) reader structure"]
impl crate::Readable for PANCTR {}
#[doc = "`write(|w| ..)` method takes [panctr::W](panctr::W) writer structure"]
impl crate::Writable for PANCTR {}
#[doc = "Panel Control Register"]
pub mod panctr;
#[doc = "Module Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](mcr) module"]
pub type MCR = crate::Reg<u32, _MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR;
#[doc = "`read()` method returns [mcr::R](mcr::R) reader structure"]
impl crate::Readable for MCR {}
#[doc = "`write(|w| ..)` method takes [mcr::W](mcr::W) writer structure"]
impl crate::Writable for MCR {}
#[doc = "Module Control Register"]
pub mod mcr;
#[doc = "Module Interrupt Trigger Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mitr](mitr) module"]
pub type MITR = crate::Reg<u32, _MITR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MITR;
#[doc = "`write(|w| ..)` method takes [mitr::W](mitr::W) writer structure"]
impl crate::Writable for MITR {}
#[doc = "Module Interrupt Trigger Register"]
pub mod mitr;

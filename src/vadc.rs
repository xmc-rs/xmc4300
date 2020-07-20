#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Control Register"]
    pub clc: CLC,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Module Identification Register"]
    pub id: ID,
    _reserved2: [u8; 28usize],
    #[doc = "0x28 - OCDS Control and Status Register"]
    pub ocs: OCS,
    _reserved3: [u8; 84usize],
    #[doc = "0x80 - Global Configuration Register"]
    pub globcfg: GLOBCFG,
    _reserved4: [u8; 28usize],
    #[doc = "0xa0 - Input Class Register, Global"]
    pub globiclass: [GLOBICLASS; 2],
    _reserved5: [u8; 16usize],
    #[doc = "0xb8 - Global Boundary Select Register"]
    pub globbound: GLOBBOUND,
    _reserved6: [u8; 36usize],
    #[doc = "0xe0 - Global Event Flag Register"]
    pub globeflag: GLOBEFLAG,
    _reserved7: [u8; 92usize],
    #[doc = "0x140 - Global Event Node Pointer Register"]
    pub globevnp: GLOBEVNP,
    _reserved8: [u8; 28usize],
    #[doc = "0x160 - Global Test Functions Register"]
    pub globtf: GLOBTF,
    _reserved9: [u8; 28usize],
    #[doc = "0x180 - Background Request Source Channel Select Register"]
    pub brssel: [BRSSEL; 2],
    _reserved10: [u8; 56usize],
    #[doc = "0x1c0 - Background Request Source Pending Register"]
    pub brspnd: [BRSPND; 2],
    _reserved11: [u8; 56usize],
    #[doc = "0x200 - Background Request Source Control Register"]
    pub brsctrl: BRSCTRL,
    #[doc = "0x204 - Background Request Source Mode Register"]
    pub brsmr: BRSMR,
    _reserved13: [u8; 120usize],
    #[doc = "0x280 - Global Result Control Register"]
    pub globrcr: GLOBRCR,
    _reserved14: [u8; 124usize],
    #[doc = "0x300 - Global Result Register"]
    pub globres: GLOBRES,
    _reserved15: [u8; 124usize],
    #[doc = "0x380 - Global Result Register, Debug"]
    pub globresd: GLOBRESD,
    _reserved16: [u8; 108usize],
    #[doc = "0x3f0 - External Multiplexer Select Register"]
    pub emuxsel: EMUXSEL,
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
#[doc = "OCDS Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocs](ocs) module"]
pub type OCS = crate::Reg<u32, _OCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCS;
#[doc = "`read()` method returns [ocs::R](ocs::R) reader structure"]
impl crate::Readable for OCS {}
#[doc = "`write(|w| ..)` method takes [ocs::W](ocs::W) writer structure"]
impl crate::Writable for OCS {}
#[doc = "OCDS Control and Status Register"]
pub mod ocs;
#[doc = "Global Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [globcfg](globcfg) module"]
pub type GLOBCFG = crate::Reg<u32, _GLOBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GLOBCFG;
#[doc = "`read()` method returns [globcfg::R](globcfg::R) reader structure"]
impl crate::Readable for GLOBCFG {}
#[doc = "`write(|w| ..)` method takes [globcfg::W](globcfg::W) writer structure"]
impl crate::Writable for GLOBCFG {}
#[doc = "Global Configuration Register"]
pub mod globcfg;
#[doc = "Input Class Register, Global\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [globiclass](globiclass) module"]
pub type GLOBICLASS = crate::Reg<u32, _GLOBICLASS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GLOBICLASS;
#[doc = "`read()` method returns [globiclass::R](globiclass::R) reader structure"]
impl crate::Readable for GLOBICLASS {}
#[doc = "`write(|w| ..)` method takes [globiclass::W](globiclass::W) writer structure"]
impl crate::Writable for GLOBICLASS {}
#[doc = "Input Class Register, Global"]
pub mod globiclass;
#[doc = "Global Boundary Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [globbound](globbound) module"]
pub type GLOBBOUND = crate::Reg<u32, _GLOBBOUND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GLOBBOUND;
#[doc = "`read()` method returns [globbound::R](globbound::R) reader structure"]
impl crate::Readable for GLOBBOUND {}
#[doc = "`write(|w| ..)` method takes [globbound::W](globbound::W) writer structure"]
impl crate::Writable for GLOBBOUND {}
#[doc = "Global Boundary Select Register"]
pub mod globbound;
#[doc = "Global Event Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [globeflag](globeflag) module"]
pub type GLOBEFLAG = crate::Reg<u32, _GLOBEFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GLOBEFLAG;
#[doc = "`read()` method returns [globeflag::R](globeflag::R) reader structure"]
impl crate::Readable for GLOBEFLAG {}
#[doc = "`write(|w| ..)` method takes [globeflag::W](globeflag::W) writer structure"]
impl crate::Writable for GLOBEFLAG {}
#[doc = "Global Event Flag Register"]
pub mod globeflag;
#[doc = "Global Event Node Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [globevnp](globevnp) module"]
pub type GLOBEVNP = crate::Reg<u32, _GLOBEVNP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GLOBEVNP;
#[doc = "`read()` method returns [globevnp::R](globevnp::R) reader structure"]
impl crate::Readable for GLOBEVNP {}
#[doc = "`write(|w| ..)` method takes [globevnp::W](globevnp::W) writer structure"]
impl crate::Writable for GLOBEVNP {}
#[doc = "Global Event Node Pointer Register"]
pub mod globevnp;
#[doc = "Global Test Functions Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [globtf](globtf) module"]
pub type GLOBTF = crate::Reg<u32, _GLOBTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GLOBTF;
#[doc = "`read()` method returns [globtf::R](globtf::R) reader structure"]
impl crate::Readable for GLOBTF {}
#[doc = "`write(|w| ..)` method takes [globtf::W](globtf::W) writer structure"]
impl crate::Writable for GLOBTF {}
#[doc = "Global Test Functions Register"]
pub mod globtf;
#[doc = "Background Request Source Channel Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brssel](brssel) module"]
pub type BRSSEL = crate::Reg<u32, _BRSSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRSSEL;
#[doc = "`read()` method returns [brssel::R](brssel::R) reader structure"]
impl crate::Readable for BRSSEL {}
#[doc = "`write(|w| ..)` method takes [brssel::W](brssel::W) writer structure"]
impl crate::Writable for BRSSEL {}
#[doc = "Background Request Source Channel Select Register"]
pub mod brssel;
#[doc = "Background Request Source Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brspnd](brspnd) module"]
pub type BRSPND = crate::Reg<u32, _BRSPND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRSPND;
#[doc = "`read()` method returns [brspnd::R](brspnd::R) reader structure"]
impl crate::Readable for BRSPND {}
#[doc = "`write(|w| ..)` method takes [brspnd::W](brspnd::W) writer structure"]
impl crate::Writable for BRSPND {}
#[doc = "Background Request Source Pending Register"]
pub mod brspnd;
#[doc = "Background Request Source Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brsctrl](brsctrl) module"]
pub type BRSCTRL = crate::Reg<u32, _BRSCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRSCTRL;
#[doc = "`read()` method returns [brsctrl::R](brsctrl::R) reader structure"]
impl crate::Readable for BRSCTRL {}
#[doc = "`write(|w| ..)` method takes [brsctrl::W](brsctrl::W) writer structure"]
impl crate::Writable for BRSCTRL {}
#[doc = "Background Request Source Control Register"]
pub mod brsctrl;
#[doc = "Background Request Source Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brsmr](brsmr) module"]
pub type BRSMR = crate::Reg<u32, _BRSMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRSMR;
#[doc = "`read()` method returns [brsmr::R](brsmr::R) reader structure"]
impl crate::Readable for BRSMR {}
#[doc = "`write(|w| ..)` method takes [brsmr::W](brsmr::W) writer structure"]
impl crate::Writable for BRSMR {}
#[doc = "Background Request Source Mode Register"]
pub mod brsmr;
#[doc = "Global Result Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [globrcr](globrcr) module"]
pub type GLOBRCR = crate::Reg<u32, _GLOBRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GLOBRCR;
#[doc = "`read()` method returns [globrcr::R](globrcr::R) reader structure"]
impl crate::Readable for GLOBRCR {}
#[doc = "`write(|w| ..)` method takes [globrcr::W](globrcr::W) writer structure"]
impl crate::Writable for GLOBRCR {}
#[doc = "Global Result Control Register"]
pub mod globrcr;
#[doc = "Global Result Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [globres](globres) module"]
pub type GLOBRES = crate::Reg<u32, _GLOBRES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GLOBRES;
#[doc = "`read()` method returns [globres::R](globres::R) reader structure"]
impl crate::Readable for GLOBRES {}
#[doc = "`write(|w| ..)` method takes [globres::W](globres::W) writer structure"]
impl crate::Writable for GLOBRES {}
#[doc = "Global Result Register"]
pub mod globres;
#[doc = "Global Result Register, Debug\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [globresd](globresd) module"]
pub type GLOBRESD = crate::Reg<u32, _GLOBRESD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GLOBRESD;
#[doc = "`read()` method returns [globresd::R](globresd::R) reader structure"]
impl crate::Readable for GLOBRESD {}
#[doc = "`write(|w| ..)` method takes [globresd::W](globresd::W) writer structure"]
impl crate::Writable for GLOBRESD {}
#[doc = "Global Result Register, Debug"]
pub mod globresd;
#[doc = "External Multiplexer Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emuxsel](emuxsel) module"]
pub type EMUXSEL = crate::Reg<u32, _EMUXSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMUXSEL;
#[doc = "`read()` method returns [emuxsel::R](emuxsel::R) reader structure"]
impl crate::Readable for EMUXSEL {}
#[doc = "`write(|w| ..)` method takes [emuxsel::W](emuxsel::W) writer structure"]
impl crate::Writable for EMUXSEL {}
#[doc = "External Multiplexer Select Register"]
pub mod emuxsel;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global Control Register"]
    pub gctrl: GCTRL,
    #[doc = "0x04 - Global Status Register"]
    pub gstat: GSTAT,
    #[doc = "0x08 - Global Idle Set"]
    pub gidls: GIDLS,
    #[doc = "0x0c - Global Idle Clear"]
    pub gidlc: GIDLC,
    #[doc = "0x10 - Global Channel Set"]
    pub gcss: GCSS,
    #[doc = "0x14 - Global Channel Clear"]
    pub gcsc: GCSC,
    #[doc = "0x18 - Global Channel status"]
    pub gcst: GCST,
    #[doc = "0x1c - Parity Checker Configuration"]
    pub gpchk: GPCHK,
    _reserved8: [u8; 96usize],
    #[doc = "0x80 - Module Identification"]
    pub midr: MIDR,
}
#[doc = "Global Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gctrl](gctrl) module"]
pub type GCTRL = crate::Reg<u32, _GCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCTRL;
#[doc = "`read()` method returns [gctrl::R](gctrl::R) reader structure"]
impl crate::Readable for GCTRL {}
#[doc = "`write(|w| ..)` method takes [gctrl::W](gctrl::W) writer structure"]
impl crate::Writable for GCTRL {}
#[doc = "Global Control Register"]
pub mod gctrl;
#[doc = "Global Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gstat](gstat) module"]
pub type GSTAT = crate::Reg<u32, _GSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GSTAT;
#[doc = "`read()` method returns [gstat::R](gstat::R) reader structure"]
impl crate::Readable for GSTAT {}
#[doc = "Global Status Register"]
pub mod gstat;
#[doc = "Global Idle Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gidls](gidls) module"]
pub type GIDLS = crate::Reg<u32, _GIDLS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GIDLS;
#[doc = "`write(|w| ..)` method takes [gidls::W](gidls::W) writer structure"]
impl crate::Writable for GIDLS {}
#[doc = "Global Idle Set"]
pub mod gidls;
#[doc = "Global Idle Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gidlc](gidlc) module"]
pub type GIDLC = crate::Reg<u32, _GIDLC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GIDLC;
#[doc = "`write(|w| ..)` method takes [gidlc::W](gidlc::W) writer structure"]
impl crate::Writable for GIDLC {}
#[doc = "Global Idle Clear"]
pub mod gidlc;
#[doc = "Global Channel Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcss](gcss) module"]
pub type GCSS = crate::Reg<u32, _GCSS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCSS;
#[doc = "`write(|w| ..)` method takes [gcss::W](gcss::W) writer structure"]
impl crate::Writable for GCSS {}
#[doc = "Global Channel Set"]
pub mod gcss;
#[doc = "Global Channel Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcsc](gcsc) module"]
pub type GCSC = crate::Reg<u32, _GCSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCSC;
#[doc = "`write(|w| ..)` method takes [gcsc::W](gcsc::W) writer structure"]
impl crate::Writable for GCSC {}
#[doc = "Global Channel Clear"]
pub mod gcsc;
#[doc = "Global Channel status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcst](gcst) module"]
pub type GCST = crate::Reg<u32, _GCST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCST;
#[doc = "`read()` method returns [gcst::R](gcst::R) reader structure"]
impl crate::Readable for GCST {}
#[doc = "Global Channel status"]
pub mod gcst;
#[doc = "Parity Checker Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpchk](gpchk) module"]
pub type GPCHK = crate::Reg<u32, _GPCHK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPCHK;
#[doc = "`read()` method returns [gpchk::R](gpchk::R) reader structure"]
impl crate::Readable for GPCHK {}
#[doc = "`write(|w| ..)` method takes [gpchk::W](gpchk::W) writer structure"]
impl crate::Writable for GPCHK {}
#[doc = "Parity Checker Configuration"]
pub mod gpchk;
#[doc = "Module Identification\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [midr](midr) module"]
pub type MIDR = crate::Reg<u32, _MIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIDR;
#[doc = "`read()` method returns [midr::R](midr::R) reader structure"]
impl crate::Readable for MIDR {}
#[doc = "Module Identification"]
pub mod midr;

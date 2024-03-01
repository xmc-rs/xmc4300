#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    srstat: Srstat,
    srraw: Srraw,
    srmsk: Srmsk,
    srclr: Srclr,
    srset: Srset,
    nmireqen: Nmireqen,
}
impl RegisterBlock {
    #[doc = "0x00 - SCU Service Request Status"]
    #[inline(always)]
    pub const fn srstat(&self) -> &Srstat {
        &self.srstat
    }
    #[doc = "0x04 - SCU Raw Service Request Status"]
    #[inline(always)]
    pub const fn srraw(&self) -> &Srraw {
        &self.srraw
    }
    #[doc = "0x08 - SCU Service Request Mask"]
    #[inline(always)]
    pub const fn srmsk(&self) -> &Srmsk {
        &self.srmsk
    }
    #[doc = "0x0c - SCU Service Request Clear"]
    #[inline(always)]
    pub const fn srclr(&self) -> &Srclr {
        &self.srclr
    }
    #[doc = "0x10 - SCU Service Request Set"]
    #[inline(always)]
    pub const fn srset(&self) -> &Srset {
        &self.srset
    }
    #[doc = "0x14 - SCU Service Request Mask"]
    #[inline(always)]
    pub const fn nmireqen(&self) -> &Nmireqen {
        &self.nmireqen
    }
}
#[doc = "SRSTAT (r) register accessor: SCU Service Request Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srstat`]
module"]
#[doc(alias = "SRSTAT")]
pub type Srstat = crate::Reg<srstat::SrstatSpec>;
#[doc = "SCU Service Request Status"]
pub mod srstat;
#[doc = "SRRAW (r) register accessor: SCU Raw Service Request Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srraw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srraw`]
module"]
#[doc(alias = "SRRAW")]
pub type Srraw = crate::Reg<srraw::SrrawSpec>;
#[doc = "SCU Raw Service Request Status"]
pub mod srraw;
#[doc = "SRMSK (rw) register accessor: SCU Service Request Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srmsk`]
module"]
#[doc(alias = "SRMSK")]
pub type Srmsk = crate::Reg<srmsk::SrmskSpec>;
#[doc = "SCU Service Request Mask"]
pub mod srmsk;
#[doc = "SRCLR (w) register accessor: SCU Service Request Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srclr`]
module"]
#[doc(alias = "SRCLR")]
pub type Srclr = crate::Reg<srclr::SrclrSpec>;
#[doc = "SCU Service Request Clear"]
pub mod srclr;
#[doc = "SRSET (w) register accessor: SCU Service Request Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srset`]
module"]
#[doc(alias = "SRSET")]
pub type Srset = crate::Reg<srset::SrsetSpec>;
#[doc = "SCU Service Request Set"]
pub mod srset;
#[doc = "NMIREQEN (rw) register accessor: SCU Service Request Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nmireqen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nmireqen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmireqen`]
module"]
#[doc(alias = "NMIREQEN")]
pub type Nmireqen = crate::Reg<nmireqen::NmireqenSpec>;
#[doc = "SCU Service Request Mask"]
pub mod nmireqen;

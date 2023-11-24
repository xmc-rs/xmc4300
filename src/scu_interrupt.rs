#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    srstat: SRSTAT,
    srraw: SRRAW,
    srmsk: SRMSK,
    srclr: SRCLR,
    srset: SRSET,
    nmireqen: NMIREQEN,
}
impl RegisterBlock {
    #[doc = "0x00 - SCU Service Request Status"]
    #[inline(always)]
    pub const fn srstat(&self) -> &SRSTAT {
        &self.srstat
    }
    #[doc = "0x04 - SCU Raw Service Request Status"]
    #[inline(always)]
    pub const fn srraw(&self) -> &SRRAW {
        &self.srraw
    }
    #[doc = "0x08 - SCU Service Request Mask"]
    #[inline(always)]
    pub const fn srmsk(&self) -> &SRMSK {
        &self.srmsk
    }
    #[doc = "0x0c - SCU Service Request Clear"]
    #[inline(always)]
    pub const fn srclr(&self) -> &SRCLR {
        &self.srclr
    }
    #[doc = "0x10 - SCU Service Request Set"]
    #[inline(always)]
    pub const fn srset(&self) -> &SRSET {
        &self.srset
    }
    #[doc = "0x14 - SCU Service Request Mask"]
    #[inline(always)]
    pub const fn nmireqen(&self) -> &NMIREQEN {
        &self.nmireqen
    }
}
#[doc = "SRSTAT (r) register accessor: SCU Service Request Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srstat`]
module"]
pub type SRSTAT = crate::Reg<srstat::SRSTAT_SPEC>;
#[doc = "SCU Service Request Status"]
pub mod srstat;
#[doc = "SRRAW (r) register accessor: SCU Raw Service Request Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srraw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srraw`]
module"]
pub type SRRAW = crate::Reg<srraw::SRRAW_SPEC>;
#[doc = "SCU Raw Service Request Status"]
pub mod srraw;
#[doc = "SRMSK (rw) register accessor: SCU Service Request Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srmsk`]
module"]
pub type SRMSK = crate::Reg<srmsk::SRMSK_SPEC>;
#[doc = "SCU Service Request Mask"]
pub mod srmsk;
#[doc = "SRCLR (w) register accessor: SCU Service Request Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srclr`]
module"]
pub type SRCLR = crate::Reg<srclr::SRCLR_SPEC>;
#[doc = "SCU Service Request Clear"]
pub mod srclr;
#[doc = "SRSET (w) register accessor: SCU Service Request Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srset`]
module"]
pub type SRSET = crate::Reg<srset::SRSET_SPEC>;
#[doc = "SCU Service Request Set"]
pub mod srset;
#[doc = "NMIREQEN (rw) register accessor: SCU Service Request Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nmireqen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nmireqen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmireqen`]
module"]
pub type NMIREQEN = crate::Reg<nmireqen::NMIREQEN_SPEC>;
#[doc = "SCU Service Request Mask"]
pub mod nmireqen;

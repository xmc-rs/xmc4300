#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sar: SAR,
    _reserved1: [u8; 0x04],
    dar: DAR,
    _reserved2: [u8; 0x04],
    llp: LLP,
    _reserved3: [u8; 0x04],
    ctll: CTLL,
    ctlh: CTLH,
    sstat: SSTAT,
    _reserved6: [u8; 0x04],
    dstat: DSTAT,
    _reserved7: [u8; 0x04],
    sstatar: SSTATAR,
    _reserved8: [u8; 0x04],
    dstatar: DSTATAR,
    _reserved9: [u8; 0x04],
    cfgl: CFGL,
    cfgh: CFGH,
    sgr: SGR,
    _reserved12: [u8; 0x04],
    dsr: DSR,
}
impl RegisterBlock {
    #[doc = "0x00 - Source Address Register"]
    #[inline(always)]
    pub const fn sar(&self) -> &SAR {
        &self.sar
    }
    #[doc = "0x08 - Destination Address Register"]
    #[inline(always)]
    pub const fn dar(&self) -> &DAR {
        &self.dar
    }
    #[doc = "0x10 - Linked List Pointer Register"]
    #[inline(always)]
    pub const fn llp(&self) -> &LLP {
        &self.llp
    }
    #[doc = "0x18 - Control Register Low"]
    #[inline(always)]
    pub const fn ctll(&self) -> &CTLL {
        &self.ctll
    }
    #[doc = "0x1c - Control Register High"]
    #[inline(always)]
    pub const fn ctlh(&self) -> &CTLH {
        &self.ctlh
    }
    #[doc = "0x20 - Source Status Register"]
    #[inline(always)]
    pub const fn sstat(&self) -> &SSTAT {
        &self.sstat
    }
    #[doc = "0x28 - Destination Status Register"]
    #[inline(always)]
    pub const fn dstat(&self) -> &DSTAT {
        &self.dstat
    }
    #[doc = "0x30 - Source Status Address Register"]
    #[inline(always)]
    pub const fn sstatar(&self) -> &SSTATAR {
        &self.sstatar
    }
    #[doc = "0x38 - Destination Status Address Register"]
    #[inline(always)]
    pub const fn dstatar(&self) -> &DSTATAR {
        &self.dstatar
    }
    #[doc = "0x40 - Configuration Register Low"]
    #[inline(always)]
    pub const fn cfgl(&self) -> &CFGL {
        &self.cfgl
    }
    #[doc = "0x44 - Configuration Register High"]
    #[inline(always)]
    pub const fn cfgh(&self) -> &CFGH {
        &self.cfgh
    }
    #[doc = "0x48 - Source Gather Register"]
    #[inline(always)]
    pub const fn sgr(&self) -> &SGR {
        &self.sgr
    }
    #[doc = "0x50 - Destination Scatter Register"]
    #[inline(always)]
    pub const fn dsr(&self) -> &DSR {
        &self.dsr
    }
}
#[doc = "SAR (rw) register accessor: Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar`]
module"]
pub type SAR = crate::Reg<sar::SAR_SPEC>;
#[doc = "Source Address Register"]
pub mod sar;
#[doc = "DAR (rw) register accessor: Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dar`]
module"]
pub type DAR = crate::Reg<dar::DAR_SPEC>;
#[doc = "Destination Address Register"]
pub mod dar;
#[doc = "LLP (rw) register accessor: Linked List Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`llp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@llp`]
module"]
pub type LLP = crate::Reg<llp::LLP_SPEC>;
#[doc = "Linked List Pointer Register"]
pub mod llp;
#[doc = "CTLL (rw) register accessor: Control Register Low\n\nYou can [`read`](crate::Reg::read) this register and get [`ctll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctll`]
module"]
pub type CTLL = crate::Reg<ctll::CTLL_SPEC>;
#[doc = "Control Register Low"]
pub mod ctll;
#[doc = "CTLH (rw) register accessor: Control Register High\n\nYou can [`read`](crate::Reg::read) this register and get [`ctlh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctlh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlh`]
module"]
pub type CTLH = crate::Reg<ctlh::CTLH_SPEC>;
#[doc = "Control Register High"]
pub mod ctlh;
#[doc = "SSTAT (rw) register accessor: Source Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstat`]
module"]
pub type SSTAT = crate::Reg<sstat::SSTAT_SPEC>;
#[doc = "Source Status Register"]
pub mod sstat;
#[doc = "DSTAT (rw) register accessor: Destination Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstat`]
module"]
pub type DSTAT = crate::Reg<dstat::DSTAT_SPEC>;
#[doc = "Destination Status Register"]
pub mod dstat;
#[doc = "SSTATAR (rw) register accessor: Source Status Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sstatar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstatar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstatar`]
module"]
pub type SSTATAR = crate::Reg<sstatar::SSTATAR_SPEC>;
#[doc = "Source Status Address Register"]
pub mod sstatar;
#[doc = "DSTATAR (rw) register accessor: Destination Status Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dstatar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstatar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstatar`]
module"]
pub type DSTATAR = crate::Reg<dstatar::DSTATAR_SPEC>;
#[doc = "Destination Status Address Register"]
pub mod dstatar;
#[doc = "CFGL (rw) register accessor: Configuration Register Low\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgl`]
module"]
pub type CFGL = crate::Reg<cfgl::CFGL_SPEC>;
#[doc = "Configuration Register Low"]
pub mod cfgl;
#[doc = "CFGH (rw) register accessor: Configuration Register High\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgh`]
module"]
pub type CFGH = crate::Reg<cfgh::CFGH_SPEC>;
#[doc = "Configuration Register High"]
pub mod cfgh;
#[doc = "SGR (rw) register accessor: Source Gather Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgr`]
module"]
pub type SGR = crate::Reg<sgr::SGR_SPEC>;
#[doc = "Source Gather Register"]
pub mod sgr;
#[doc = "DSR (rw) register accessor: Destination Scatter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsr`]
module"]
pub type DSR = crate::Reg<dsr::DSR_SPEC>;
#[doc = "Destination Scatter Register"]
pub mod dsr;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gctrl: GCTRL,
    gstat: GSTAT,
    gidls: GIDLS,
    gidlc: GIDLC,
    gcss: GCSS,
    gcsc: GCSC,
    gcst: GCST,
    _reserved7: [u8; 0x64],
    midr: MIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - Global Control Register"]
    #[inline(always)]
    pub const fn gctrl(&self) -> &GCTRL {
        &self.gctrl
    }
    #[doc = "0x04 - Global Status Register"]
    #[inline(always)]
    pub const fn gstat(&self) -> &GSTAT {
        &self.gstat
    }
    #[doc = "0x08 - Global Idle Set"]
    #[inline(always)]
    pub const fn gidls(&self) -> &GIDLS {
        &self.gidls
    }
    #[doc = "0x0c - Global Idle Clear"]
    #[inline(always)]
    pub const fn gidlc(&self) -> &GIDLC {
        &self.gidlc
    }
    #[doc = "0x10 - Global Channel Set"]
    #[inline(always)]
    pub const fn gcss(&self) -> &GCSS {
        &self.gcss
    }
    #[doc = "0x14 - Global Channel Clear"]
    #[inline(always)]
    pub const fn gcsc(&self) -> &GCSC {
        &self.gcsc
    }
    #[doc = "0x18 - Global Channel Status"]
    #[inline(always)]
    pub const fn gcst(&self) -> &GCST {
        &self.gcst
    }
    #[doc = "0x80 - Module Identification"]
    #[inline(always)]
    pub const fn midr(&self) -> &MIDR {
        &self.midr
    }
}
#[doc = "GCTRL (rw) register accessor: Global Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gctrl`]
module"]
pub type GCTRL = crate::Reg<gctrl::GCTRL_SPEC>;
#[doc = "Global Control Register"]
pub mod gctrl;
#[doc = "GSTAT (r) register accessor: Global Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gstat`]
module"]
pub type GSTAT = crate::Reg<gstat::GSTAT_SPEC>;
#[doc = "Global Status Register"]
pub mod gstat;
#[doc = "GIDLS (w) register accessor: Global Idle Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gidls::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gidls`]
module"]
pub type GIDLS = crate::Reg<gidls::GIDLS_SPEC>;
#[doc = "Global Idle Set"]
pub mod gidls;
#[doc = "GIDLC (w) register accessor: Global Idle Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gidlc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gidlc`]
module"]
pub type GIDLC = crate::Reg<gidlc::GIDLC_SPEC>;
#[doc = "Global Idle Clear"]
pub mod gidlc;
#[doc = "GCSS (w) register accessor: Global Channel Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcss::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcss`]
module"]
pub type GCSS = crate::Reg<gcss::GCSS_SPEC>;
#[doc = "Global Channel Set"]
pub mod gcss;
#[doc = "GCSC (w) register accessor: Global Channel Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcsc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcsc`]
module"]
pub type GCSC = crate::Reg<gcsc::GCSC_SPEC>;
#[doc = "Global Channel Clear"]
pub mod gcsc;
#[doc = "GCST (r) register accessor: Global Channel Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcst`]
module"]
pub type GCST = crate::Reg<gcst::GCST_SPEC>;
#[doc = "Global Channel Status"]
pub mod gcst;
#[doc = "MIDR (r) register accessor: Module Identification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`midr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@midr`]
module"]
pub type MIDR = crate::Reg<midr::MIDR_SPEC>;
#[doc = "Module Identification"]
pub mod midr;

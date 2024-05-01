#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sar: Sar,
    _reserved1: [u8; 0x04],
    dar: Dar,
    _reserved2: [u8; 0x04],
    llp: Llp,
    _reserved3: [u8; 0x04],
    ctll: Ctll,
    ctlh: Ctlh,
    sstat: Sstat,
    _reserved6: [u8; 0x04],
    dstat: Dstat,
    _reserved7: [u8; 0x04],
    sstatar: Sstatar,
    _reserved8: [u8; 0x04],
    dstatar: Dstatar,
    _reserved9: [u8; 0x04],
    cfgl: Cfgl,
    cfgh: Cfgh,
    sgr: Sgr,
    _reserved12: [u8; 0x04],
    dsr: Dsr,
}
impl RegisterBlock {
    #[doc = "0x00 - Source Address Register"]
    #[inline(always)]
    pub const fn sar(&self) -> &Sar {
        &self.sar
    }
    #[doc = "0x08 - Destination Address Register"]
    #[inline(always)]
    pub const fn dar(&self) -> &Dar {
        &self.dar
    }
    #[doc = "0x10 - Linked List Pointer Register"]
    #[inline(always)]
    pub const fn llp(&self) -> &Llp {
        &self.llp
    }
    #[doc = "0x18 - Control Register Low"]
    #[inline(always)]
    pub const fn ctll(&self) -> &Ctll {
        &self.ctll
    }
    #[doc = "0x1c - Control Register High"]
    #[inline(always)]
    pub const fn ctlh(&self) -> &Ctlh {
        &self.ctlh
    }
    #[doc = "0x20 - Source Status Register"]
    #[inline(always)]
    pub const fn sstat(&self) -> &Sstat {
        &self.sstat
    }
    #[doc = "0x28 - Destination Status Register"]
    #[inline(always)]
    pub const fn dstat(&self) -> &Dstat {
        &self.dstat
    }
    #[doc = "0x30 - Source Status Address Register"]
    #[inline(always)]
    pub const fn sstatar(&self) -> &Sstatar {
        &self.sstatar
    }
    #[doc = "0x38 - Destination Status Address Register"]
    #[inline(always)]
    pub const fn dstatar(&self) -> &Dstatar {
        &self.dstatar
    }
    #[doc = "0x40 - Configuration Register Low"]
    #[inline(always)]
    pub const fn cfgl(&self) -> &Cfgl {
        &self.cfgl
    }
    #[doc = "0x44 - Configuration Register High"]
    #[inline(always)]
    pub const fn cfgh(&self) -> &Cfgh {
        &self.cfgh
    }
    #[doc = "0x48 - Source Gather Register"]
    #[inline(always)]
    pub const fn sgr(&self) -> &Sgr {
        &self.sgr
    }
    #[doc = "0x50 - Destination Scatter Register"]
    #[inline(always)]
    pub const fn dsr(&self) -> &Dsr {
        &self.dsr
    }
}
#[doc = "SAR (rw) register accessor: Source Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar`]
module"]
#[doc(alias = "SAR")]
pub type Sar = crate::Reg<sar::SarSpec>;
#[doc = "Source Address Register"]
pub mod sar;
#[doc = "DAR (rw) register accessor: Destination Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dar`]
module"]
#[doc(alias = "DAR")]
pub type Dar = crate::Reg<dar::DarSpec>;
#[doc = "Destination Address Register"]
pub mod dar;
#[doc = "LLP (rw) register accessor: Linked List Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`llp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`llp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@llp`]
module"]
#[doc(alias = "LLP")]
pub type Llp = crate::Reg<llp::LlpSpec>;
#[doc = "Linked List Pointer Register"]
pub mod llp;
#[doc = "CTLL (rw) register accessor: Control Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctll`]
module"]
#[doc(alias = "CTLL")]
pub type Ctll = crate::Reg<ctll::CtllSpec>;
#[doc = "Control Register Low"]
pub mod ctll;
#[doc = "CTLH (rw) register accessor: Control Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlh`]
module"]
#[doc(alias = "CTLH")]
pub type Ctlh = crate::Reg<ctlh::CtlhSpec>;
#[doc = "Control Register High"]
pub mod ctlh;
#[doc = "SSTAT (rw) register accessor: Source Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstat`]
module"]
#[doc(alias = "SSTAT")]
pub type Sstat = crate::Reg<sstat::SstatSpec>;
#[doc = "Source Status Register"]
pub mod sstat;
#[doc = "DSTAT (rw) register accessor: Destination Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstat`]
module"]
#[doc(alias = "DSTAT")]
pub type Dstat = crate::Reg<dstat::DstatSpec>;
#[doc = "Destination Status Register"]
pub mod dstat;
#[doc = "SSTATAR (rw) register accessor: Source Status Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sstatar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sstatar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstatar`]
module"]
#[doc(alias = "SSTATAR")]
pub type Sstatar = crate::Reg<sstatar::SstatarSpec>;
#[doc = "Source Status Address Register"]
pub mod sstatar;
#[doc = "DSTATAR (rw) register accessor: Destination Status Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dstatar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dstatar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstatar`]
module"]
#[doc(alias = "DSTATAR")]
pub type Dstatar = crate::Reg<dstatar::DstatarSpec>;
#[doc = "Destination Status Address Register"]
pub mod dstatar;
#[doc = "CFGL (rw) register accessor: Configuration Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgl`]
module"]
#[doc(alias = "CFGL")]
pub type Cfgl = crate::Reg<cfgl::CfglSpec>;
#[doc = "Configuration Register Low"]
pub mod cfgl;
#[doc = "CFGH (rw) register accessor: Configuration Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgh`]
module"]
#[doc(alias = "CFGH")]
pub type Cfgh = crate::Reg<cfgh::CfghSpec>;
#[doc = "Configuration Register High"]
pub mod cfgh;
#[doc = "SGR (rw) register accessor: Source Gather Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgr`]
module"]
#[doc(alias = "SGR")]
pub type Sgr = crate::Reg<sgr::SgrSpec>;
#[doc = "Source Gather Register"]
pub mod sgr;
#[doc = "DSR (rw) register accessor: Destination Scatter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsr`]
module"]
#[doc(alias = "DSR")]
pub type Dsr = crate::Reg<dsr::DsrSpec>;
#[doc = "Destination Scatter Register"]
pub mod dsr;

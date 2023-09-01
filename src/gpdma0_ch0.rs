#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Source Address Register"]
    pub sar: SAR,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Destination Address Register"]
    pub dar: DAR,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - Linked List Pointer Register"]
    pub llp: LLP,
    _reserved3: [u8; 0x04],
    #[doc = "0x18 - Control Register Low"]
    pub ctll: CTLL,
    #[doc = "0x1c - Control Register High"]
    pub ctlh: CTLH,
    #[doc = "0x20 - Source Status Register"]
    pub sstat: SSTAT,
    _reserved6: [u8; 0x04],
    #[doc = "0x28 - Destination Status Register"]
    pub dstat: DSTAT,
    _reserved7: [u8; 0x04],
    #[doc = "0x30 - Source Status Address Register"]
    pub sstatar: SSTATAR,
    _reserved8: [u8; 0x04],
    #[doc = "0x38 - Destination Status Address Register"]
    pub dstatar: DSTATAR,
    _reserved9: [u8; 0x04],
    #[doc = "0x40 - Configuration Register Low"]
    pub cfgl: CFGL,
    #[doc = "0x44 - Configuration Register High"]
    pub cfgh: CFGH,
    #[doc = "0x48 - Source Gather Register"]
    pub sgr: SGR,
    _reserved12: [u8; 0x04],
    #[doc = "0x50 - Destination Scatter Register"]
    pub dsr: DSR,
}
#[doc = "SAR (rw) register accessor: Source Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar`]
module"]
pub type SAR = crate::Reg<sar::SAR_SPEC>;
#[doc = "Source Address Register"]
pub mod sar;
#[doc = "DAR (rw) register accessor: Destination Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dar`]
module"]
pub type DAR = crate::Reg<dar::DAR_SPEC>;
#[doc = "Destination Address Register"]
pub mod dar;
#[doc = "LLP (rw) register accessor: Linked List Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`llp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`llp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`llp`]
module"]
pub type LLP = crate::Reg<llp::LLP_SPEC>;
#[doc = "Linked List Pointer Register"]
pub mod llp;
#[doc = "CTLL (rw) register accessor: Control Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctll`]
module"]
pub type CTLL = crate::Reg<ctll::CTLL_SPEC>;
#[doc = "Control Register Low"]
pub mod ctll;
#[doc = "CTLH (rw) register accessor: Control Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctlh`]
module"]
pub type CTLH = crate::Reg<ctlh::CTLH_SPEC>;
#[doc = "Control Register High"]
pub mod ctlh;
#[doc = "SSTAT (rw) register accessor: Source Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sstat`]
module"]
pub type SSTAT = crate::Reg<sstat::SSTAT_SPEC>;
#[doc = "Source Status Register"]
pub mod sstat;
#[doc = "DSTAT (rw) register accessor: Destination Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dstat`]
module"]
pub type DSTAT = crate::Reg<dstat::DSTAT_SPEC>;
#[doc = "Destination Status Register"]
pub mod dstat;
#[doc = "SSTATAR (rw) register accessor: Source Status Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sstatar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sstatar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sstatar`]
module"]
pub type SSTATAR = crate::Reg<sstatar::SSTATAR_SPEC>;
#[doc = "Source Status Address Register"]
pub mod sstatar;
#[doc = "DSTATAR (rw) register accessor: Destination Status Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dstatar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dstatar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dstatar`]
module"]
pub type DSTATAR = crate::Reg<dstatar::DSTATAR_SPEC>;
#[doc = "Destination Status Address Register"]
pub mod dstatar;
#[doc = "CFGL (rw) register accessor: Configuration Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cfgl`]
module"]
pub type CFGL = crate::Reg<cfgl::CFGL_SPEC>;
#[doc = "Configuration Register Low"]
pub mod cfgl;
#[doc = "CFGH (rw) register accessor: Configuration Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cfgh`]
module"]
pub type CFGH = crate::Reg<cfgh::CFGH_SPEC>;
#[doc = "Configuration Register High"]
pub mod cfgh;
#[doc = "SGR (rw) register accessor: Source Gather Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sgr`]
module"]
pub type SGR = crate::Reg<sgr::SGR_SPEC>;
#[doc = "Source Gather Register"]
pub mod sgr;
#[doc = "DSR (rw) register accessor: Destination Scatter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsr`]
module"]
pub type DSR = crate::Reg<dsr::DSR_SPEC>;
#[doc = "Destination Scatter Register"]
pub mod dsr;

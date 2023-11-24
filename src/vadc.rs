#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    clc: CLC,
    _reserved1: [u8; 0x04],
    id: ID,
    _reserved2: [u8; 0x1c],
    ocs: OCS,
    _reserved3: [u8; 0x54],
    globcfg: GLOBCFG,
    _reserved4: [u8; 0x1c],
    globiclass: [GLOBICLASS; 2],
    _reserved5: [u8; 0x10],
    globbound: GLOBBOUND,
    _reserved6: [u8; 0x24],
    globeflag: GLOBEFLAG,
    _reserved7: [u8; 0x5c],
    globevnp: GLOBEVNP,
    _reserved8: [u8; 0x1c],
    globtf: GLOBTF,
    _reserved9: [u8; 0x1c],
    brssel: [BRSSEL; 2],
    _reserved10: [u8; 0x38],
    brspnd: [BRSPND; 2],
    _reserved11: [u8; 0x38],
    brsctrl: BRSCTRL,
    brsmr: BRSMR,
    _reserved13: [u8; 0x78],
    globrcr: GLOBRCR,
    _reserved14: [u8; 0x7c],
    globres: GLOBRES,
    _reserved15: [u8; 0x7c],
    globresd: GLOBRESD,
    _reserved16: [u8; 0x6c],
    emuxsel: EMUXSEL,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock Control Register"]
    #[inline(always)]
    pub const fn clc(&self) -> &CLC {
        &self.clc
    }
    #[doc = "0x08 - Module Identification Register"]
    #[inline(always)]
    pub const fn id(&self) -> &ID {
        &self.id
    }
    #[doc = "0x28 - OCDS Control and Status Register"]
    #[inline(always)]
    pub const fn ocs(&self) -> &OCS {
        &self.ocs
    }
    #[doc = "0x80 - Global Configuration Register"]
    #[inline(always)]
    pub const fn globcfg(&self) -> &GLOBCFG {
        &self.globcfg
    }
    #[doc = "0xa0..0xa8 - Input Class Register, Global"]
    #[inline(always)]
    pub const fn globiclass(&self, n: usize) -> &GLOBICLASS {
        &self.globiclass[n]
    }
    #[doc = "0xb8 - Global Boundary Select Register"]
    #[inline(always)]
    pub const fn globbound(&self) -> &GLOBBOUND {
        &self.globbound
    }
    #[doc = "0xe0 - Global Event Flag Register"]
    #[inline(always)]
    pub const fn globeflag(&self) -> &GLOBEFLAG {
        &self.globeflag
    }
    #[doc = "0x140 - Global Event Node Pointer Register"]
    #[inline(always)]
    pub const fn globevnp(&self) -> &GLOBEVNP {
        &self.globevnp
    }
    #[doc = "0x160 - Global Test Functions Register"]
    #[inline(always)]
    pub const fn globtf(&self) -> &GLOBTF {
        &self.globtf
    }
    #[doc = "0x180..0x188 - Background Request Source Channel Select Register"]
    #[inline(always)]
    pub const fn brssel(&self, n: usize) -> &BRSSEL {
        &self.brssel[n]
    }
    #[doc = "0x1c0..0x1c8 - Background Request Source Pending Register"]
    #[inline(always)]
    pub const fn brspnd(&self, n: usize) -> &BRSPND {
        &self.brspnd[n]
    }
    #[doc = "0x200 - Background Request Source Control Register"]
    #[inline(always)]
    pub const fn brsctrl(&self) -> &BRSCTRL {
        &self.brsctrl
    }
    #[doc = "0x204 - Background Request Source Mode Register"]
    #[inline(always)]
    pub const fn brsmr(&self) -> &BRSMR {
        &self.brsmr
    }
    #[doc = "0x280 - Global Result Control Register"]
    #[inline(always)]
    pub const fn globrcr(&self) -> &GLOBRCR {
        &self.globrcr
    }
    #[doc = "0x300 - Global Result Register"]
    #[inline(always)]
    pub const fn globres(&self) -> &GLOBRES {
        &self.globres
    }
    #[doc = "0x380 - Global Result Register, Debug"]
    #[inline(always)]
    pub const fn globresd(&self) -> &GLOBRESD {
        &self.globresd
    }
    #[doc = "0x3f0 - External Multiplexer Select Register"]
    #[inline(always)]
    pub const fn emuxsel(&self) -> &EMUXSEL {
        &self.emuxsel
    }
}
#[doc = "CLC (rw) register accessor: Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clc`]
module"]
pub type CLC = crate::Reg<clc::CLC_SPEC>;
#[doc = "Clock Control Register"]
pub mod clc;
#[doc = "ID (r) register accessor: Module Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "Module Identification Register"]
pub mod id;
#[doc = "OCS (rw) register accessor: OCDS Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocs`]
module"]
pub type OCS = crate::Reg<ocs::OCS_SPEC>;
#[doc = "OCDS Control and Status Register"]
pub mod ocs;
#[doc = "GLOBCFG (rw) register accessor: Global Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@globcfg`]
module"]
pub type GLOBCFG = crate::Reg<globcfg::GLOBCFG_SPEC>;
#[doc = "Global Configuration Register"]
pub mod globcfg;
#[doc = "GLOBICLASS (rw) register accessor: Input Class Register, Global\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globiclass::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globiclass::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@globiclass`]
module"]
pub type GLOBICLASS = crate::Reg<globiclass::GLOBICLASS_SPEC>;
#[doc = "Input Class Register, Global"]
pub mod globiclass;
#[doc = "GLOBBOUND (rw) register accessor: Global Boundary Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globbound::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globbound::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@globbound`]
module"]
pub type GLOBBOUND = crate::Reg<globbound::GLOBBOUND_SPEC>;
#[doc = "Global Boundary Select Register"]
pub mod globbound;
#[doc = "GLOBEFLAG (rw) register accessor: Global Event Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globeflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globeflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@globeflag`]
module"]
pub type GLOBEFLAG = crate::Reg<globeflag::GLOBEFLAG_SPEC>;
#[doc = "Global Event Flag Register"]
pub mod globeflag;
#[doc = "GLOBEVNP (rw) register accessor: Global Event Node Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globevnp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globevnp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@globevnp`]
module"]
pub type GLOBEVNP = crate::Reg<globevnp::GLOBEVNP_SPEC>;
#[doc = "Global Event Node Pointer Register"]
pub mod globevnp;
#[doc = "GLOBTF (rw) register accessor: Global Test Functions Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globtf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globtf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@globtf`]
module"]
pub type GLOBTF = crate::Reg<globtf::GLOBTF_SPEC>;
#[doc = "Global Test Functions Register"]
pub mod globtf;
#[doc = "BRSSEL (rw) register accessor: Background Request Source Channel Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brssel`]
module"]
pub type BRSSEL = crate::Reg<brssel::BRSSEL_SPEC>;
#[doc = "Background Request Source Channel Select Register"]
pub mod brssel;
#[doc = "BRSPND (rw) register accessor: Background Request Source Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brspnd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brspnd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brspnd`]
module"]
pub type BRSPND = crate::Reg<brspnd::BRSPND_SPEC>;
#[doc = "Background Request Source Pending Register"]
pub mod brspnd;
#[doc = "BRSCTRL (rw) register accessor: Background Request Source Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brsctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brsctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brsctrl`]
module"]
pub type BRSCTRL = crate::Reg<brsctrl::BRSCTRL_SPEC>;
#[doc = "Background Request Source Control Register"]
pub mod brsctrl;
#[doc = "BRSMR (rw) register accessor: Background Request Source Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brsmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brsmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brsmr`]
module"]
pub type BRSMR = crate::Reg<brsmr::BRSMR_SPEC>;
#[doc = "Background Request Source Mode Register"]
pub mod brsmr;
#[doc = "GLOBRCR (rw) register accessor: Global Result Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globrcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globrcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@globrcr`]
module"]
pub type GLOBRCR = crate::Reg<globrcr::GLOBRCR_SPEC>;
#[doc = "Global Result Control Register"]
pub mod globrcr;
#[doc = "GLOBRES (rw) register accessor: Global Result Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globres::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globres::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@globres`]
module"]
pub type GLOBRES = crate::Reg<globres::GLOBRES_SPEC>;
#[doc = "Global Result Register"]
pub mod globres;
#[doc = "GLOBRESD (rw) register accessor: Global Result Register, Debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globresd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globresd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@globresd`]
module"]
pub type GLOBRESD = crate::Reg<globresd::GLOBRESD_SPEC>;
#[doc = "Global Result Register, Debug"]
pub mod globresd;
#[doc = "EMUXSEL (rw) register accessor: External Multiplexer Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emuxsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emuxsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emuxsel`]
module"]
pub type EMUXSEL = crate::Reg<emuxsel::EMUXSEL_SPEC>;
#[doc = "External Multiplexer Select Register"]
pub mod emuxsel;

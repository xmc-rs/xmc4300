#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clc: Clc,
    _reserved1: [u8; 0x04],
    id: Id,
    _reserved2: [u8; 0x1c],
    ocs: Ocs,
    _reserved3: [u8; 0x54],
    globcfg: Globcfg,
    _reserved4: [u8; 0x1c],
    globiclass: [Globiclass; 2],
    _reserved5: [u8; 0x10],
    globbound: Globbound,
    _reserved6: [u8; 0x24],
    globeflag: Globeflag,
    _reserved7: [u8; 0x5c],
    globevnp: Globevnp,
    _reserved8: [u8; 0x1c],
    globtf: Globtf,
    _reserved9: [u8; 0x1c],
    brssel: [Brssel; 2],
    _reserved10: [u8; 0x38],
    brspnd: [Brspnd; 2],
    _reserved11: [u8; 0x38],
    brsctrl: Brsctrl,
    brsmr: Brsmr,
    _reserved13: [u8; 0x78],
    globrcr: Globrcr,
    _reserved14: [u8; 0x7c],
    globres: Globres,
    _reserved15: [u8; 0x7c],
    globresd: Globresd,
    _reserved16: [u8; 0x6c],
    emuxsel: Emuxsel,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock Control Register"]
    #[inline(always)]
    pub const fn clc(&self) -> &Clc {
        &self.clc
    }
    #[doc = "0x08 - Module Identification Register"]
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
    #[doc = "0x28 - OCDS Control and Status Register"]
    #[inline(always)]
    pub const fn ocs(&self) -> &Ocs {
        &self.ocs
    }
    #[doc = "0x80 - Global Configuration Register"]
    #[inline(always)]
    pub const fn globcfg(&self) -> &Globcfg {
        &self.globcfg
    }
    #[doc = "0xa0..0xa8 - Input Class Register, Global"]
    #[inline(always)]
    pub const fn globiclass(&self, n: usize) -> &Globiclass {
        &self.globiclass[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa0..0xa8 - Input Class Register, Global"]
    #[inline(always)]
    pub fn globiclass_iter(&self) -> impl Iterator<Item = &Globiclass> {
        self.globiclass.iter()
    }
    #[doc = "0xb8 - Global Boundary Select Register"]
    #[inline(always)]
    pub const fn globbound(&self) -> &Globbound {
        &self.globbound
    }
    #[doc = "0xe0 - Global Event Flag Register"]
    #[inline(always)]
    pub const fn globeflag(&self) -> &Globeflag {
        &self.globeflag
    }
    #[doc = "0x140 - Global Event Node Pointer Register"]
    #[inline(always)]
    pub const fn globevnp(&self) -> &Globevnp {
        &self.globevnp
    }
    #[doc = "0x160 - Global Test Functions Register"]
    #[inline(always)]
    pub const fn globtf(&self) -> &Globtf {
        &self.globtf
    }
    #[doc = "0x180..0x188 - Background Request Source Channel Select Register"]
    #[inline(always)]
    pub const fn brssel(&self, n: usize) -> &Brssel {
        &self.brssel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x180..0x188 - Background Request Source Channel Select Register"]
    #[inline(always)]
    pub fn brssel_iter(&self) -> impl Iterator<Item = &Brssel> {
        self.brssel.iter()
    }
    #[doc = "0x1c0..0x1c8 - Background Request Source Pending Register"]
    #[inline(always)]
    pub const fn brspnd(&self, n: usize) -> &Brspnd {
        &self.brspnd[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c0..0x1c8 - Background Request Source Pending Register"]
    #[inline(always)]
    pub fn brspnd_iter(&self) -> impl Iterator<Item = &Brspnd> {
        self.brspnd.iter()
    }
    #[doc = "0x200 - Background Request Source Control Register"]
    #[inline(always)]
    pub const fn brsctrl(&self) -> &Brsctrl {
        &self.brsctrl
    }
    #[doc = "0x204 - Background Request Source Mode Register"]
    #[inline(always)]
    pub const fn brsmr(&self) -> &Brsmr {
        &self.brsmr
    }
    #[doc = "0x280 - Global Result Control Register"]
    #[inline(always)]
    pub const fn globrcr(&self) -> &Globrcr {
        &self.globrcr
    }
    #[doc = "0x300 - Global Result Register"]
    #[inline(always)]
    pub const fn globres(&self) -> &Globres {
        &self.globres
    }
    #[doc = "0x380 - Global Result Register, Debug"]
    #[inline(always)]
    pub const fn globresd(&self) -> &Globresd {
        &self.globresd
    }
    #[doc = "0x3f0 - External Multiplexer Select Register"]
    #[inline(always)]
    pub const fn emuxsel(&self) -> &Emuxsel {
        &self.emuxsel
    }
}
#[doc = "CLC (rw) register accessor: Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clc`]
module"]
#[doc(alias = "CLC")]
pub type Clc = crate::Reg<clc::ClcSpec>;
#[doc = "Clock Control Register"]
pub mod clc;
#[doc = "ID (r) register accessor: Module Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
#[doc(alias = "ID")]
pub type Id = crate::Reg<id::IdSpec>;
#[doc = "Module Identification Register"]
pub mod id;
#[doc = "OCS (rw) register accessor: OCDS Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocs`]
module"]
#[doc(alias = "OCS")]
pub type Ocs = crate::Reg<ocs::OcsSpec>;
#[doc = "OCDS Control and Status Register"]
pub mod ocs;
#[doc = "GLOBCFG (rw) register accessor: Global Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@globcfg`]
module"]
#[doc(alias = "GLOBCFG")]
pub type Globcfg = crate::Reg<globcfg::GlobcfgSpec>;
#[doc = "Global Configuration Register"]
pub mod globcfg;
#[doc = "GLOBICLASS (rw) register accessor: Input Class Register, Global\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globiclass::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globiclass::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@globiclass`]
module"]
#[doc(alias = "GLOBICLASS")]
pub type Globiclass = crate::Reg<globiclass::GlobiclassSpec>;
#[doc = "Input Class Register, Global"]
pub mod globiclass;
#[doc = "GLOBBOUND (rw) register accessor: Global Boundary Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globbound::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globbound::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@globbound`]
module"]
#[doc(alias = "GLOBBOUND")]
pub type Globbound = crate::Reg<globbound::GlobboundSpec>;
#[doc = "Global Boundary Select Register"]
pub mod globbound;
#[doc = "GLOBEFLAG (rw) register accessor: Global Event Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globeflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globeflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@globeflag`]
module"]
#[doc(alias = "GLOBEFLAG")]
pub type Globeflag = crate::Reg<globeflag::GlobeflagSpec>;
#[doc = "Global Event Flag Register"]
pub mod globeflag;
#[doc = "GLOBEVNP (rw) register accessor: Global Event Node Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globevnp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globevnp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@globevnp`]
module"]
#[doc(alias = "GLOBEVNP")]
pub type Globevnp = crate::Reg<globevnp::GlobevnpSpec>;
#[doc = "Global Event Node Pointer Register"]
pub mod globevnp;
#[doc = "GLOBTF (rw) register accessor: Global Test Functions Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globtf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globtf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@globtf`]
module"]
#[doc(alias = "GLOBTF")]
pub type Globtf = crate::Reg<globtf::GlobtfSpec>;
#[doc = "Global Test Functions Register"]
pub mod globtf;
#[doc = "BRSSEL (rw) register accessor: Background Request Source Channel Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brssel`]
module"]
#[doc(alias = "BRSSEL")]
pub type Brssel = crate::Reg<brssel::BrsselSpec>;
#[doc = "Background Request Source Channel Select Register"]
pub mod brssel;
#[doc = "BRSPND (rw) register accessor: Background Request Source Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brspnd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brspnd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brspnd`]
module"]
#[doc(alias = "BRSPND")]
pub type Brspnd = crate::Reg<brspnd::BrspndSpec>;
#[doc = "Background Request Source Pending Register"]
pub mod brspnd;
#[doc = "BRSCTRL (rw) register accessor: Background Request Source Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brsctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brsctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brsctrl`]
module"]
#[doc(alias = "BRSCTRL")]
pub type Brsctrl = crate::Reg<brsctrl::BrsctrlSpec>;
#[doc = "Background Request Source Control Register"]
pub mod brsctrl;
#[doc = "BRSMR (rw) register accessor: Background Request Source Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brsmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brsmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brsmr`]
module"]
#[doc(alias = "BRSMR")]
pub type Brsmr = crate::Reg<brsmr::BrsmrSpec>;
#[doc = "Background Request Source Mode Register"]
pub mod brsmr;
#[doc = "GLOBRCR (rw) register accessor: Global Result Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globrcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globrcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@globrcr`]
module"]
#[doc(alias = "GLOBRCR")]
pub type Globrcr = crate::Reg<globrcr::GlobrcrSpec>;
#[doc = "Global Result Control Register"]
pub mod globrcr;
#[doc = "GLOBRES (rw) register accessor: Global Result Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globres::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globres::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@globres`]
module"]
#[doc(alias = "GLOBRES")]
pub type Globres = crate::Reg<globres::GlobresSpec>;
#[doc = "Global Result Register"]
pub mod globres;
#[doc = "GLOBRESD (rw) register accessor: Global Result Register, Debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globresd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globresd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@globresd`]
module"]
#[doc(alias = "GLOBRESD")]
pub type Globresd = crate::Reg<globresd::GlobresdSpec>;
#[doc = "Global Result Register, Debug"]
pub mod globresd;
#[doc = "EMUXSEL (rw) register accessor: External Multiplexer Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emuxsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emuxsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emuxsel`]
module"]
#[doc(alias = "EMUXSEL")]
pub type Emuxsel = crate::Reg<emuxsel::EmuxselSpec>;
#[doc = "External Multiplexer Select Register"]
pub mod emuxsel;

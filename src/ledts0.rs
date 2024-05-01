#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    id: Id,
    globctl: Globctl,
    fnctl: Fnctl,
    evfr: Evfr,
    tsval: Tsval,
    line0: Line0,
    line1: Line1,
    ldcmp0: Ldcmp0,
    ldcmp1: Ldcmp1,
    tscmp0: Tscmp0,
    tscmp1: Tscmp1,
}
impl RegisterBlock {
    #[doc = "0x00 - Module Identification Register"]
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
    #[doc = "0x04 - Global Control Register"]
    #[inline(always)]
    pub const fn globctl(&self) -> &Globctl {
        &self.globctl
    }
    #[doc = "0x08 - Function Control Register"]
    #[inline(always)]
    pub const fn fnctl(&self) -> &Fnctl {
        &self.fnctl
    }
    #[doc = "0x0c - Event Flag Register"]
    #[inline(always)]
    pub const fn evfr(&self) -> &Evfr {
        &self.evfr
    }
    #[doc = "0x10 - Touch-sense TS-Counter Value"]
    #[inline(always)]
    pub const fn tsval(&self) -> &Tsval {
        &self.tsval
    }
    #[doc = "0x14 - Line Pattern Register 0"]
    #[inline(always)]
    pub const fn line0(&self) -> &Line0 {
        &self.line0
    }
    #[doc = "0x18 - Line Pattern Register 1"]
    #[inline(always)]
    pub const fn line1(&self) -> &Line1 {
        &self.line1
    }
    #[doc = "0x1c - LED Compare Register 0"]
    #[inline(always)]
    pub const fn ldcmp0(&self) -> &Ldcmp0 {
        &self.ldcmp0
    }
    #[doc = "0x20 - LED Compare Register 1"]
    #[inline(always)]
    pub const fn ldcmp1(&self) -> &Ldcmp1 {
        &self.ldcmp1
    }
    #[doc = "0x24 - Touch-sense Compare Register 0"]
    #[inline(always)]
    pub const fn tscmp0(&self) -> &Tscmp0 {
        &self.tscmp0
    }
    #[doc = "0x28 - Touch-sense Compare Register 1"]
    #[inline(always)]
    pub const fn tscmp1(&self) -> &Tscmp1 {
        &self.tscmp1
    }
}
#[doc = "ID (r) register accessor: Module Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
#[doc(alias = "ID")]
pub type Id = crate::Reg<id::IdSpec>;
#[doc = "Module Identification Register"]
pub mod id;
#[doc = "GLOBCTL (rw) register accessor: Global Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@globctl`]
module"]
#[doc(alias = "GLOBCTL")]
pub type Globctl = crate::Reg<globctl::GlobctlSpec>;
#[doc = "Global Control Register"]
pub mod globctl;
#[doc = "FNCTL (rw) register accessor: Function Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fnctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fnctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fnctl`]
module"]
#[doc(alias = "FNCTL")]
pub type Fnctl = crate::Reg<fnctl::FnctlSpec>;
#[doc = "Function Control Register"]
pub mod fnctl;
#[doc = "EVFR (rw) register accessor: Event Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evfr`]
module"]
#[doc(alias = "EVFR")]
pub type Evfr = crate::Reg<evfr::EvfrSpec>;
#[doc = "Event Flag Register"]
pub mod evfr;
#[doc = "TSVAL (rw) register accessor: Touch-sense TS-Counter Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsval`]
module"]
#[doc(alias = "TSVAL")]
pub type Tsval = crate::Reg<tsval::TsvalSpec>;
#[doc = "Touch-sense TS-Counter Value"]
pub mod tsval;
#[doc = "LINE0 (rw) register accessor: Line Pattern Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`line0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`line0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@line0`]
module"]
#[doc(alias = "LINE0")]
pub type Line0 = crate::Reg<line0::Line0Spec>;
#[doc = "Line Pattern Register 0"]
pub mod line0;
#[doc = "LINE1 (rw) register accessor: Line Pattern Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`line1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`line1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@line1`]
module"]
#[doc(alias = "LINE1")]
pub type Line1 = crate::Reg<line1::Line1Spec>;
#[doc = "Line Pattern Register 1"]
pub mod line1;
#[doc = "LDCMP0 (rw) register accessor: LED Compare Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ldcmp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ldcmp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldcmp0`]
module"]
#[doc(alias = "LDCMP0")]
pub type Ldcmp0 = crate::Reg<ldcmp0::Ldcmp0Spec>;
#[doc = "LED Compare Register 0"]
pub mod ldcmp0;
#[doc = "LDCMP1 (rw) register accessor: LED Compare Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ldcmp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ldcmp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldcmp1`]
module"]
#[doc(alias = "LDCMP1")]
pub type Ldcmp1 = crate::Reg<ldcmp1::Ldcmp1Spec>;
#[doc = "LED Compare Register 1"]
pub mod ldcmp1;
#[doc = "TSCMP0 (rw) register accessor: Touch-sense Compare Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscmp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscmp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscmp0`]
module"]
#[doc(alias = "TSCMP0")]
pub type Tscmp0 = crate::Reg<tscmp0::Tscmp0Spec>;
#[doc = "Touch-sense Compare Register 0"]
pub mod tscmp0;
#[doc = "TSCMP1 (rw) register accessor: Touch-sense Compare Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscmp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscmp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscmp1`]
module"]
#[doc(alias = "TSCMP1")]
pub type Tscmp1 = crate::Reg<tscmp1::Tscmp1Spec>;
#[doc = "Touch-sense Compare Register 1"]
pub mod tscmp1;

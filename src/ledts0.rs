#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    id: ID,
    globctl: GLOBCTL,
    fnctl: FNCTL,
    evfr: EVFR,
    tsval: TSVAL,
    line0: LINE0,
    line1: LINE1,
    ldcmp0: LDCMP0,
    ldcmp1: LDCMP1,
    tscmp0: TSCMP0,
    tscmp1: TSCMP1,
}
impl RegisterBlock {
    #[doc = "0x00 - Module Identification Register"]
    #[inline(always)]
    pub const fn id(&self) -> &ID {
        &self.id
    }
    #[doc = "0x04 - Global Control Register"]
    #[inline(always)]
    pub const fn globctl(&self) -> &GLOBCTL {
        &self.globctl
    }
    #[doc = "0x08 - Function Control Register"]
    #[inline(always)]
    pub const fn fnctl(&self) -> &FNCTL {
        &self.fnctl
    }
    #[doc = "0x0c - Event Flag Register"]
    #[inline(always)]
    pub const fn evfr(&self) -> &EVFR {
        &self.evfr
    }
    #[doc = "0x10 - Touch-sense TS-Counter Value"]
    #[inline(always)]
    pub const fn tsval(&self) -> &TSVAL {
        &self.tsval
    }
    #[doc = "0x14 - Line Pattern Register 0"]
    #[inline(always)]
    pub const fn line0(&self) -> &LINE0 {
        &self.line0
    }
    #[doc = "0x18 - Line Pattern Register 1"]
    #[inline(always)]
    pub const fn line1(&self) -> &LINE1 {
        &self.line1
    }
    #[doc = "0x1c - LED Compare Register 0"]
    #[inline(always)]
    pub const fn ldcmp0(&self) -> &LDCMP0 {
        &self.ldcmp0
    }
    #[doc = "0x20 - LED Compare Register 1"]
    #[inline(always)]
    pub const fn ldcmp1(&self) -> &LDCMP1 {
        &self.ldcmp1
    }
    #[doc = "0x24 - Touch-sense Compare Register 0"]
    #[inline(always)]
    pub const fn tscmp0(&self) -> &TSCMP0 {
        &self.tscmp0
    }
    #[doc = "0x28 - Touch-sense Compare Register 1"]
    #[inline(always)]
    pub const fn tscmp1(&self) -> &TSCMP1 {
        &self.tscmp1
    }
}
#[doc = "ID (r) register accessor: Module Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "Module Identification Register"]
pub mod id;
#[doc = "GLOBCTL (rw) register accessor: Global Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@globctl`]
module"]
pub type GLOBCTL = crate::Reg<globctl::GLOBCTL_SPEC>;
#[doc = "Global Control Register"]
pub mod globctl;
#[doc = "FNCTL (rw) register accessor: Function Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fnctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fnctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fnctl`]
module"]
pub type FNCTL = crate::Reg<fnctl::FNCTL_SPEC>;
#[doc = "Function Control Register"]
pub mod fnctl;
#[doc = "EVFR (rw) register accessor: Event Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evfr`]
module"]
pub type EVFR = crate::Reg<evfr::EVFR_SPEC>;
#[doc = "Event Flag Register"]
pub mod evfr;
#[doc = "TSVAL (rw) register accessor: Touch-sense TS-Counter Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsval`]
module"]
pub type TSVAL = crate::Reg<tsval::TSVAL_SPEC>;
#[doc = "Touch-sense TS-Counter Value"]
pub mod tsval;
#[doc = "LINE0 (rw) register accessor: Line Pattern Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`line0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`line0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@line0`]
module"]
pub type LINE0 = crate::Reg<line0::LINE0_SPEC>;
#[doc = "Line Pattern Register 0"]
pub mod line0;
#[doc = "LINE1 (rw) register accessor: Line Pattern Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`line1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`line1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@line1`]
module"]
pub type LINE1 = crate::Reg<line1::LINE1_SPEC>;
#[doc = "Line Pattern Register 1"]
pub mod line1;
#[doc = "LDCMP0 (rw) register accessor: LED Compare Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ldcmp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ldcmp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldcmp0`]
module"]
pub type LDCMP0 = crate::Reg<ldcmp0::LDCMP0_SPEC>;
#[doc = "LED Compare Register 0"]
pub mod ldcmp0;
#[doc = "LDCMP1 (rw) register accessor: LED Compare Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ldcmp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ldcmp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldcmp1`]
module"]
pub type LDCMP1 = crate::Reg<ldcmp1::LDCMP1_SPEC>;
#[doc = "LED Compare Register 1"]
pub mod ldcmp1;
#[doc = "TSCMP0 (rw) register accessor: Touch-sense Compare Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscmp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscmp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscmp0`]
module"]
pub type TSCMP0 = crate::Reg<tscmp0::TSCMP0_SPEC>;
#[doc = "Touch-sense Compare Register 0"]
pub mod tscmp0;
#[doc = "TSCMP1 (rw) register accessor: Touch-sense Compare Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscmp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscmp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscmp1`]
module"]
pub type TSCMP1 = crate::Reg<tscmp1::TSCMP1_SPEC>;
#[doc = "Touch-sense Compare Register 1"]
pub mod tscmp1;

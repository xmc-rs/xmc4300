#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rststat: RSTSTAT,
    rstset: RSTSET,
    rstclr: RSTCLR,
    prstat0: PRSTAT0,
    prset0: PRSET0,
    prclr0: PRCLR0,
    prstat1: PRSTAT1,
    prset1: PRSET1,
    prclr1: PRCLR1,
    prstat2: PRSTAT2,
    prset2: PRSET2,
    prclr2: PRCLR2,
}
impl RegisterBlock {
    #[doc = "0x00 - RCU Reset Status"]
    #[inline(always)]
    pub const fn rststat(&self) -> &RSTSTAT {
        &self.rststat
    }
    #[doc = "0x04 - RCU Reset Set Register"]
    #[inline(always)]
    pub const fn rstset(&self) -> &RSTSET {
        &self.rstset
    }
    #[doc = "0x08 - RCU Reset Clear Register"]
    #[inline(always)]
    pub const fn rstclr(&self) -> &RSTCLR {
        &self.rstclr
    }
    #[doc = "0x0c - RCU Peripheral 0 Reset Status"]
    #[inline(always)]
    pub const fn prstat0(&self) -> &PRSTAT0 {
        &self.prstat0
    }
    #[doc = "0x10 - RCU Peripheral 0 Reset Set"]
    #[inline(always)]
    pub const fn prset0(&self) -> &PRSET0 {
        &self.prset0
    }
    #[doc = "0x14 - RCU Peripheral 0 Reset Clear"]
    #[inline(always)]
    pub const fn prclr0(&self) -> &PRCLR0 {
        &self.prclr0
    }
    #[doc = "0x18 - RCU Peripheral 1 Reset Status"]
    #[inline(always)]
    pub const fn prstat1(&self) -> &PRSTAT1 {
        &self.prstat1
    }
    #[doc = "0x1c - RCU Peripheral 1 Reset Set"]
    #[inline(always)]
    pub const fn prset1(&self) -> &PRSET1 {
        &self.prset1
    }
    #[doc = "0x20 - RCU Peripheral 1 Reset Clear"]
    #[inline(always)]
    pub const fn prclr1(&self) -> &PRCLR1 {
        &self.prclr1
    }
    #[doc = "0x24 - RCU Peripheral 2 Reset Status"]
    #[inline(always)]
    pub const fn prstat2(&self) -> &PRSTAT2 {
        &self.prstat2
    }
    #[doc = "0x28 - RCU Peripheral 2 Reset Set"]
    #[inline(always)]
    pub const fn prset2(&self) -> &PRSET2 {
        &self.prset2
    }
    #[doc = "0x2c - RCU Peripheral 2 Reset Clear"]
    #[inline(always)]
    pub const fn prclr2(&self) -> &PRCLR2 {
        &self.prclr2
    }
}
#[doc = "RSTSTAT (r) register accessor: RCU Reset Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rststat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rststat`]
module"]
pub type RSTSTAT = crate::Reg<rststat::RSTSTAT_SPEC>;
#[doc = "RCU Reset Status"]
pub mod rststat;
#[doc = "RSTSET (w) register accessor: RCU Reset Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstset`]
module"]
pub type RSTSET = crate::Reg<rstset::RSTSET_SPEC>;
#[doc = "RCU Reset Set Register"]
pub mod rstset;
#[doc = "RSTCLR (w) register accessor: RCU Reset Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstclr`]
module"]
pub type RSTCLR = crate::Reg<rstclr::RSTCLR_SPEC>;
#[doc = "RCU Reset Clear Register"]
pub mod rstclr;
#[doc = "PRSTAT0 (r) register accessor: RCU Peripheral 0 Reset Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prstat0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prstat0`]
module"]
pub type PRSTAT0 = crate::Reg<prstat0::PRSTAT0_SPEC>;
#[doc = "RCU Peripheral 0 Reset Status"]
pub mod prstat0;
#[doc = "PRSET0 (w) register accessor: RCU Peripheral 0 Reset Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prset0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prset0`]
module"]
pub type PRSET0 = crate::Reg<prset0::PRSET0_SPEC>;
#[doc = "RCU Peripheral 0 Reset Set"]
pub mod prset0;
#[doc = "PRCLR0 (w) register accessor: RCU Peripheral 0 Reset Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prclr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prclr0`]
module"]
pub type PRCLR0 = crate::Reg<prclr0::PRCLR0_SPEC>;
#[doc = "RCU Peripheral 0 Reset Clear"]
pub mod prclr0;
#[doc = "PRSTAT1 (r) register accessor: RCU Peripheral 1 Reset Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prstat1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prstat1`]
module"]
pub type PRSTAT1 = crate::Reg<prstat1::PRSTAT1_SPEC>;
#[doc = "RCU Peripheral 1 Reset Status"]
pub mod prstat1;
#[doc = "PRSET1 (w) register accessor: RCU Peripheral 1 Reset Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prset1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prset1`]
module"]
pub type PRSET1 = crate::Reg<prset1::PRSET1_SPEC>;
#[doc = "RCU Peripheral 1 Reset Set"]
pub mod prset1;
#[doc = "PRCLR1 (w) register accessor: RCU Peripheral 1 Reset Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prclr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prclr1`]
module"]
pub type PRCLR1 = crate::Reg<prclr1::PRCLR1_SPEC>;
#[doc = "RCU Peripheral 1 Reset Clear"]
pub mod prclr1;
#[doc = "PRSTAT2 (r) register accessor: RCU Peripheral 2 Reset Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prstat2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prstat2`]
module"]
pub type PRSTAT2 = crate::Reg<prstat2::PRSTAT2_SPEC>;
#[doc = "RCU Peripheral 2 Reset Status"]
pub mod prstat2;
#[doc = "PRSET2 (w) register accessor: RCU Peripheral 2 Reset Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prset2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prset2`]
module"]
pub type PRSET2 = crate::Reg<prset2::PRSET2_SPEC>;
#[doc = "RCU Peripheral 2 Reset Set"]
pub mod prset2;
#[doc = "PRCLR2 (w) register accessor: RCU Peripheral 2 Reset Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prclr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prclr2`]
module"]
pub type PRCLR2 = crate::Reg<prclr2::PRCLR2_SPEC>;
#[doc = "RCU Peripheral 2 Reset Clear"]
pub mod prclr2;

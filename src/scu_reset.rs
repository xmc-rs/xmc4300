#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rststat: Rststat,
    rstset: Rstset,
    rstclr: Rstclr,
    prstat0: Prstat0,
    prset0: Prset0,
    prclr0: Prclr0,
    prstat1: Prstat1,
    prset1: Prset1,
    prclr1: Prclr1,
    prstat2: Prstat2,
    prset2: Prset2,
    prclr2: Prclr2,
}
impl RegisterBlock {
    #[doc = "0x00 - RCU Reset Status"]
    #[inline(always)]
    pub const fn rststat(&self) -> &Rststat {
        &self.rststat
    }
    #[doc = "0x04 - RCU Reset Set Register"]
    #[inline(always)]
    pub const fn rstset(&self) -> &Rstset {
        &self.rstset
    }
    #[doc = "0x08 - RCU Reset Clear Register"]
    #[inline(always)]
    pub const fn rstclr(&self) -> &Rstclr {
        &self.rstclr
    }
    #[doc = "0x0c - RCU Peripheral 0 Reset Status"]
    #[inline(always)]
    pub const fn prstat0(&self) -> &Prstat0 {
        &self.prstat0
    }
    #[doc = "0x10 - RCU Peripheral 0 Reset Set"]
    #[inline(always)]
    pub const fn prset0(&self) -> &Prset0 {
        &self.prset0
    }
    #[doc = "0x14 - RCU Peripheral 0 Reset Clear"]
    #[inline(always)]
    pub const fn prclr0(&self) -> &Prclr0 {
        &self.prclr0
    }
    #[doc = "0x18 - RCU Peripheral 1 Reset Status"]
    #[inline(always)]
    pub const fn prstat1(&self) -> &Prstat1 {
        &self.prstat1
    }
    #[doc = "0x1c - RCU Peripheral 1 Reset Set"]
    #[inline(always)]
    pub const fn prset1(&self) -> &Prset1 {
        &self.prset1
    }
    #[doc = "0x20 - RCU Peripheral 1 Reset Clear"]
    #[inline(always)]
    pub const fn prclr1(&self) -> &Prclr1 {
        &self.prclr1
    }
    #[doc = "0x24 - RCU Peripheral 2 Reset Status"]
    #[inline(always)]
    pub const fn prstat2(&self) -> &Prstat2 {
        &self.prstat2
    }
    #[doc = "0x28 - RCU Peripheral 2 Reset Set"]
    #[inline(always)]
    pub const fn prset2(&self) -> &Prset2 {
        &self.prset2
    }
    #[doc = "0x2c - RCU Peripheral 2 Reset Clear"]
    #[inline(always)]
    pub const fn prclr2(&self) -> &Prclr2 {
        &self.prclr2
    }
}
#[doc = "RSTSTAT (r) register accessor: RCU Reset Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rststat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rststat`]
module"]
#[doc(alias = "RSTSTAT")]
pub type Rststat = crate::Reg<rststat::RststatSpec>;
#[doc = "RCU Reset Status"]
pub mod rststat;
#[doc = "RSTSET (w) register accessor: RCU Reset Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstset`]
module"]
#[doc(alias = "RSTSET")]
pub type Rstset = crate::Reg<rstset::RstsetSpec>;
#[doc = "RCU Reset Set Register"]
pub mod rstset;
#[doc = "RSTCLR (w) register accessor: RCU Reset Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstclr`]
module"]
#[doc(alias = "RSTCLR")]
pub type Rstclr = crate::Reg<rstclr::RstclrSpec>;
#[doc = "RCU Reset Clear Register"]
pub mod rstclr;
#[doc = "PRSTAT0 (r) register accessor: RCU Peripheral 0 Reset Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prstat0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prstat0`]
module"]
#[doc(alias = "PRSTAT0")]
pub type Prstat0 = crate::Reg<prstat0::Prstat0Spec>;
#[doc = "RCU Peripheral 0 Reset Status"]
pub mod prstat0;
#[doc = "PRSET0 (w) register accessor: RCU Peripheral 0 Reset Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prset0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prset0`]
module"]
#[doc(alias = "PRSET0")]
pub type Prset0 = crate::Reg<prset0::Prset0Spec>;
#[doc = "RCU Peripheral 0 Reset Set"]
pub mod prset0;
#[doc = "PRCLR0 (w) register accessor: RCU Peripheral 0 Reset Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prclr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prclr0`]
module"]
#[doc(alias = "PRCLR0")]
pub type Prclr0 = crate::Reg<prclr0::Prclr0Spec>;
#[doc = "RCU Peripheral 0 Reset Clear"]
pub mod prclr0;
#[doc = "PRSTAT1 (r) register accessor: RCU Peripheral 1 Reset Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prstat1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prstat1`]
module"]
#[doc(alias = "PRSTAT1")]
pub type Prstat1 = crate::Reg<prstat1::Prstat1Spec>;
#[doc = "RCU Peripheral 1 Reset Status"]
pub mod prstat1;
#[doc = "PRSET1 (w) register accessor: RCU Peripheral 1 Reset Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prset1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prset1`]
module"]
#[doc(alias = "PRSET1")]
pub type Prset1 = crate::Reg<prset1::Prset1Spec>;
#[doc = "RCU Peripheral 1 Reset Set"]
pub mod prset1;
#[doc = "PRCLR1 (w) register accessor: RCU Peripheral 1 Reset Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prclr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prclr1`]
module"]
#[doc(alias = "PRCLR1")]
pub type Prclr1 = crate::Reg<prclr1::Prclr1Spec>;
#[doc = "RCU Peripheral 1 Reset Clear"]
pub mod prclr1;
#[doc = "PRSTAT2 (r) register accessor: RCU Peripheral 2 Reset Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prstat2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prstat2`]
module"]
#[doc(alias = "PRSTAT2")]
pub type Prstat2 = crate::Reg<prstat2::Prstat2Spec>;
#[doc = "RCU Peripheral 2 Reset Status"]
pub mod prstat2;
#[doc = "PRSET2 (w) register accessor: RCU Peripheral 2 Reset Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prset2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prset2`]
module"]
#[doc(alias = "PRSET2")]
pub type Prset2 = crate::Reg<prset2::Prset2Spec>;
#[doc = "RCU Peripheral 2 Reset Set"]
pub mod prset2;
#[doc = "PRCLR2 (w) register accessor: RCU Peripheral 2 Reset Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prclr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prclr2`]
module"]
#[doc(alias = "PRCLR2")]
pub type Prclr2 = crate::Reg<prclr2::Prclr2Spec>;
#[doc = "RCU Peripheral 2 Reset Clear"]
pub mod prclr2;

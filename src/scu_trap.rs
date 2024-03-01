#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    trapstat: Trapstat,
    trapraw: Trapraw,
    trapdis: Trapdis,
    trapclr: Trapclr,
    trapset: Trapset,
}
impl RegisterBlock {
    #[doc = "0x00 - Trap Status Register"]
    #[inline(always)]
    pub const fn trapstat(&self) -> &Trapstat {
        &self.trapstat
    }
    #[doc = "0x04 - Trap Raw Status Register"]
    #[inline(always)]
    pub const fn trapraw(&self) -> &Trapraw {
        &self.trapraw
    }
    #[doc = "0x08 - Trap Disable Register"]
    #[inline(always)]
    pub const fn trapdis(&self) -> &Trapdis {
        &self.trapdis
    }
    #[doc = "0x0c - Trap Clear Register"]
    #[inline(always)]
    pub const fn trapclr(&self) -> &Trapclr {
        &self.trapclr
    }
    #[doc = "0x10 - Trap Set Register"]
    #[inline(always)]
    pub const fn trapset(&self) -> &Trapset {
        &self.trapset
    }
}
#[doc = "TRAPSTAT (r) register accessor: Trap Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trapstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trapstat`]
module"]
#[doc(alias = "TRAPSTAT")]
pub type Trapstat = crate::Reg<trapstat::TrapstatSpec>;
#[doc = "Trap Status Register"]
pub mod trapstat;
#[doc = "TRAPRAW (r) register accessor: Trap Raw Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trapraw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trapraw`]
module"]
#[doc(alias = "TRAPRAW")]
pub type Trapraw = crate::Reg<trapraw::TraprawSpec>;
#[doc = "Trap Raw Status Register"]
pub mod trapraw;
#[doc = "TRAPDIS (rw) register accessor: Trap Disable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trapdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trapdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trapdis`]
module"]
#[doc(alias = "TRAPDIS")]
pub type Trapdis = crate::Reg<trapdis::TrapdisSpec>;
#[doc = "Trap Disable Register"]
pub mod trapdis;
#[doc = "TRAPCLR (w) register accessor: Trap Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trapclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trapclr`]
module"]
#[doc(alias = "TRAPCLR")]
pub type Trapclr = crate::Reg<trapclr::TrapclrSpec>;
#[doc = "Trap Clear Register"]
pub mod trapclr;
#[doc = "TRAPSET (w) register accessor: Trap Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trapset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trapset`]
module"]
#[doc(alias = "TRAPSET")]
pub type Trapset = crate::Reg<trapset::TrapsetSpec>;
#[doc = "Trap Set Register"]
pub mod trapset;

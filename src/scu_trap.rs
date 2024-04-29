#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    trapstat: TRAPSTAT,
    trapraw: TRAPRAW,
    trapdis: TRAPDIS,
    trapclr: TRAPCLR,
    trapset: TRAPSET,
}
impl RegisterBlock {
    #[doc = "0x00 - Trap Status Register"]
    #[inline(always)]
    pub const fn trapstat(&self) -> &TRAPSTAT {
        &self.trapstat
    }
    #[doc = "0x04 - Trap Raw Status Register"]
    #[inline(always)]
    pub const fn trapraw(&self) -> &TRAPRAW {
        &self.trapraw
    }
    #[doc = "0x08 - Trap Disable Register"]
    #[inline(always)]
    pub const fn trapdis(&self) -> &TRAPDIS {
        &self.trapdis
    }
    #[doc = "0x0c - Trap Clear Register"]
    #[inline(always)]
    pub const fn trapclr(&self) -> &TRAPCLR {
        &self.trapclr
    }
    #[doc = "0x10 - Trap Set Register"]
    #[inline(always)]
    pub const fn trapset(&self) -> &TRAPSET {
        &self.trapset
    }
}
#[doc = "TRAPSTAT (r) register accessor: Trap Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trapstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trapstat`]
module"]
pub type TRAPSTAT = crate::Reg<trapstat::TRAPSTAT_SPEC>;
#[doc = "Trap Status Register"]
pub mod trapstat;
#[doc = "TRAPRAW (r) register accessor: Trap Raw Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trapraw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trapraw`]
module"]
pub type TRAPRAW = crate::Reg<trapraw::TRAPRAW_SPEC>;
#[doc = "Trap Raw Status Register"]
pub mod trapraw;
#[doc = "TRAPDIS (rw) register accessor: Trap Disable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trapdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trapdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trapdis`]
module"]
pub type TRAPDIS = crate::Reg<trapdis::TRAPDIS_SPEC>;
#[doc = "Trap Disable Register"]
pub mod trapdis;
#[doc = "TRAPCLR (w) register accessor: Trap Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trapclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trapclr`]
module"]
pub type TRAPCLR = crate::Reg<trapclr::TRAPCLR_SPEC>;
#[doc = "Trap Clear Register"]
pub mod trapclr;
#[doc = "TRAPSET (w) register accessor: Trap Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trapset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trapset`]
module"]
pub type TRAPSET = crate::Reg<trapset::TRAPSET_SPEC>;
#[doc = "Trap Set Register"]
pub mod trapset;

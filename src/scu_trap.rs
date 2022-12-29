#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Trap Status Register"]
    pub trapstat: TRAPSTAT,
    #[doc = "0x04 - Trap Raw Status Register"]
    pub trapraw: TRAPRAW,
    #[doc = "0x08 - Trap Disable Register"]
    pub trapdis: TRAPDIS,
    #[doc = "0x0c - Trap Clear Register"]
    pub trapclr: TRAPCLR,
    #[doc = "0x10 - Trap Set Register"]
    pub trapset: TRAPSET,
}
#[doc = "TRAPSTAT (r) register accessor: an alias for `Reg<TRAPSTAT_SPEC>`"]
pub type TRAPSTAT = crate::Reg<trapstat::TRAPSTAT_SPEC>;
#[doc = "Trap Status Register"]
pub mod trapstat;
#[doc = "TRAPRAW (r) register accessor: an alias for `Reg<TRAPRAW_SPEC>`"]
pub type TRAPRAW = crate::Reg<trapraw::TRAPRAW_SPEC>;
#[doc = "Trap Raw Status Register"]
pub mod trapraw;
#[doc = "TRAPDIS (rw) register accessor: an alias for `Reg<TRAPDIS_SPEC>`"]
pub type TRAPDIS = crate::Reg<trapdis::TRAPDIS_SPEC>;
#[doc = "Trap Disable Register"]
pub mod trapdis;
#[doc = "TRAPCLR (w) register accessor: an alias for `Reg<TRAPCLR_SPEC>`"]
pub type TRAPCLR = crate::Reg<trapclr::TRAPCLR_SPEC>;
#[doc = "Trap Clear Register"]
pub mod trapclr;
#[doc = "TRAPSET (w) register accessor: an alias for `Reg<TRAPSET_SPEC>`"]
pub type TRAPSET = crate::Reg<trapset::TRAPSET_SPEC>;
#[doc = "Trap Set Register"]
pub mod trapset;
